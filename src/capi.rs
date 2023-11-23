#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::os::raw::c_char;
use std::os::raw::c_int;

use std::ffi::CStr;

use std::slice;

use libc::size_t;

use crate::decoder::*;
use crate::AvifError;
use crate::AvifResult;
use crate::AvifStrictness;
use crate::AvifStrictnessFlag;
use crate::ChromaSamplePosition;
use crate::PixelFormat;

#[repr(C)]
pub struct avifROData {
    pub data: *const u8,
    pub size: size_t,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum avifResult {
    Ok,
    UnknownError,
    InvalidFtyp,
    NoContent,
    NoYuvFormatSelected,
    ReformatFailed,
    UnsupportedDepth,
    EncodeColorFailed,
    EncodeAlphaFailed,
    BmffParseFailed,
    MissingImageItem,
    DecodeColorFailed,
    DecodeAlphaFailed,
    ColorAlphaSizeMismatch,
    IspeSizeMismatch,
    NoCodecAvailable,
    NoImagesRemaining,
    InvalidExifPayload,
    InvalidImageGrid,
    InvalidCodecSpecificOption,
    TruncatedData,
    IoNotSet,
    IoError,
    WaitingOnIo,
    InvalidArgument,
    NotImplemented,
    OutOfMemory,
    CannotChangeSetting,
    IncompatibleImage,
    EncodeGainMapFailed,
    DecodeGainMapFailed,
    InvalidToneMappedImage,
}

impl From<&AvifError> for avifResult {
    fn from(err: &AvifError) -> Self {
        match err {
            AvifError::Ok => avifResult::Ok,
            AvifError::UnknownError => avifResult::UnknownError,
            AvifError::InvalidFtyp => avifResult::InvalidFtyp,
            AvifError::NoContent => avifResult::NoContent,
            AvifError::NoYuvFormatSelected => avifResult::NoYuvFormatSelected,
            AvifError::ReformatFailed => avifResult::ReformatFailed,
            AvifError::UnsupportedDepth => avifResult::UnsupportedDepth,
            AvifError::EncodeColorFailed => avifResult::EncodeColorFailed,
            AvifError::EncodeAlphaFailed => avifResult::EncodeAlphaFailed,
            AvifError::BmffParseFailed => avifResult::BmffParseFailed,
            AvifError::MissingImageItem => avifResult::MissingImageItem,
            AvifError::DecodeColorFailed => avifResult::DecodeColorFailed,
            AvifError::DecodeAlphaFailed => avifResult::DecodeAlphaFailed,
            AvifError::ColorAlphaSizeMismatch => avifResult::ColorAlphaSizeMismatch,
            AvifError::IspeSizeMismatch => avifResult::IspeSizeMismatch,
            AvifError::NoCodecAvailable => avifResult::NoCodecAvailable,
            AvifError::NoImagesRemaining => avifResult::NoImagesRemaining,
            AvifError::InvalidExifPayload => avifResult::InvalidExifPayload,
            AvifError::InvalidImageGrid => avifResult::InvalidImageGrid,
            AvifError::InvalidCodecSpecificOption => avifResult::InvalidCodecSpecificOption,
            AvifError::TruncatedData => avifResult::TruncatedData,
            AvifError::IoNotSet => avifResult::IoNotSet,
            AvifError::IoError => avifResult::IoError,
            AvifError::WaitingOnIo => avifResult::WaitingOnIo,
            AvifError::InvalidArgument => avifResult::InvalidArgument,
            AvifError::NotImplemented => avifResult::NotImplemented,
            AvifError::OutOfMemory => avifResult::OutOfMemory,
            AvifError::CannotChangeSetting => avifResult::CannotChangeSetting,
            AvifError::IncompatibleImage => avifResult::IncompatibleImage,
            AvifError::EncodeGainMapFailed => avifResult::EncodeGainMapFailed,
            AvifError::DecodeGainMapFailed => avifResult::DecodeGainMapFailed,
            AvifError::InvalidToneMappedImage => avifResult::InvalidToneMappedImage,
        }
    }
}

pub type avifBool = c_int;
pub const AVIF_TRUE: c_int = 1;
pub const AVIF_FALSE: c_int = 0;

#[repr(C)]
#[derive(Debug)]
pub enum avifPixelFormat {
    None,
    Yuv444,
    Yuv422,
    Yuv420,
    Yuv400,
    Count,
}

impl From<PixelFormat> for avifPixelFormat {
    fn from(format: PixelFormat) -> Self {
        match format {
            PixelFormat::Yuv444 => Self::Yuv444,
            PixelFormat::Yuv422 => Self::Yuv422,
            PixelFormat::Yuv420 => Self::Yuv420,
            PixelFormat::Monochrome => Self::Yuv400,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
enum avifRange {
    LIMITED = 0,
    FULL = 1,
}

impl From<bool> for avifRange {
    fn from(full_range: bool) -> Self {
        match full_range {
            true => Self::FULL,
            false => Self::LIMITED,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
enum avifChromaSamplePosition {
    Unknown = 0,
    Vertical = 1,
    Colocated = 2,
}

impl From<ChromaSamplePosition> for avifChromaSamplePosition {
    fn from(chroma_sample_position: ChromaSamplePosition) -> Self {
        match chroma_sample_position {
            ChromaSamplePosition::Unknown => avifChromaSamplePosition::Unknown,
            ChromaSamplePosition::Vertical => avifChromaSamplePosition::Vertical,
            ChromaSamplePosition::Colocated => avifChromaSamplePosition::Colocated,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct avifImage {
    width: u32,
    height: u32,
    depth: u32,

    yuvFormat: avifPixelFormat,
    yuvRange: avifRange,
    yuvChromaSamplePosition: avifChromaSamplePosition,
    yuvPlanes: [*mut u8; 3],
    yuvRowBytes: [u32; 3],
    imageOwnsYUVPlanes: avifBool,

    alphaPlane: *mut u8,
    alphaRowBytes: u32,
    imageOwnsAlphaPlane: avifBool,
    alphaPremultiplied: avifBool,
    // avifRWData icc;
    // avifColorPrimaries colorPrimaries;
    // avifTransferCharacteristics transferCharacteristics;
    // avifMatrixCoefficients matrixCoefficients;
    // avifContentLightLevelInformationBox clli;
    // avifTransformFlags transformFlags;
    // avifPixelAspectRatioBox pasp;
    // avifCleanApertureBox clap;
    // avifImageRotation irot;
    // avifImageMirror imir;
    // avifRWData exif;
    // avifRWData xmp;
    // avifGainMap gainMap;
}

impl Default for avifImage {
    fn default() -> Self {
        avifImage {
            width: 0,
            height: 0,
            depth: 0,
            yuvFormat: avifPixelFormat::None,
            yuvRange: avifRange::FULL,
            yuvChromaSamplePosition: avifChromaSamplePosition::Unknown,
            yuvPlanes: [std::ptr::null_mut(); 3],
            yuvRowBytes: [0; 3],
            imageOwnsYUVPlanes: AVIF_FALSE,
            alphaPlane: std::ptr::null_mut(),
            alphaRowBytes: 0,
            imageOwnsAlphaPlane: AVIF_FALSE,
            alphaPremultiplied: AVIF_FALSE,
        }
    }
}

impl From<&AvifImageInfo> for avifImage {
    fn from(info: &AvifImageInfo) -> Self {
        avifImage {
            width: info.width,
            height: info.height,
            depth: info.depth as u32,
            yuvFormat: info.yuv_format.into(),
            yuvRange: info.full_range.into(),
            yuvChromaSamplePosition: info.chroma_sample_position.into(),
            alphaPremultiplied: info.alpha_premultiplied as avifBool,
            ..Self::default()
        }
    }
}

impl From<&AvifImage> for avifImage {
    fn from(image: &AvifImage) -> Self {
        let mut dst_image: avifImage = (&image.info).into();
        for i in 0usize..3 {
            if image.yuv_planes[i].is_none() {
                continue;
            }
            dst_image.yuvPlanes[i] = image.yuv_planes[i].unwrap() as *mut u8;
            dst_image.yuvRowBytes[i] = image.yuv_row_bytes[i];
        }
        if image.alpha_plane.is_some() {
            dst_image.alphaPlane = image.alpha_plane.unwrap() as *mut u8;
            dst_image.alphaRowBytes = image.alpha_row_bytes;
        }
        dst_image
    }
}

pub const AVIF_STRICT_DISABLED: u32 = 0;
pub const AVIF_STRICT_PIXI_REQUIRED: u32 = 1 << 0;
pub const AVIF_STRICT_CLAP_VALID: u32 = 1 << 1;
pub const AVIF_STRICT_ALPHA_ISPE_REQUIRED: u32 = 1 << 2;
pub const AVIF_STRICT_ENABLED: u32 =
    AVIF_STRICT_PIXI_REQUIRED | AVIF_STRICT_CLAP_VALID | AVIF_STRICT_ALPHA_ISPE_REQUIRED;
pub type avifStrictFlags = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum avifDecoderSource {
    Auto,
    PrimaryItem,
    Tracks,
}

impl From<avifDecoderSource> for AvifDecoderSource {
    fn from(source: avifDecoderSource) -> Self {
        match source {
            avifDecoderSource::Auto => AvifDecoderSource::Auto,
            avifDecoderSource::PrimaryItem => AvifDecoderSource::PrimaryItem,
            avifDecoderSource::Tracks => AvifDecoderSource::Tracks,
        }
    }
}

#[repr(C)]
pub struct avifDecoder {
    // avifCodecChoice codecChoice;
    pub maxThreads: i32,
    pub requestedSource: avifDecoderSource,
    pub allowIncremental: avifBool,
    pub allowProgressive: avifBool,
    pub ignoreExif: avifBool,
    pub ignoreXMP: avifBool,
    // uint32_t imageSizeLimit;
    // uint32_t imageDimensionLimit;
    // uint32_t imageCountLimit;
    pub strictFlags: avifStrictFlags,

    // Output params.
    pub image: *mut avifImage,
    pub imageIndex: i32,
    pub imageCount: i32,
    // avifProgressiveState progressiveState; // See avifProgressiveState declaration
    // avifImageTiming imageTiming;           //
    pub timescale: u64,
    pub duration: f64,
    pub durationInTimescales: u64,
    pub repetitionCount: i32,

    pub alphaPresent: avifBool,

    //avifIOStats ioStats;

    //avifDiagnostics diag;

    //avifIO * io;

    //struct avifDecoderData * data;

    //avifBool gainMapPresent;
    // avifBool enableDecodingGainMap;
    // avifBool enableParsingGainMapMetadata;
    // avifBool ignoreColorAndAlpha;
    pub imageSequenceTrackPresent: avifBool,

    // TODO: maybe wrap these fields in a private data kind of field?
    rust_decoder: Box<AvifDecoder>,
    image_object: avifImage,
}

impl Default for avifDecoder {
    fn default() -> Self {
        Self {
            maxThreads: 1,
            requestedSource: avifDecoderSource::Auto,
            allowIncremental: AVIF_FALSE,
            allowProgressive: AVIF_FALSE,
            ignoreExif: AVIF_FALSE,
            ignoreXMP: AVIF_FALSE,
            strictFlags: AVIF_STRICT_ENABLED,
            image: std::ptr::null_mut(),
            imageIndex: -1,
            imageCount: 0,
            timescale: 0,
            duration: 0.0,
            durationInTimescales: 0,
            repetitionCount: 0,
            alphaPresent: AVIF_FALSE,
            imageSequenceTrackPresent: AVIF_FALSE,
            rust_decoder: Box::new(AvifDecoder::default()),
            image_object: avifImage::default(),
        }
    }
}

fn to_avifBool(val: bool) -> avifBool {
    if val {
        AVIF_TRUE
    } else {
        AVIF_FALSE
    }
}

fn to_avifResult<T>(res: &AvifResult<T>) -> avifResult {
    match res {
        Ok(_) => avifResult::Ok,
        Err(err) => {
            let res: avifResult = err.into();
            res
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn avifPeekCompatibleFileType(input: *const avifROData) -> avifBool {
    let data = slice::from_raw_parts((*input).data, (*input).size);
    to_avifBool(AvifDecoder::peek_compatible_file_type(data))
}

#[no_mangle]
pub unsafe extern "C" fn avifDecoderCreate() -> *mut avifDecoder {
    Box::into_raw(Box::new(avifDecoder::default()))
}

#[no_mangle]
pub unsafe extern "C" fn avifDecoderSetIOFile(
    decoder: *mut avifDecoder,
    filename: *const c_char,
) -> avifResult {
    let rust_decoder = &mut (*decoder).rust_decoder;
    let filename = CStr::from_ptr(filename).to_str().unwrap_or("");
    let filename = String::from(filename);
    to_avifResult(&rust_decoder.set_io_file(&filename))
}

#[no_mangle]
pub unsafe extern "C" fn avifDecoderSetSource(
    decoder: *mut avifDecoder,
    source: avifDecoderSource,
) -> avifResult {
    (*decoder).requestedSource = source;
    // TODO: should decoder be reset here in case this is called after parse?
    avifResult::Ok
}

impl From<&avifDecoder> for AvifDecoderSettings {
    fn from(decoder: &avifDecoder) -> Self {
        let strictness = if decoder.strictFlags == AVIF_STRICT_DISABLED {
            AvifStrictness::None
        } else if decoder.strictFlags == AVIF_STRICT_ENABLED {
            AvifStrictness::All
        } else {
            let mut flags: Vec<AvifStrictnessFlag> = Vec::new();
            if (decoder.strictFlags & AVIF_STRICT_PIXI_REQUIRED) != 0 {
                flags.push(AvifStrictnessFlag::PixiRequired);
            }
            if (decoder.strictFlags & AVIF_STRICT_CLAP_VALID) != 0 {
                flags.push(AvifStrictnessFlag::ClapValid);
            }
            if (decoder.strictFlags & AVIF_STRICT_ALPHA_ISPE_REQUIRED) != 0 {
                flags.push(AvifStrictnessFlag::AlphaIspeRequired);
            }
            AvifStrictness::SpecificInclude(flags)
        };
        Self {
            source: decoder.requestedSource.into(),
            strictness,
            ..Self::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn avifDecoderParse(decoder: *mut avifDecoder) -> avifResult {
    let rust_decoder = &mut (*decoder).rust_decoder;
    rust_decoder.settings = (&(*decoder)).into();

    println!("settings: {:#?}", rust_decoder.settings);

    let res = rust_decoder.parse();
    if res.is_err() {
        return to_avifResult(&res);
    }

    // Copy image info.
    let info = res.unwrap();
    (*decoder).image_object = info.into();

    // Copy decoder properties. Properties from |info| must be copied first to
    // not mess with the borrow checker.
    (*decoder).alphaPresent = to_avifBool(info.alpha_present);
    (*decoder).imageSequenceTrackPresent = to_avifBool(info.image_sequence_track_present);
    (*decoder).imageCount = rust_decoder.image_count as i32;
    (*decoder).image = (&mut (*decoder).image_object) as *mut avifImage;

    avifResult::Ok
}

#[no_mangle]
pub unsafe extern "C" fn avifDecoderNextImage(decoder: *mut avifDecoder) -> avifResult {
    let rust_decoder = &mut (*decoder).rust_decoder;

    let res = rust_decoder.next_image();
    if res.is_err() {
        return to_avifResult(&res);
    }

    // Copy image.
    let image = res.unwrap();
    (*decoder).image_object = image.into();

    // Copy decoder properties. Properties from |image.info| must be copied first to
    // not mess with the borrow checker.
    (*decoder).alphaPresent = to_avifBool(image.info.alpha_present);
    (*decoder).imageSequenceTrackPresent = to_avifBool(image.info.image_sequence_track_present);
    (*decoder).imageCount = rust_decoder.image_count as i32;
    (*decoder).image = (&mut (*decoder).image_object) as *mut avifImage;

    avifResult::Ok
}

#[no_mangle]
pub unsafe extern "C" fn avifDecoderDestroy(decoder: *mut avifDecoder) {
    let _ = Box::from_raw(decoder);
}
