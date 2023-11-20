use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

use crate::dav1d::*;
use crate::io::*;
use crate::mp4box::ItemProperty::AuxiliaryType;
use crate::mp4box::ItemProperty::ImageSpatialExtents;
use crate::mp4box::*;
use crate::stream::*;

// TODO: needed only for debug to AvifImage. Can be removed it AvifIMage does not have to be debug printable.
use derivative::Derivative;

#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct AvifImage {
    pub width: u32,
    pub height: u32,
    pub depth: u8,

    pub yuv_format: u8,
    pub full_range: bool,
    pub chroma_sample_position: u8,

    pub yuv_planes: [Option<*const u8>; 3],
    pub yuv_row_bytes: [u32; 3], // TODO: named constant
    pub image_owns_yuv_planes: bool,

    pub alpha_plane: Option<*const u8>,
    pub alpha_row_bytes: u32,
    pub image_owns_alpha_plane: bool,
    pub alpha_premultiplied: bool,

    pub icc: u8, //Option<Vec<u8>>,

    pub color_primaries: u16,
    pub transfer_characteristics: u16,
    pub matrix_coefficients: u16,
    // some more boxes. clli, transformations. pasp, clap, irot, imir.

    // exif, xmp.

    // gainmap.
    #[derivative(Debug = "ignore")]
    plane_buffers: [Vec<u8>; 4],
}

#[derive(Debug)]
pub struct AvifPlane {
    pub data: *const u8,
    pub width: u32,
    pub height: u32,
    pub row_bytes: u32,
    pub pixel_size: u32,
}

impl AvifImage {
    pub fn plane(&self, plane: usize) -> Option<AvifPlane> {
        assert!(plane < 4);
        let pixel_size = if self.depth == 8 { 1 } else { 2 };
        if plane < 3 {
            if self.yuv_planes[plane].is_none() {
                return None;
            }
            let mut plane_width = self.width;
            let mut plane_height = self.height;
            if plane > 0 {
                if self.yuv_format == 1 {
                    plane_width = (plane_width + 1) / 2;
                    plane_height = (plane_height + 1) / 2;
                } else if self.yuv_format == 2 {
                    plane_width = (plane_width + 1) / 2;
                }
            }
            let stride_index: usize = if plane == 0 { 0 } else { 1 };
            // let plane_data;
            // if self.image_owns_yuv_planes {
            //     plane_data = plane_buffer[plane].as_ptr();
            // } else {
            //     plane_data = self.yuv_planes[plane].unwrap();
            // }
            return Some(AvifPlane {
                data: self.yuv_planes[plane].unwrap(),
                width: plane_width,
                height: plane_height,
                row_bytes: self.yuv_row_bytes[plane],
                pixel_size,
            });
        }
        if self.alpha_plane.is_none() {
            return None;
        }
        return Some(AvifPlane {
            data: self.alpha_plane.unwrap(),
            width: self.width,
            height: self.height,
            row_bytes: self.alpha_row_bytes,
            pixel_size,
        });
    }

    fn allocate_planes(&mut self, category: usize) -> bool {
        // TODO : assumes 444. do other stuff.
        let pixel_size: u32 = if self.depth == 8 { 1 } else { 2 };
        let plane_size = (self.width * self.height * pixel_size) as usize;
        if category == 0 {
            for plane_index in 0usize..3 {
                self.plane_buffers[plane_index].reserve(plane_size);
                self.plane_buffers[plane_index].resize(plane_size, 0);
                self.yuv_row_bytes[plane_index] = self.width * pixel_size;
                self.yuv_planes[plane_index] = Some(self.plane_buffers[plane_index].as_ptr());
            }
            self.image_owns_yuv_planes = true;
        } else if category == 1 {
            self.plane_buffers[3].reserve(plane_size);
            self.plane_buffers[3].resize(plane_size, 255);
            self.alpha_row_bytes = self.width * pixel_size;
            self.alpha_plane = Some(self.plane_buffers[3].as_ptr());
            self.image_owns_alpha_plane = true;
        } else {
            println!("unknown category {category}. cannot allocate.");
            return false;
        }
        true
    }

