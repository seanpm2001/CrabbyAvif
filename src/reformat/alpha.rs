use super::libyuv;
use super::rgb;

use crate::image::Plane;
use crate::internal_utils::pixels::*;
use crate::internal_utils::*;
use crate::*;

impl rgb::Image {
    pub fn premultiply_alpha(&mut self) -> AvifResult<()> {
        if self.pixels().is_null() || self.row_bytes == 0 {
            return Err(AvifError::ReformatFailed);
        }
        if !self.has_alpha() {
            return Err(AvifError::InvalidArgument);
        }
        match libyuv::process_alpha(self, true) {
            Ok(_) => return Ok(()),
            Err(err) => {
                if err != AvifError::NotImplemented {
                    return Err(err);
                }
            }
        }
        unimplemented!("native alpha multiply implementation");
    }

    pub fn unpremultiply_alpha(&mut self) -> AvifResult<()> {
        if self.pixels().is_null() || self.row_bytes == 0 {
            return Err(AvifError::ReformatFailed);
        }
        if !self.has_alpha() {
            return Err(AvifError::InvalidArgument);
        }
        match libyuv::process_alpha(self, false) {
            Ok(_) => return Ok(()),
            Err(err) => {
                if err != AvifError::NotImplemented {
                    return Err(err);
                }
            }
        }
        unimplemented!("native alpha unmultiply implementation");
    }

    pub fn fill_alpha(&mut self) -> AvifResult<()> {
        if !self.has_alpha() {
            return Err(AvifError::InvalidArgument);
        }
        let alpha_offset = self.format.alpha_offset();
        if self.depth > 8 {
            let max_channel = ((1 << self.depth) - 1) as u16;
            for y in 0..self.height {
                let width = usize_from_u32(self.width)?;
                let row = self.row16_mut(y)?;
                for x in 0..width {
                    row[(x * 4) + alpha_offset] = max_channel;
                }
            }
        } else {
            for y in 0..self.height {
                let width = usize_from_u32(self.width)?;
                let row = self.row_mut(y)?;
                for x in 0..width {
                    row[(x * 4) + alpha_offset] = 255;
                }
            }
        }
        Ok(())
    }

    // TODO: Add test for this function.
    fn rescale_alpha_value(value: u16, src_max_channel_f: f32, dst_max_channel: u16) -> u16 {
        let alpha_f = (value as f32) / src_max_channel_f;
        let dst_max_channel_f = dst_max_channel as f32;
        let alpha = (0.5 + (alpha_f * dst_max_channel_f)) as u16;
        clamp_u16(alpha, 0, dst_max_channel)
    }

    pub fn reformat_alpha(&mut self, image: &image::Image) -> AvifResult<()> {
        if !self.has_alpha() {
            return Err(AvifError::InvalidArgument);
        }
        let width = usize_from_u32(self.width)?;
        let dst_alpha_offset = self.format.alpha_offset();
        if self.depth == image.depth as u32 {
            if self.depth > 8 {
                for y in 0..self.height {
                    let dst_row = self.row16_mut(y)?;
                    let src_row = image.row16(Plane::A, y)?;
                    for x in 0..width {
                        dst_row[(x * 4) + dst_alpha_offset] = src_row[x];
                    }
                }
                return Ok(());
            }
            for y in 0..self.height {
                let dst_row = self.row_mut(y)?;
                let src_row = image.row(Plane::A, y)?;
                for x in 0..width {
                    dst_row[(x * 4) + dst_alpha_offset] = src_row[x];
                }
            }
            return Ok(());
        }
        let max_channel = self.max_channel();
        if image.depth > 8 {
            if self.depth > 8 {
                // u16 to u16 depth rescaling.
                for y in 0..self.height {
                    let dst_row = self.row16_mut(y)?;
                    let src_row = image.row16(Plane::A, y)?;
                    for x in 0..width {
                        dst_row[(x * 4) + dst_alpha_offset] = Self::rescale_alpha_value(
                            src_row[x],
                            image.max_channel_f(),
                            max_channel,
                        );
                    }
                }
                return Ok(());
            }
            // u16 to u8 depth rescaling.
            for y in 0..self.height {
                let dst_row = self.row_mut(y)?;
                let src_row = image.row16(Plane::A, y)?;
                for x in 0..width {
                    dst_row[(x * 4) + dst_alpha_offset] =
                        Self::rescale_alpha_value(src_row[x], image.max_channel_f(), max_channel)
                            as u8;
                }
            }
            return Ok(());
        }
        // u8 to u16 depth rescaling.
        for y in 0..self.height {
            let dst_row = self.row16_mut(y)?;
            let src_row = image.row(Plane::A, y)?;
            for x in 0..width {
                dst_row[(x * 4) + dst_alpha_offset] = Self::rescale_alpha_value(
                    src_row[x] as u16,
                    image.max_channel_f(),
                    max_channel,
                );
            }
        }
        Ok(())
    }
}

