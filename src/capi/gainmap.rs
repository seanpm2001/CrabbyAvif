// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::image::*;
use super::io::*;
use super::types::*;

use crate::decoder::gainmap::*;
use crate::image::YuvRange;
use crate::parser::mp4box::*;
use crate::*;

pub type avifContentLightLevelInformationBox = ContentLightLevelInformation;

#[repr(C)]
#[derive(Debug, Default)]
pub struct avifGainMapMetadata {
    pub gainMapMinN: [i32; 3],
    pub gainMapMinD: [u32; 3],
    pub gainMapMaxN: [i32; 3],
    pub gainMapMaxD: [u32; 3],
    pub gainMapGammaN: [u32; 3],
    pub gainMapGammaD: [u32; 3],
    pub baseOffsetN: [i32; 3],
    pub baseOffsetD: [u32; 3],
    pub alternateOffsetN: [i32; 3],
    pub alternateOffsetD: [u32; 3],
    pub baseHdrHeadroomN: u32,
    pub baseHdrHeadroomD: u32,
    pub alternateHdrHeadroomN: u32,
    pub alternateHdrHeadroomD: u32,
    pub useBaseColorSpace: avifBool,
}

#[repr(C)]
#[derive(Debug)]
pub struct avifGainMap {
    pub image: *mut avifImage,
    pub gainMapMinN: [i32; 3],
    pub gainMapMinD: [u32; 3],
    pub gainMapMaxN: [i32; 3],
    pub gainMapMaxD: [u32; 3],
    pub gainMapGammaN: [u32; 3],
    pub gainMapGammaD: [u32; 3],
    pub baseOffsetN: [i32; 3],
    pub baseOffsetD: [u32; 3],
    pub alternateOffsetN: [i32; 3],
    pub alternateOffsetD: [u32; 3],
    pub baseHdrHeadroomN: u32,
    pub baseHdrHeadroomD: u32,
    pub alternateHdrHeadroomN: u32,
    pub alternateHdrHeadroomD: u32,
    pub useBaseColorSpace: avifBool,
    pub altICC: avifRWData,
    pub altColorPrimaries: ColorPrimaries,
    pub altTransferCharacteristics: TransferCharacteristics,
    pub altMatrixCoefficients: MatrixCoefficients,
    pub altYUVRange: YuvRange,
    pub altDepth: u32,
    pub altPlaneCount: u32,
    pub altCLLI: avifContentLightLevelInformationBox,
}

impl Default for avifGainMap {
    fn default() -> Self {
        avifGainMap {
            image: std::ptr::null_mut(),
            gainMapMinN: [1, 1, 1],
            gainMapMinD: [1, 1, 1],
            gainMapMaxN: [1, 1, 1],
            gainMapMaxD: [1, 1, 1],
            gainMapGammaN: [1, 1, 1],
            gainMapGammaD: [1, 1, 1],
            baseOffsetN: [1, 1, 1],
            baseOffsetD: [64, 64, 64],
            alternateOffsetN: [1, 1, 1],
            alternateOffsetD: [64, 64, 64],
            baseHdrHeadroomN: 0,
            baseHdrHeadroomD: 1,
            alternateHdrHeadroomN: 1,
            alternateHdrHeadroomD: 1,
            useBaseColorSpace: to_avifBool(false),
            altICC: avifRWData::default(),
            altColorPrimaries: ColorPrimaries::default(),
            altTransferCharacteristics: TransferCharacteristics::default(),
            altMatrixCoefficients: MatrixCoefficients::default(),
            altYUVRange: YuvRange::Full,
            altDepth: 0,
            altPlaneCount: 0,
            altCLLI: Default::default(),
        }
    }
}

impl From<&GainMap> for avifGainMap {
    fn from(gainmap: &GainMap) -> Self {
        avifGainMap {
            gainMapMinN: [
                gainmap.metadata.min[0].0,
                gainmap.metadata.min[1].0,
                gainmap.metadata.min[2].0,
            ],
            gainMapMinD: [
                gainmap.metadata.min[0].1,
                gainmap.metadata.min[1].1,
                gainmap.metadata.min[2].1,
            ],
            gainMapMaxN: [
                gainmap.metadata.max[0].0,
                gainmap.metadata.max[1].0,
                gainmap.metadata.max[2].0,
            ],
            gainMapMaxD: [
                gainmap.metadata.max[0].1,
                gainmap.metadata.max[1].1,
                gainmap.metadata.max[2].1,
            ],
            gainMapGammaN: [
                gainmap.metadata.gamma[0].0,
                gainmap.metadata.gamma[1].0,
                gainmap.metadata.gamma[2].0,
            ],
            gainMapGammaD: [
                gainmap.metadata.gamma[0].1,
                gainmap.metadata.gamma[1].1,
                gainmap.metadata.gamma[2].1,
            ],
            baseOffsetN: [
                gainmap.metadata.base_offset[0].0,
                gainmap.metadata.base_offset[1].0,
                gainmap.metadata.base_offset[2].0,
            ],
            baseOffsetD: [
                gainmap.metadata.base_offset[0].1,
                gainmap.metadata.base_offset[1].1,
                gainmap.metadata.base_offset[2].1,
            ],
            alternateOffsetN: [
                gainmap.metadata.alternate_offset[0].0,
                gainmap.metadata.alternate_offset[1].0,
                gainmap.metadata.alternate_offset[2].0,
            ],
            alternateOffsetD: [
                gainmap.metadata.alternate_offset[0].1,
                gainmap.metadata.alternate_offset[1].1,
                gainmap.metadata.alternate_offset[2].1,
            ],
            baseHdrHeadroomN: gainmap.metadata.base_hdr_headroom.0,
            baseHdrHeadroomD: gainmap.metadata.base_hdr_headroom.1,
            alternateHdrHeadroomN: gainmap.metadata.alternate_hdr_headroom.0,
            alternateHdrHeadroomD: gainmap.metadata.alternate_hdr_headroom.1,
            useBaseColorSpace: gainmap.metadata.use_base_color_space as avifBool,
            altICC: (&gainmap.alt_icc).into(),
            altColorPrimaries: gainmap.alt_color_primaries,
            altTransferCharacteristics: gainmap.alt_transfer_characteristics,
            altMatrixCoefficients: gainmap.alt_matrix_coefficients,
            altYUVRange: gainmap.alt_yuv_range,
            altDepth: u32::from(gainmap.alt_plane_depth),
            altPlaneCount: u32::from(gainmap.alt_plane_count),
            altCLLI: gainmap.alt_clli,
            ..Self::default()
        }
    }
}