    fn copy_from_tile(
        &mut self,
        tile: &AvifImage,
        tile_info: &AvifTileInfo,
        tile_index: u32,
        category: usize,
    ) -> bool {
        let row_index: usize = (tile_index / tile_info.grid.columns) as usize;
        let column_index: usize = (tile_index % tile_info.grid.columns) as usize;
        println!("copying tile {tile_index} {row_index} {column_index}");

        let plane_range = if category == 1 { 3usize..4 } else { 0usize..3 };
        // TODO: what about gainmap category?

        for plane_index in plane_range {
            println!("plane_index {plane_index}");
            let src_plane = tile.plane(plane_index);
            if src_plane.is_none() {
                continue;
            }
            let src_plane = src_plane.unwrap();
            let src_width_to_copy;
            // If this is the last tile column, clamp to left over width.
            if (column_index as u32) == tile_info.grid.columns - 1 {
                let width_so_far = src_plane.width * (column_index as u32);
                // TODO: does self.width need to be accounted for subsampling?
                src_width_to_copy = self.width - width_so_far;
            } else {
                src_width_to_copy = src_plane.width;
            }
            let src_byte_count: usize = (src_width_to_copy * src_plane.pixel_size)
                .try_into()
                .unwrap();
            let dst_row_bytes = if plane_index < 3 {
                self.yuv_row_bytes[plane_index]
            } else {
                self.alpha_row_bytes
            };

            let mut dst_base_offset: usize = 0;
            dst_base_offset += row_index * ((src_plane.height * dst_row_bytes) as usize);
            dst_base_offset += column_index * ((src_plane.width * src_plane.pixel_size) as usize);
            //println!("dst base_offset: {dst_base_offset}");

            let src_height_to_copy;
            // If this is the last tile row, clamp to left over height.
            if (row_index as u32) == tile_info.grid.rows - 1 {
                let height_so_far = src_plane.height * (row_index as u32);
                // TODO: does self.height need to be accounted for subsampling?
                src_height_to_copy = self.height - height_so_far;
            } else {
                src_height_to_copy = src_plane.height;
            }

            for y in 0..src_height_to_copy {
                let src_stride_offset: isize = (y * src_plane.row_bytes).try_into().unwrap();
                let ptr = unsafe { src_plane.data.offset(src_stride_offset) };
                let pixels = unsafe { std::slice::from_raw_parts(ptr, src_byte_count) };
                let dst_stride_offset: usize = dst_base_offset + ((y * dst_row_bytes) as usize);
                let dst_end_offset: usize = dst_stride_offset + src_byte_count;

                let mut dst_slice =
                    &mut self.plane_buffers[plane_index][dst_stride_offset..dst_end_offset];
                if y == 0 {
                    println!(
                        "src slice len: {} dst_slice_len: {}",
                        pixels.len(),
                        dst_slice.len()
                    );
                }
                dst_slice.copy_from_slice(pixels);
            }
        }
        true
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum AvifDecoderSource {
    Tracks,
    PrimaryItem,
    #[default]
    Auto,
    // TODO: Thumbnail,
}

#[derive(Debug, Default)]
pub struct AvifDecoderSettings {
    pub source: AvifDecoderSource,
    pub ignore_exif: bool,
    pub ignore_icc: bool,
}

#[derive(Debug, Default)]
pub struct AvifDecoder {
    pub settings: AvifDecoderSettings,
    image: AvifImage,
    codec: Dav1d,
    source: AvifDecoderSource,
    tile_info: [AvifTileInfo; 3],
    tiles: [Vec<AvifTile>; 3],
    alpha_present: bool,
    image_index: i32,
    pub image_count: u32,
    pub timescale: u32,
    pub duration_in_timescales: u64,
    pub duration: f64,
    pub repetition_count: i32,
    avif_items: HashMap<u32, AvifItem>,
    tracks: Vec<AvifTrack>,
    io: AvifDecoderFileIO,
}

#[derive(Debug, Default)]
struct AvifGrid {
    rows: u32,
    columns: u32,
    width: u32,
    height: u32,
}

#[derive(Debug, Default)]
struct AvifTileInfo {
    tile_count: u32,
    decoded_tile_count: u32,
    grid: AvifGrid,
}

#[derive(Debug, Default)]
struct AvifItem {
    id: u32,
    item_type: String,
    size: usize,
    width: u32,
    height: u32,
    content_type: String,
    properties: Vec<ItemProperty>,
    extents: Vec<ItemLocationExtent>,
    // TODO mergedExtents stuff.
    thumbnail_for_id: u32,
    aux_for_id: u32,
    desc_for_id: u32,
    dimg_for_id: u32,
    dimg_index: u32,
    prem_by_id: u32,
    has_unsupported_essential_property: bool,
    ipma_seen: bool,
    progressive: bool,
    idat: Vec<u8>,
}

macro_rules! find_property {
    ($self:ident, $a:ident) => {
        $self
            .properties
            .iter()
            .find(|x| matches!(x, ItemProperty::$a(_)))
    };
}

macro_rules! find_properties {
    ($self:ident, $a:ident) => {
        $self
            .properties
            .iter()
            .filter(|x| matches!(x, ItemProperty::$a(_)))
            .collect()
    };
}

impl AvifItem {
    fn data_offset(&self) -> u64 {
        self.extents[0].offset as u64
    }

    fn read_and_parse(&self, io: &mut impl AvifDecoderIO, grid: &mut AvifGrid) -> bool {
        // TODO: this function also has to extract codec type.
        if self.item_type != "grid" {
            return true;
        }
        // TODO: handle multiple extents.
        let mut io_data = match self.idat.is_empty() {
            true => match io.read(self.data_offset(), self.size) {
                Ok(data) => data,
                Err(err) => return false,
            },
            false => {
                // TODO: assumes idat offset is 0.
                self.idat.as_slice()
            }
        };
        let mut stream = IStream::create(io_data);
        // unsigned int(8) version = 0;
        let version = stream.read_u8();
        if version != 0 {
            println!("unsupported version for grid");
            return false;
        }
        // unsigned int(8) flags;
        let flags = stream.read_u8();
        // unsigned int(8) rows_minus_one;
        grid.rows = stream.read_u8() as u32;
        grid.rows += 1;
        // unsigned int(8) columns_minus_one;
        grid.columns = stream.read_u8() as u32;
        grid.columns += 1;
        if (flags & 1) == 1 {
            // unsigned int(32) output_width;
            grid.width = stream.read_u32();
            // unsigned int(32) output_height;
            grid.height = stream.read_u32();
        } else {
            // unsigned int(16) output_width;
            grid.width = stream.read_u16() as u32;
            // unsigned int(16) output_height;
            grid.height = stream.read_u16() as u32;
        }
        if grid.width == 0 || grid.height == 0 {
            println!("invalid dimensions in grid box");
            return false;
        }
        println!("grid: {:#?}", grid);
        // TODO: check for too large of a grid.
        true
    }

    fn operating_point(&self) -> u8 {
        match find_property!(self, OperatingPointSelector) {
            Some(a1op) => match a1op {
                ItemProperty::OperatingPointSelector(operating_point) => *operating_point,
                _ => 0, // not reached.
            },
            None => 0, // default operating point.
        }
    }

    fn harvest_ispe(&mut self) -> bool {
        if self.size == 0 {
            return true;
        }
        if self.has_unsupported_essential_property {
            // An essential property isn't supported by libavif. Ignore.
            return true;
        }

        let is_grid = self.item_type == "grid";
        if self.item_type != "av01" && !is_grid {
            // probably exif or some other data.
            return true;
        }
        match find_property!(self, ImageSpatialExtents) {
            Some(property) => match property {
                ItemProperty::ImageSpatialExtents(x) => {
                    self.width = x.width;
                    self.height = x.height;
                    if self.width == 0 || self.height == 0 {
                        println!("item id has invalid size.");
                        return false;
                    }
                }
                _ => return false, // not reached.
            },
            None => {
                // No ispe was found.
                if self.is_auxiliary_alpha() {
                    // TODO: provide a strict flag to bypass this check.
                    println!("alpha auxiliary image is missing mandatory ispe");
                    return false;
                } else {
                    println!("item id is missing mandatory ispe property");
                    return false;
                }
            }
        }
        true
    }

    fn av1C(&self) -> Option<&CodecConfiguration> {
        match find_property!(self, CodecConfiguration) {
            Some(property) => match property {
                ItemProperty::CodecConfiguration(av1C) => Some(&av1C),
                _ => None, // not reached.
            },
            None => None,
        }
    }

    fn a1lx(&self) -> Option<&[usize; 3]> {
        match find_property!(self, AV1LayeredImageIndexing) {
            Some(property) => match property {
                ItemProperty::AV1LayeredImageIndexing(a1lx) => Some(&a1lx),
                _ => None, // not reached.
            },
            None => None,
        }
    }

    fn lsel(&self) -> Option<u16> {
        match find_property!(self, LayerSelector) {
            Some(property) => match property {
                ItemProperty::LayerSelector(lsel) => Some(*lsel),
                _ => None, // not reached.
            },
            None => None,
        }
    }

    fn is_auxiliary_alpha(&self) -> bool {
        match find_property!(self, AuxiliaryType) {
            Some(auxC) => match auxC {
                ItemProperty::AuxiliaryType(aux_type) => {
                    aux_type == "urn:mpeg:mpegB:cicp:systems:auxiliary:alpha"
                        || aux_type == "urn:mpeg:hevc:2015:auxid:1"
                }
                _ => false, // not reached.
            },
            None => false,
        }
    }
}

fn find_nclx(properties: &Vec<ItemProperty>) -> Result<&Nclx, bool> {
    let nclx_properties: Vec<_> = properties
        .iter()
        .filter(|x| match x {
            ItemProperty::ColorInformation(colr) => match colr {
                ColorInformation::Nclx(_) => true,
                _ => false,
            },
            _ => false,
        })
        .collect();
    match nclx_properties.len() {
        0 => Err(false),
        1 => match nclx_properties[0] {
            ItemProperty::ColorInformation(colr) => match colr {
                ColorInformation::Nclx(nclx) => Ok(&nclx),
                _ => Err(false), // not reached.
            },
            _ => Err(false), // not reached.
        },
        _ => Err(true), // multiple nclx were found.
    }
}

fn find_icc(properties: &Vec<ItemProperty>) -> Result<&Icc, bool> {
    let icc_properties: Vec<_> = properties
        .iter()
        .filter(|x| match x {
            ItemProperty::ColorInformation(colr) => match colr {
                ColorInformation::Icc(_) => true,
                _ => false,
            },
            _ => false,
        })
        .collect();
    match icc_properties.len() {
        0 => Err(false),
        1 => match icc_properties[0] {
            ItemProperty::ColorInformation(colr) => match colr {
                ColorInformation::Icc(icc) => Ok(&icc),
                _ => Err(false), // not reached.
            },
            _ => Err(false), // not reached.
        },
        _ => Err(true), // multiple icc were found.
    }
}

fn find_av1C(properties: &Vec<ItemProperty>) -> Option<&CodecConfiguration> {
    match properties
        .iter()
        .find(|x| matches!(x, ItemProperty::CodecConfiguration(_)))
    {
        Some(property) => match property {
            ItemProperty::CodecConfiguration(av1C) => Some(&av1C),
            _ => None, // not reached.
        },
        None => None,
    }
}

fn read_file(filename: &String) -> Vec<u8> {
    let mut file = File::open(filename).expect("file not found");
    let mut data: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut data);
    data
}

// This design is not final. It's possible to do this in the same loop where boxes are parsed. But it
// seems a little cleaner to do this after the fact.
fn construct_avif_items(meta: &MetaBox) -> Result<HashMap<u32, AvifItem>, &str> {
    let mut avif_items: HashMap<u32, AvifItem> = HashMap::new();
    for item in &meta.iinf {
        let mut avif_item: AvifItem = Default::default();
        avif_item.id = item.item_id;
        avif_item.item_type = item.item_type.clone();
        avif_item.content_type = item.content_type.clone();
        avif_items.insert(avif_item.id, avif_item);
    }
    for item in &meta.iloc.items {
        // TODO: Make sure item id exists before unwrapping.
        let avif_item = avif_items.get_mut(&item.item_id).unwrap();
        if !avif_item.extents.is_empty() {
            return Err("item already has extents.");
        }
        if item.construction_method == 1 {
            avif_item.idat = meta.idat.clone();
        }
        // TODO: handle overflows in the addition below.
        for extent in &item.extents {
            avif_item.extents.push(ItemLocationExtent {
                offset: item.base_offset + extent.offset,
                length: extent.length,
            });
            avif_item.size += extent.length as usize;
        }
    }
    for association in &meta.iprp.associations {
        // TODO: Make sure item id exists before unwrapping.
        let avif_item = avif_items.get_mut(&association.item_id).unwrap();
        if avif_item.ipma_seen {
            // TODO: ipma_seen can be a local hashmap or set here instea of being in the
            // struct as it is only used for this validation.
            return Err("item has duplictate ipma.");
        }
        avif_item.ipma_seen = true;
        for (property_index_ref, essential_ref) in &association.associations {
            let property_index: usize = *property_index_ref as usize;
            let essential = *essential_ref;
            if property_index == 0 {
                // Not associated with any item.
                continue;
            }
            if property_index > meta.iprp.properties.len() {
                println!(
                    "property index: {} len: {}",
                    property_index,
                    meta.iprp.properties.len()
                );
                return Err("invalid property_index in ipma.");
            }
            // property_index is 1-indexed.
            let property = meta.iprp.properties[property_index - 1].clone();
            // TODO: Add more boxes here once they are supported.
            let is_supported_property = match property {
                ItemProperty::ImageSpatialExtents(_)
                | ItemProperty::ColorInformation(_)
                | ItemProperty::CodecConfiguration(_)
                | ItemProperty::PixelInformation(_)
                | ItemProperty::PixelAspectRatio(_)
                | ItemProperty::AuxiliaryType(_)
                | ItemProperty::ClearAperture(_)
                | ItemProperty::ImageRotation(_)
                | ItemProperty::ImageMirror(_)
                | ItemProperty::OperatingPointSelector(_)
                | ItemProperty::LayerSelector(_)
                | ItemProperty::AV1LayeredImageIndexing(_)
                | ItemProperty::ContentLightLevelInformation(_) => true,
                _ => false,
            };
            if is_supported_property {
                if essential {
                    // a1lx is not allowed to be marked as essential.
                    // TODO: enforce that.
                } else {
                    // a1op and lsel must be marked as essential.
                    // TODO: enforce that.
                }
                avif_item.properties.push(property);
            } else {
                if essential {
                    avif_item.has_unsupported_essential_property = true;
                }
            }
        }
    }
    for (reference_index, reference) in meta.iref.iter().enumerate() {
        let item = avif_items.get_mut(&reference.from_item_id);
        if item.is_none() {
            return Err("invalid from_item_id in iref");
        }
        let item = item.unwrap();
        match reference.reference_type.as_str() {
            "thmb" => item.thumbnail_for_id = reference.to_item_id,
            "auxl" => item.aux_for_id = reference.to_item_id,
            "cdsc" => item.desc_for_id = reference.to_item_id,
            "prem" => item.prem_by_id = reference.to_item_id,
            "dimg" => {
                // derived images refer in the opposite direction.
                let dimg_item = avif_items.get_mut(&reference.to_item_id);
                if dimg_item.is_none() {
                    return Err("invalid to_item_id in iref");
                }
                let dimg_item = dimg_item.unwrap();
                dimg_item.dimg_for_id = reference.from_item_id;
                dimg_item.dimg_index = reference_index as u32;
            }
            _ => {
                // unknown reference type, ignore.
            }
        }
    }
    Ok(avif_items)
}

fn should_skip_decoder_item(item: &AvifItem) -> bool {
    item.size == 0
        || item.has_unsupported_essential_property
        || (item.item_type != "av01" && item.item_type != "grid")
        || item.thumbnail_for_id != 0
}

fn find_color_item(avif_items: &HashMap<u32, AvifItem>, primary_item_id: u32) -> u32 {
    if primary_item_id == 0 {
        return 0;
    }
    // TODO: perhaps this can be an idiomatic oneliner ?
    for (_, item) in avif_items {
        if should_skip_decoder_item(item) {
            continue;
        }
        if item.id == primary_item_id {
            return item.id;
        }
    }
    0
}

fn find_alpha_item(avif_items: &HashMap<u32, AvifItem>, color_item: &AvifItem) -> u32 {
    for (_, item) in avif_items {
        if should_skip_decoder_item(item) {
            continue;
        }
        if item.aux_for_id != color_item.id {
            continue;
        }
        if !item.is_auxiliary_alpha() {
            continue;
        }
        return item.id;
    }
    if color_item.item_type != "grid" {
        return 0;
    }
    // TODO: If color item is a grid, check if there is an alpha channel which is represented as an auxl item to each color tile item.
    0
}

#[derive(Debug, Default)]
struct AvifDecodeSample {
    // owns_data
    // partial_data
    item_id: u32,
    offset: u64,
    size: usize,
    spatial_id: u8,
    sync: bool,
    // TODO: these two can be some enum?
    data_buffer: Option<Vec<u8>>,
}

impl AvifDecodeSample {
    pub fn data<'a>(&'a self, io: &'a mut impl AvifDecoderIO) -> Result<&[u8], i32> {
        match &self.data_buffer {
            Some(data_buffer) => Ok(&data_buffer),
            None => io.read(self.offset, self.size),
        }
    }
}