impl image::Image {
    pub fn alpha_to_full_range(&mut self) -> AvifResult<()> {
        if self.planes2[3].is_none() {
            return Ok(());
        }
        if !self.planes2[3].as_ref().unwrap().is_pointer() {
            // TODO: implement this function for non-pointer inputs.
            return Err(AvifError::NotImplemented);
        }
        let src = image::Image {
            width: self.width,
            height: self.height,
            depth: self.depth,
            yuv_format: self.yuv_format,
            planes2: [
                None,
                None,
                None,
                Some(Pixels::Pointer(self.planes2[3].as_ref().unwrap().pointer())),
            ],
            row_bytes: [0, 0, 0, self.row_bytes[3]],
            ..image::Image::default()
        };
        self.allocate_planes(1)?;
        let width = self.width as usize;
        let depth = self.depth;
        if depth > 8 {
            for y in 0..self.height {
                let src_row = src.row16(Plane::A, y)?;
                let dst_row = self.row16_mut(Plane::A, y)?;
                for x in 0..width {
                    dst_row[x] = limited_to_full_y(depth, src_row[x]);
                }
            }
        } else {
            for y in 0..self.height {
                let src_row = src.row(Plane::A, y)?;
                let dst_row = self.row_mut(Plane::A, y)?;
                for x in 0..width {
                    dst_row[x] = limited_to_full_y(8, src_row[x] as u16) as u8;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use test_case::test_matrix;

    const ALPHA_RGB_FORMATS: [rgb::Format; 4] = [
        rgb::Format::Rgba,
        rgb::Format::Argb,
        rgb::Format::Bgra,
        rgb::Format::Abgr,
    ];

    fn rgb_image(
        width: u32,
        height: u32,
        depth: u32,
        format: rgb::Format,
        use_pointer: bool,
        buffer: &mut Vec<u8>,
    ) -> AvifResult<rgb::Image> {
        let mut rgb = rgb::Image {
            width,
            height,
            depth,
            format,
            ..rgb::Image::default()
        };
        if use_pointer {
            let pixel_size = if depth == 8 { 1 } else { 2 };
            let buffer_size = (width * height * 4 * pixel_size) as usize;
            buffer.reserve_exact(buffer_size);
            buffer.resize(buffer_size, 0);
            rgb.pixels = Some(Pixels::Pointer(buffer.as_mut_ptr()));
            rgb.row_bytes = width * 4 * pixel_size;
        } else {
            rgb.allocate()?;
        }
        Ok(rgb)
    }

    #[test_matrix(20, 10, [8, 10, 12, 16], 0..4, [true, false])]
    fn fill_alpha(
        width: u32,
        height: u32,
        depth: u32,
        format_index: usize,
        use_pointer: bool,
    ) -> AvifResult<()> {
        let format = ALPHA_RGB_FORMATS[format_index];
        let mut buffer: Vec<u8> = vec![];
        let mut rgb = rgb_image(width, height, depth, format, use_pointer, &mut buffer)?;

        rgb.fill_alpha()?;

        let alpha_offset = rgb.format.alpha_offset();
        if depth == 8 {
            for y in 0..height {
                let row = rgb.row(y)?;
                assert_eq!(row.len(), (width * 4) as usize);
                for x in 0..width as usize {
                    for idx in 0usize..4 {
                        let expected_value = if idx == alpha_offset { 255 } else { 0 };
                        assert_eq!(row[(x * 4) + idx], expected_value);
                    }
                }
            }
        } else {
            let max_channel = ((1 << depth) - 1) as u16;
            for y in 0..height {
                let row = rgb.row16(y)?;
                assert_eq!(row.len(), (width * 4) as usize);
                for x in 0..width as usize {
                    for idx in 0usize..4 {
                        let expected_value = if idx == alpha_offset { max_channel } else { 0 };
                        assert_eq!(row[(x * 4) + idx], expected_value);
                    }
                }
            }
        }
        Ok(())
    }

    #[test_matrix(20, 10, [8, 10, 12, 16], 0..4, [8, 10, 12], [true, false])]
    fn reformat_alpha(
        width: u32,
        height: u32,
        rgb_depth: u32,
        format_index: usize,
        yuv_depth: u8,
        use_pointer: bool,
    ) -> AvifResult<()> {
        // Note: This test simply makes sure reformat_alpha puts the alpha pixels in the right
        // place in the rgb image (with scaling). It does not check for the actual validity of the
        // scaled pixels.
        let format = ALPHA_RGB_FORMATS[format_index];
        let mut buffer: Vec<u8> = vec![];
        let mut rgb = rgb_image(width, height, rgb_depth, format, use_pointer, &mut buffer)?;

        let mut image = image::Image::default();
        image.width = width;
        image.height = height;
        image.depth = yuv_depth;
        image.allocate_planes(1)?;

        let mut rng = rand::thread_rng();
        let mut expected_values: Vec<u16> = Vec::new();
        let image_max_channel_f = image.max_channel_f();
        if yuv_depth == 8 {
            for y in 0..height {
                let row = image.row_mut(Plane::A, y)?;
                for x in 0..width as usize {
                    let value = rng.gen_range(0..256) as u8;
                    if rgb.depth == 8 {
                        expected_values.push(value as u16);
                    } else {
                        expected_values.push(rgb::Image::rescale_alpha_value(
                            value as u16,
                            image_max_channel_f,
                            rgb.max_channel(),
                        ));
                    }
                    row[x] = value;
                }
            }
        } else {
            for y in 0..height {
                let row = image.row16_mut(Plane::A, y)?;
                for x in 0..width as usize {
                    let value = rng.gen_range(0..(1i32 << yuv_depth)) as u16;
                    if rgb.depth == yuv_depth as u32 {
                        expected_values.push(value);
                    } else {
                        expected_values.push(rgb::Image::rescale_alpha_value(
                            value as u16,
                            image_max_channel_f,
                            rgb.max_channel(),
                        ));
                    }
                    row[x] = value;
                }
            }
        }

        rgb.reformat_alpha(&image)?;

        let alpha_offset = rgb.format.alpha_offset();
        let mut expected_values = expected_values.into_iter();
        if rgb_depth == 8 {
            for y in 0..height {
                let rgb_row = rgb.row(y)?;
                assert_eq!(rgb_row.len(), (width * 4) as usize);
                for x in 0..width as usize {
                    for idx in 0usize..4 {
                        let expected_value =
                            if idx == alpha_offset { expected_values.next().unwrap() } else { 0 };
                        assert_eq!(rgb_row[(x * 4) + idx], expected_value as u8);
                    }
                }
            }
        } else {
            for y in 0..height {
                let rgb_row = rgb.row16(y)?;
                assert_eq!(rgb_row.len(), (width * 4) as usize);
                for x in 0..width as usize {
                    for idx in 0usize..4 {
                        let expected_value =
                            if idx == alpha_offset { expected_values.next().unwrap() } else { 0 };
                        assert_eq!(rgb_row[(x * 4) + idx], expected_value);
                    }
                }
            }
        }
        Ok(())
    }
}
