use crate::image::Image;
use crate::image::ALL_PLANES;
use crate::reformat::rgb;
use crate::OptionExtension;

use std::fs::File;
use std::io::prelude::*;

#[derive(Default)]
pub struct RawWriter {
    pub filename: Option<String>,
    pub rgb: bool,
    file: Option<File>,
}

impl RawWriter {
    pub fn create(filename: &str) -> Self {
        Self {
            filename: Some(filename.to_owned()),
            ..Self::default()
        }
    }

    fn write_header(&mut self) -> bool {
        if self.file.is_none() {
            assert!(self.filename.is_some());
            let file = File::create(self.filename.unwrap_ref());
            if file.is_err() {
                return false;
            }
            self.file = Some(file.unwrap());
        }
        true
    }

    pub fn write_frame(&mut self, image: &Image) -> bool {
        if !self.write_header() {
            return false;
        }
        if self.rgb {
            let mut rgb = rgb::Image::create_from_yuv(image);
            rgb.format = rgb::Format::Rgba;
            rgb.depth = 16;
            //rgb.depth = 8;
            rgb.premultiply_alpha = true;
            rgb.is_float = true;
            if rgb.allocate().is_err() || rgb.convert_from_yuv(image).is_err() {
                return false;
            }
            for y in 0..rgb.height {
                if rgb.depth == 8 {
                    let row = rgb.row(y).unwrap();
                    if self.file.unwrap_ref().write_all(row).is_err() {
                        return false;
                    }
                } else {
                    let row = rgb.row16(y).unwrap();
                    let mut row16: Vec<u8> = Vec::new();
                    for &pixel in row {
                        row16.extend_from_slice(&pixel.to_be_bytes());
                    }
                    if self.file.unwrap_ref().write_all(&row16[..]).is_err() {
                        return false;
                    }
                }
            }
            return true;
        }
        for plane in ALL_PLANES {
            let plane_data = image.plane_data(plane);
            if plane_data.is_none() {
                continue;
            }
            let plane_data = plane_data.unwrap();
            for y in 0..plane_data.height {
                // TODO: Handle row16.
                let row = if let Ok(row) = image.row(plane, y) {
                    row
                } else {
                    return false;
                };
                if self.file.unwrap_ref().write_all(row).is_err() {
                    return false;
                }
            }
        }
        true
    }
}