#[derive(Debug, Default)]
struct AvifDecodeInput {
    samples: Vec<AvifDecodeSample>,
    all_layers: bool,
    category: u8,
}

#[derive(Debug, Default)]
struct AvifTile {
    width: u32,
    height: u32,
    operating_point: u8,
    image: AvifImage,
    input: AvifDecodeInput,
    codec: Dav1d,
}

fn create_tile(item: &AvifItem) -> Option<AvifTile> {
    let mut tile = AvifTile::default();
    tile.width = item.width;
    tile.height = item.height;
    tile.operating_point = item.operating_point();
    tile.image = AvifImage::default();
    // TODO: do all the layer stuff (a1op and lsel) in avifCodecDecodeInputFillFromDecoderItem.
    let mut layer_sizes: [usize; 4] = [0; 4];
    let mut layer_count = 0;
    let a1lx = item.a1lx();
    if a1lx.is_some() {
        let a1lx = a1lx.unwrap();
        println!("item size: {} a1lx: {:#?}", item.size, a1lx);
        let mut remaining_size: usize = item.size;
        for i in 0usize..3 {
            layer_count += 1;
            if a1lx[i] > 0 {
                // >= instead of > because there must be room for the last layer
                if a1lx[i] >= remaining_size {
                    println!("a1lx layer index [{i}] does not fit in item size");
                    return None;
                }
                layer_sizes[i] = a1lx[i];
                remaining_size -= a1lx[i];
            } else {
                layer_sizes[i] = remaining_size;
                remaining_size = 0;
                break;
            }
        }
        if remaining_size > 0 {
            assert!(layer_count == 3);
            layer_count += 1;
            layer_sizes[3] = remaining_size;
        }
        println!("layer count: {layer_count} layer_sizes: {:#?}", layer_sizes);
    }
    let lsel = item.lsel();
    // TODO: account for progressive (avifCodecDecodeInputFillFromDecoderItem).
    if lsel.is_some() && lsel.unwrap() != 0xFFFF {
        // Layer selection. This requires that the underlying AV1 codec decodes all layers,
        // and then only returns the requested layer as a single frame. To the user of libavif,
        // this appears to be a single frame.
        tile.input.all_layers = true;
        let mut sample_size: usize = 0;
        let layer_id = lsel.unwrap();
        if layer_count > 0 {
            // TODO: test this with a case?
            println!("im here");
            return None;
            // Optimization: If we're selecting a layer that doesn't require
            // the entire image's payload (hinted via the a1lx box).
            if layer_id >= layer_count {
                println!("lsel layer index not found in a1lx.");
                return None;
            }
            let layer_id_plus_1: usize = (layer_id + 1) as usize;
            for i in 0usize..layer_id_plus_1 {
                sample_size += layer_sizes[i];
            }
        } else {
            // This layer payload subsection is not known. Use the whole payload.
            sample_size = item.size;
        }
        let sample = AvifDecodeSample {
            item_id: item.id,
            offset: 0,
            size: sample_size,
            spatial_id: lsel.unwrap() as u8,
            sync: true,
            data_buffer: None,
        };
        tile.input.samples.push(sample);
    } else if (false) {
        // TODO: case for progressive and allow progressive.
    } else {
        // Typical case: Use the entire item's payload for a single frame output
        let sample = AvifDecodeSample {
            item_id: item.id,
            offset: 0,
            size: item.size,
            // Legal spatial_id values are [0,1,2,3], so this serves as a sentinel
            // value for "do not filter by spatial_id"
            spatial_id: 0xff,
            sync: true,
            data_buffer: None,
        };
        tile.input.samples.push(sample);
    }
    Some(tile)
}

fn create_tile_from_track(track: &AvifTrack) -> Option<AvifTile> {
    let mut tile = AvifTile::default();
    tile.width = track.width;
    tile.height = track.height;
    tile.operating_point = 0; // No way to set operating point via tracks

    // TODO: implement the imagecount check in avifCodecDecodeInputFillFromSampleTable.

    let mut sample_size_index = 0;
    let sample_table = &track.sample_table.as_ref().unwrap();
    for (chunk_index, chunk_offset) in sample_table.chunk_offsets.iter().enumerate() {
        // Figure out how many samples are in this chunk.
        let sample_count = sample_table.get_sample_count_of_chunk(chunk_index);
        if sample_count == 0 {
            println!("chunk with 0 samples found");
            return None;
        }

        let mut sample_offset = *chunk_offset;
        for sample_index in 0..sample_count {
            let mut sample_size = sample_table.all_samples_size;
            if sample_size == 0 {
                if sample_size_index >= sample_table.sample_sizes.len() {
                    println!("not enough sampel sizes in the table");
                    return None;
                }
                sample_size = sample_table.sample_sizes[sample_size_index];
            }
            let sample = AvifDecodeSample {
                item_id: 0,
                offset: sample_offset,
                size: sample_size as usize,
                // Legal spatial_id values are [0,1,2,3], so this serves as a sentinel
                // value for "do not filter by spatial_id"
                spatial_id: 0xff,
                // Assume first sample is always sync (in case stss box was missing).
                sync: tile.input.samples.is_empty(),
                data_buffer: None,
            };
            tile.input.samples.push(sample);
            // TODO: verify if sample size math can be done here.
            sample_offset += sample_size as u64;
            sample_size_index += 1;
        }
    }
    for sync_sample_number in &sample_table.sync_samples {
        let index: usize = (*sync_sample_number - 1) as usize; // sample_table.sync_samples is 1-based.
        if index < tile.input.samples.len() {
            tile.input.samples[index].sync = true;
        }
    }
    Some(tile)
}

fn generate_tiles(
    avif_items: &mut HashMap<u32, AvifItem>,
    iinf: &Vec<ItemInfo>,
    item_id: u32,
    info: &AvifTileInfo,
    category: usize,
) -> Option<Vec<AvifTile>> {
    let mut tiles: Vec<AvifTile> = Vec::new();
    if info.grid.rows > 0 && info.grid.columns > 0 {
        println!("grid###: {:#?}", info.grid);
        let mut grid_item_ids: Vec<u32> = Vec::new();
        let mut first_av1C: CodecConfiguration = Default::default();
        // Collect all the dimg items.
        // Cannot directly iterate through avif_items here directly because HashMap is not ordered.
        for item_info in iinf {
            let dimg_item = avif_items.get(&item_info.item_id);
            if dimg_item.is_none() {
                println!("invalid item");
                return None;
            }
            let dimg_item = dimg_item.unwrap();
            if dimg_item.dimg_for_id != item_id {
                continue;
            }
            if dimg_item.item_type != "av01" {
                println!("invalid item_type in dimg grid");
                return None;
            }
            if dimg_item.has_unsupported_essential_property {
                println!(
                    "Grid image contains tile with an unsupported property marked as essential"
                );
                return None;
            }
            let tile = create_tile(dimg_item);
            if tile.is_none() {
                return None;
            }
            let mut tile = tile.unwrap();
            tile.input.category = category as u8;
            tiles.push(tile);

            if tiles.len() == 1 {
                // Adopt the configuration property of the first tile.
                let dimg_av1C = dimg_item.av1C();
                if dimg_av1C.is_none() {
                    println!("dimg is missing dimg_av1C");
                    return None;
                }
                first_av1C = dimg_av1C.unwrap().clone();
            }
            grid_item_ids.push(item_info.item_id);
        }
        println!("grid item itds: {:#?}", grid_item_ids);
        // TODO: check if there are enough grids.
        avif_items
            .get_mut(&item_id)
            .unwrap()
            .properties
            .push(ItemProperty::CodecConfiguration(first_av1C));
        println!("grid item ids: {:#?}", grid_item_ids);
        for item in iinf.iter() {
            println!("item id: {}", item.item_id);
        }
    } else {
        let item = avif_items.get(&item_id).unwrap();
        if item.size == 0 {
            return None;
        }
        let tile = create_tile(item);
        if tile.is_none() {
            return None;
        }
        let mut tile = tile.unwrap();
        tile.input.category = category as u8;
        tiles.push(tile);
    }
    Some(tiles)
}

fn steal_planes(dst: &mut AvifImage, src: &mut AvifImage, category: usize) {
    match category {
        0 => {
            dst.yuv_planes[0] = src.yuv_planes[0];
            dst.yuv_planes[1] = src.yuv_planes[1];
            dst.yuv_planes[2] = src.yuv_planes[2];
            dst.yuv_row_bytes[0] = src.yuv_row_bytes[0];
            dst.yuv_row_bytes[1] = src.yuv_row_bytes[1];
            dst.yuv_row_bytes[2] = src.yuv_row_bytes[2];
            src.yuv_planes[0] = None;
            src.yuv_planes[1] = None;
            src.yuv_planes[2] = None;
            src.yuv_row_bytes[0] = 0;
            src.yuv_row_bytes[1] = 0;
            src.yuv_row_bytes[2] = 0;
        }
        1 => {
            dst.alpha_plane = src.alpha_plane;
            dst.alpha_row_bytes = src.alpha_row_bytes;
            src.alpha_plane = None;
            src.alpha_row_bytes = 0;
        }
        _ => {
            // do nothing.
        }
    }
}

impl AvifDecoder {
    pub fn set_file(&mut self, filename: &String) -> bool {
        let io = AvifDecoderFileIO::create(filename);
        if io.is_none() {
            return false;
        }
        self.io = io.unwrap();
        true
    }

    pub fn parse(&mut self) -> Option<&AvifImage> {
        let avif_boxes = MP4Box::parse(&mut self.io);
        self.tracks = avif_boxes.moov.tracks;
        self.avif_items = match construct_avif_items(&avif_boxes.meta) {
            Ok(items) => items,
            Err(err) => {
                println!("failed to construct_avif_items: {err}");
                return None;
            }
        };
        for (id, item) in &mut self.avif_items {
            if !item.harvest_ispe() {
                println!("failed to harvest ispe");
                return None;
            }
        }
        println!("{:#?}", self.avif_items);

        // Build the decoder input.
        self.source = self.settings.source;
        match self.settings.source {
            AvifDecoderSource::Auto => {
                // Decide the source based on the major brand.
                if avif_boxes.ftyp.major_brand == "avis" {
                    self.source = AvifDecoderSource::Tracks;
                } else if avif_boxes.ftyp.major_brand == "avif" {
                    self.source = AvifDecoderSource::PrimaryItem;
                } else {
                    // TODO: add a else if for if track count > 0, then use tracks.
                    self.source = AvifDecoderSource::PrimaryItem;
                }
            }
            _ => {}
        }

        let color_properties: &Vec<ItemProperty>;
        match self.source {
            AvifDecoderSource::Tracks => {
                let mut color_track_index: Option<usize> = None;
                // Find primary color track.
                // TODO: move this to a function.
                for (track_index, track) in self.tracks.iter().enumerate() {
                    if track.sample_table.is_none() {
                        continue;
                    }
                    if track.id == 0 {
                        // trak box might be missing a tkhd box inside, skip it.
                        continue;
                    }
                    if track
                        .sample_table
                        .as_ref()
                        .unwrap()
                        .chunk_offsets
                        .is_empty()
                    {
                        continue;
                    }
                    if !track.sample_table.as_ref().unwrap().has_av1_sample() {
                        continue;
                    }
                    if track.aux_for_id != 0 {
                        continue;
                    }
                    // Found the color track.
                    color_track_index = Some(track_index);
                    break;
                }
                if color_track_index.is_none() {
                    println!("color track not found");
                    return None;
                }
                let color_track = &self.tracks[color_track_index.unwrap()];
                let color_properties_op =
                    color_track.sample_table.as_ref().unwrap().get_properties();
                if color_properties_op.is_none() {
                    println!("color properties not found");
                    return None;
                }
                color_properties = color_properties_op.unwrap();

                // TODO: exif/xmp from meta.

                let mut alpha_track_index: Option<usize> = None;
                for (track_index, track) in self.tracks.iter().enumerate() {
                    if track.sample_table.is_none() {
                        continue;
                    }
                    if track.id == 0 {
                        continue;
                    }
                    if track
                        .sample_table
                        .as_ref()
                        .unwrap()
                        .chunk_offsets
                        .is_empty()
                    {
                        continue;
                    }
                    if !track.sample_table.as_ref().unwrap().has_av1_sample() {
                        continue;
                    }
                    if track.aux_for_id == color_track.id {
                        // Found the alpha track.
                        alpha_track_index = Some(track_index);
                        break;
                    }
                }
                println!("alpha_track_index: {:#?}", alpha_track_index);

                let color_tile = create_tile_from_track(&color_track);
                if color_tile.is_none() {
                    println!("failed to create color tile");
                    return None;
                }
                println!("color_tile: {:#?}", color_tile);
                self.tile_info[0].tile_count = 1;
                self.tiles[0].push(color_tile.unwrap());

                if alpha_track_index.is_some() {
                    let alpha_track = &self.tracks[alpha_track_index.unwrap()];
                    let alpha_tile = create_tile_from_track(alpha_track);
                    if alpha_tile.is_none() {
                        println!("failed to create color tile");
                        return None;
                    }
                    println!("alpha_tile: {:#?}", alpha_tile);
                    self.tile_info[1].tile_count = 1;
                    self.tiles[1].push(alpha_tile.unwrap());
                    self.alpha_present = true;
                    self.image.alpha_premultiplied = color_track.prem_by_id == alpha_track.id;
                }

                self.image_index = -1;
                self.image_count = self.tiles[0][0].input.samples.len() as u32;
                self.timescale = color_track.media_timescale;
                self.duration_in_timescales = color_track.media_duration;
                if self.timescale != 0 {
                    self.duration = (self.duration_in_timescales as f64) / (self.timescale as f64);
                } else {
                    self.duration = 0.0;
                }
                self.repetition_count = color_track.repetition_count;
                // TODO: self.image timing.

                println!("image_count: {}", self.image_count);
                println!("timescale: {}", self.timescale);
                println!("duration_in_timescales: {}", self.duration_in_timescales);

                self.image.width = color_track.width;
                self.image.height = color_track.height;
            }
            AvifDecoderSource::PrimaryItem => {
                // 0 color, 1 alpha, 2 gainmap
                let mut item_ids: [u32; 3] = [0; 3];
                // Mandatory color item.
                item_ids[0] = find_color_item(&self.avif_items, avif_boxes.meta.primary_item_id);
                if item_ids[0] == 0 {
                    println!("primary color item not found.");
                    return None;
                }
                if !self
                    .avif_items
                    .get_mut(&item_ids[0])
                    .unwrap()
                    .read_and_parse(&mut self.io, &mut self.tile_info[0].grid)
                {
                    println!("failed to read_and_parse color item");
                    return None;
                }

                // Optional alpha auxiliary item
                item_ids[1] =
                    find_alpha_item(&self.avif_items, self.avif_items.get(&item_ids[0]).unwrap());
                if item_ids[1] != 0
                    && !self
                        .avif_items
                        .get_mut(&item_ids[1])
                        .unwrap()
                        .read_and_parse(&mut self.io, &mut self.tile_info[1].grid)
                {
                    println!("failed to read_and_parse alpha item");
                    return None;
                }

                println!("item ids: {:#?}", item_ids);

                // TODO: gainmap item.

                // TODO: find exif or xmp metadata.

                self.image_index = -1;
                self.image_count = 1;
                self.timescale = 1;
                self.duration_in_timescales = 1;
                // TODO: duration, imagetiming.

                for (index, item_id) in item_ids.iter().enumerate() {
                    if *item_id == 0 {
                        continue;
                    }
                    {
                        let item = self.avif_items.get(&item_id).unwrap();
                        if index == 1 && item.width == 0 && item.height == 0 {
                            // NON-STANDARD: Alpha subimage does not have an ispe
                            // property; adopt width/height from color item.
                            // TODO: need to assert for strict flag.
                            // item.width = items[0].unwrap().width;
                            // item.height = items[0].unwrap().height;
                            // TODO: make this work. some mut problem.
                        }
                    }
                    let tiles = generate_tiles(
                        &mut self.avif_items,
                        &avif_boxes.meta.iinf,
                        *item_id,
                        &self.tile_info[index],
                        index,
                    );
                    if tiles.is_none() {
                        println!("Failed to generate_tiles");
                        return None;
                    }
                    self.tiles[index] = tiles.unwrap();
                    // TODO: validate item properties.
                }

                let color_item = self.avif_items.get(&item_ids[0]).unwrap();
                self.image.width = color_item.width;
                self.image.height = color_item.height;
                self.alpha_present = item_ids[1] != 0;
                // alphapremultiplied.

                // This borrow has to be in the end of this branch.
                color_properties = &self.avif_items.get(&item_ids[0]).unwrap().properties;
            }
            _ => return None, // not reached.
        }

        // Check validity of samples.
        for tiles in &self.tiles {
            for tile in tiles {
                for sample in &tile.input.samples {
                    if sample.size == 0 {
                        println!("sample has invalid size.");
                        return None;
                    }
                    // TODO: iostats?
                }
            }
        }

        // Find and adopt all colr boxes "at most one for a given value of
        // colour type" (HEIF 6.5.5.1, from Amendment 3) Accept one of each
        // type, and bail out if more than one of a given type is provided.
        //match color_item.nclx() {
        match find_nclx(color_properties) {
            Ok(nclx) => {
                self.image.color_primaries = nclx.color_primaries;
                self.image.transfer_characteristics = nclx.transfer_characteristics;
                self.image.matrix_coefficients = nclx.matrix_coefficients;
                self.image.full_range = nclx.full_range;
            }
            Err(multiple_nclx_found) => {
                if multiple_nclx_found {
                    println!("multiple nclx were found");
                    return None;
                }
            }
        }
        match find_icc(color_properties) {
            Ok(icc) => {
                // TODO: attach icc to self.image.
            }
            Err(multiple_icc_found) => {
                if multiple_icc_found {
                    println!("multiple icc were found");
                    return None;
                }
            }
        }

        // TODO: clli, pasp, clap, irot, imir

        // TODO: if cicp was not found, harvest it from the seq hdr.

        // TODO: copy info from av1c. avifReadCodecConfigProperty.
        let av1C = find_av1C(color_properties);
        if av1C.is_none() {
            println!("missing av1C");
            return None;
        }
        let av1C = av1C.unwrap();
        self.image.depth = av1C.depth();
        if av1C.monochrome {
            self.image.yuv_format = 0;
        } else {
            if av1C.chroma_subsampling_x == 1 && av1C.chroma_subsampling_y == 1 {
                self.image.yuv_format = 1;
            } else if (av1C.chroma_subsampling_x == 1) {
                self.image.yuv_format = 2;
            } else {
                self.image.yuv_format = 3;
            }
        }
        self.image.chroma_sample_position = av1C.chroma_sample_position;

        Some(&self.image)
    }

    fn create_codecs(&mut self) -> bool {
        // TODO: share codecs for grid, etc.
        for tiles in &mut self.tiles {
            for tile in tiles {
                tile.codec
                    .initialize(tile.operating_point, tile.input.all_layers);
            }
        }
        true
    }

    fn prepare_samples(&mut self, image_index: usize) -> bool {
        // TODO: this function can probably be moved into AvifDecodeSample.data().
        for tiles in &mut self.tiles {
            for tile in tiles {
                if tile.input.samples.len() <= image_index {
                    println!("sample for index {image_index} not found.");
                    return false;
                }
                let sample = &mut tile.input.samples[image_index];
                if sample.item_id != 0 {
                    // Data comes from an item.
                    let item = self.avif_items.get(&sample.item_id);
                    if item.is_none() {
                        return false;
                    }
                    let item = item.unwrap();
                    if item.extents.len() > 1 {
                        println!("item has multiple extents");
                        if sample.data_buffer.is_none() {
                            let mut data: Vec<u8> = Vec::new();
                            data.reserve(item.size);
                            for extent in &item.extents {
                                // TODO: extent.length usize cast safety?
                                let extent_payload =
                                    match self.io.read(extent.offset, extent.length as usize) {
                                        Ok(payload) => payload,
                                        Err(e) => return false,
                                    };
                                data.extend_from_slice(extent_payload);
                            }
                            println!("merged size: {}", data.len());
                            sample.data_buffer = Some(data);
                        }
                    } else {
                        sample.offset = item.data_offset();
                    }
                } else {
                    // TODO: handle tracks.
                }
            }
        }
        true
    }

    fn decode_tiles(&mut self, image_index: usize) -> bool {
        //for (category, tiles) in self.tiles.iter_mut().enumerate() {
        for category in 0usize..3 {
            let grid = &self.tile_info[category].grid;
            let is_grid = grid.rows > 0 && grid.columns > 0;
            if is_grid {
                // allocate grid planes.
                if !self.image.allocate_planes(category) {
                    println!("failed to allocate image for grid image");
                    return false;
                }
            }
            for (tile_index, tile) in self.tiles[category].iter_mut().enumerate() {
                let sample = &tile.input.samples[image_index];
                let sample_payload = match sample.data(&mut self.io) {
                    Ok(payload) => payload,
                    Err(e) => return false,
                };
                if !tile.codec.get_next_image(
                    sample_payload,
                    sample.spatial_id,
                    &mut tile.image,
                    category,
                ) {
                    return false;
                }
                // TODO: convert alpha from limited range to full range.
                // TODO: scale tile to match output dimension.

                if is_grid {
                    println!("GRID!!");
                    // TODO: make sure all tiles decoded properties match.
                    // Need to figure out a way to do it with proper borrows.
                    self.image.copy_from_tile(
                        &tile.image,
                        &self.tile_info[category],
                        tile_index as u32,
                        category,
                    );
                } else {
                    // Non grid path, steal planes from the only tile.

                    if category == 0 {
                        self.image.width = tile.image.width;
                        self.image.height = tile.image.height;
                        self.image.depth = tile.image.depth;
                        self.image.yuv_format = tile.image.yuv_format;
                    } else if category == 1 {
                        // check width height mismatch.
                    }

                    steal_planes(&mut self.image, &mut tile.image, category);
                }
            }
        }
        true
    }

    pub fn next_image(&mut self) -> Option<&AvifImage> {
        if self.tiles[0].is_empty() && self.tiles[1].is_empty() && self.tiles[2].is_empty() {
            // Nothing has been parsed yet.
            return None;
        }

        //println!("tiles: {:#?}", self.tiles);

        let next_image_index = self.image_index + 1;
        if !self.create_codecs() {
            return None;
        }
        if !self.prepare_samples(next_image_index as usize) {
            return None;
        }
        if !self.decode_tiles(next_image_index as usize) {
            return None;
        }

        self.image_index = next_image_index;
        // TODO provide timing info for tracks.
        Some(&self.image)
    }
}
