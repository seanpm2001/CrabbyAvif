/* automatically generated by rust-bindgen 0.69.1 */

#![allow(warnings)]

pub const Libgav1ChromaSamplePosition_kLibgav1ChromaSamplePositionUnknown:
    Libgav1ChromaSamplePosition = 0;
pub const Libgav1ChromaSamplePosition_kLibgav1ChromaSamplePositionVertical:
    Libgav1ChromaSamplePosition = 1;
pub const Libgav1ChromaSamplePosition_kLibgav1ChromaSamplePositionColocated:
    Libgav1ChromaSamplePosition = 2;
pub const Libgav1ChromaSamplePosition_kLibgav1ChromaSamplePositionReserved:
    Libgav1ChromaSamplePosition = 3;
pub type Libgav1ChromaSamplePosition = ::std::os::raw::c_uint;
pub const Libgav1ImageFormat_kLibgav1ImageFormatYuv420: Libgav1ImageFormat = 0;
pub const Libgav1ImageFormat_kLibgav1ImageFormatYuv422: Libgav1ImageFormat = 1;
pub const Libgav1ImageFormat_kLibgav1ImageFormatYuv444: Libgav1ImageFormat = 2;
pub const Libgav1ImageFormat_kLibgav1ImageFormatMonochrome400: Libgav1ImageFormat = 3;
pub type Libgav1ImageFormat = ::std::os::raw::c_uint;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimaryBt709: Libgav1ColorPrimary = 1;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimaryUnspecified: Libgav1ColorPrimary = 2;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimaryBt470M: Libgav1ColorPrimary = 4;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimaryBt470Bg: Libgav1ColorPrimary = 5;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimaryBt601: Libgav1ColorPrimary = 6;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimarySmpte240: Libgav1ColorPrimary = 7;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimaryGenericFilm: Libgav1ColorPrimary = 8;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimaryBt2020: Libgav1ColorPrimary = 9;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimaryXyz: Libgav1ColorPrimary = 10;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimarySmpte431: Libgav1ColorPrimary = 11;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimarySmpte432: Libgav1ColorPrimary = 12;
pub const Libgav1ColorPrimary_kLibgav1ColorPrimaryEbu3213: Libgav1ColorPrimary = 22;
pub const Libgav1ColorPrimary_kLibgav1MaxColorPrimaries: Libgav1ColorPrimary = 255;
pub type Libgav1ColorPrimary = ::std::os::raw::c_uint;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsBt709:
    Libgav1TransferCharacteristics = 1;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsUnspecified:
    Libgav1TransferCharacteristics = 2;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsBt470M:
    Libgav1TransferCharacteristics = 4;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsBt470Bg:
    Libgav1TransferCharacteristics = 5;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsBt601:
    Libgav1TransferCharacteristics = 6;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsSmpte240:
    Libgav1TransferCharacteristics = 7;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsLinear:
    Libgav1TransferCharacteristics = 8;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsLog100:
    Libgav1TransferCharacteristics = 9;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsLog100Sqrt10:
    Libgav1TransferCharacteristics = 10;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsIec61966:
    Libgav1TransferCharacteristics = 11;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsBt1361:
    Libgav1TransferCharacteristics = 12;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsSrgb:
    Libgav1TransferCharacteristics = 13;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsBt2020TenBit:
    Libgav1TransferCharacteristics = 14;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsBt2020TwelveBit:
    Libgav1TransferCharacteristics = 15;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsSmpte2084:
    Libgav1TransferCharacteristics = 16;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsSmpte428:
    Libgav1TransferCharacteristics = 17;
pub const Libgav1TransferCharacteristics_kLibgav1TransferCharacteristicsHlg:
    Libgav1TransferCharacteristics = 18;
pub const Libgav1TransferCharacteristics_kLibgav1MaxTransferCharacteristics:
    Libgav1TransferCharacteristics = 255;
pub type Libgav1TransferCharacteristics = ::std::os::raw::c_uint;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsIdentity: Libgav1MatrixCoefficients =
    0;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsBt709: Libgav1MatrixCoefficients = 1;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsUnspecified:
    Libgav1MatrixCoefficients = 2;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsFcc: Libgav1MatrixCoefficients = 4;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsBt470BG: Libgav1MatrixCoefficients =
    5;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsBt601: Libgav1MatrixCoefficients = 6;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsSmpte240: Libgav1MatrixCoefficients =
    7;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsSmpteYcgco:
    Libgav1MatrixCoefficients = 8;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsBt2020Ncl: Libgav1MatrixCoefficients =
    9;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsBt2020Cl: Libgav1MatrixCoefficients =
    10;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsSmpte2085: Libgav1MatrixCoefficients =
    11;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsChromatNcl:
    Libgav1MatrixCoefficients = 12;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsChromatCl: Libgav1MatrixCoefficients =
    13;
pub const Libgav1MatrixCoefficients_kLibgav1MatrixCoefficientsIctcp: Libgav1MatrixCoefficients = 14;
pub const Libgav1MatrixCoefficients_kLibgav1MaxMatrixCoefficients: Libgav1MatrixCoefficients = 255;
pub type Libgav1MatrixCoefficients = ::std::os::raw::c_uint;
pub const Libgav1ColorRange_kLibgav1ColorRangeStudio: Libgav1ColorRange = 0;
pub const Libgav1ColorRange_kLibgav1ColorRangeFull: Libgav1ColorRange = 1;
pub type Libgav1ColorRange = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Libgav1ObuMetadataHdrCll {
    pub max_cll: u16,
    pub max_fall: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Libgav1ObuMetadataHdrMdcv {
    pub primary_chromaticity_x: [u16; 3usize],
    pub primary_chromaticity_y: [u16; 3usize],
    pub white_point_chromaticity_x: u16,
    pub white_point_chromaticity_y: u16,
    pub luminance_max: u32,
    pub luminance_min: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Libgav1ObuMetadataItutT35 {
    pub country_code: u8,
    pub country_code_extension_byte: u8,
    pub payload_bytes: *mut u8,
    pub payload_size: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Libgav1DecoderBuffer {
    pub chroma_sample_position: Libgav1ChromaSamplePosition,
    pub image_format: Libgav1ImageFormat,
    pub color_range: Libgav1ColorRange,
    pub color_primary: Libgav1ColorPrimary,
    pub transfer_characteristics: Libgav1TransferCharacteristics,
    pub matrix_coefficients: Libgav1MatrixCoefficients,
    pub bitdepth: ::std::os::raw::c_int,
    pub displayed_width: [::std::os::raw::c_int; 3usize],
    pub displayed_height: [::std::os::raw::c_int; 3usize],
    pub stride: [::std::os::raw::c_int; 3usize],
    pub plane: [*mut u8; 3usize],
    pub spatial_id: ::std::os::raw::c_int,
    pub temporal_id: ::std::os::raw::c_int,
    pub hdr_cll: Libgav1ObuMetadataHdrCll,
    pub has_hdr_cll: ::std::os::raw::c_int,
    pub hdr_mdcv: Libgav1ObuMetadataHdrMdcv,
    pub has_hdr_mdcv: ::std::os::raw::c_int,
    pub itut_t35: Libgav1ObuMetadataItutT35,
    pub has_itut_t35: ::std::os::raw::c_int,
    pub user_private_data: i64,
    pub buffer_private_data: *mut ::std::os::raw::c_void,
}
pub const Libgav1StatusCode_kLibgav1StatusOk: Libgav1StatusCode = 0;
pub const Libgav1StatusCode_kLibgav1StatusUnknownError: Libgav1StatusCode = -1;
pub const Libgav1StatusCode_kLibgav1StatusInvalidArgument: Libgav1StatusCode = -2;
pub const Libgav1StatusCode_kLibgav1StatusOutOfMemory: Libgav1StatusCode = -3;
pub const Libgav1StatusCode_kLibgav1StatusResourceExhausted: Libgav1StatusCode = -4;
pub const Libgav1StatusCode_kLibgav1StatusNotInitialized: Libgav1StatusCode = -5;
pub const Libgav1StatusCode_kLibgav1StatusAlready: Libgav1StatusCode = -6;
pub const Libgav1StatusCode_kLibgav1StatusUnimplemented: Libgav1StatusCode = -7;
pub const Libgav1StatusCode_kLibgav1StatusInternalError: Libgav1StatusCode = -8;
pub const Libgav1StatusCode_kLibgav1StatusBitstreamError: Libgav1StatusCode = -9;
pub const Libgav1StatusCode_kLibgav1StatusTryAgain: Libgav1StatusCode = -10;
pub const Libgav1StatusCode_kLibgav1StatusNothingToDequeue: Libgav1StatusCode = -11;
pub const Libgav1StatusCode_kLibgav1StatusReservedForFutureExpansionUseDefaultInSwitchInstead_:
    Libgav1StatusCode = -1000;
pub type Libgav1StatusCode = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Libgav1FrameBuffer {
    pub plane: [*mut u8; 3usize],
    pub stride: [::std::os::raw::c_int; 3usize],
    pub private_data: *mut ::std::os::raw::c_void,
}
pub type Libgav1FrameBufferSizeChangedCallback = ::std::option::Option<
    unsafe extern "C" fn(
        callback_private_data: *mut ::std::os::raw::c_void,
        bitdepth: ::std::os::raw::c_int,
        image_format: Libgav1ImageFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        left_border: ::std::os::raw::c_int,
        right_border: ::std::os::raw::c_int,
        top_border: ::std::os::raw::c_int,
        bottom_border: ::std::os::raw::c_int,
        stride_alignment: ::std::os::raw::c_int,
    ) -> Libgav1StatusCode,
>;
pub type Libgav1GetFrameBufferCallback = ::std::option::Option<
    unsafe extern "C" fn(
        callback_private_data: *mut ::std::os::raw::c_void,
        bitdepth: ::std::os::raw::c_int,
        image_format: Libgav1ImageFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        left_border: ::std::os::raw::c_int,
        right_border: ::std::os::raw::c_int,
        top_border: ::std::os::raw::c_int,
        bottom_border: ::std::os::raw::c_int,
        stride_alignment: ::std::os::raw::c_int,
        frame_buffer: *mut Libgav1FrameBuffer,
    ) -> Libgav1StatusCode,
>;
pub type Libgav1ReleaseFrameBufferCallback = ::std::option::Option<
    unsafe extern "C" fn(
        callback_private_data: *mut ::std::os::raw::c_void,
        buffer_private_data: *mut ::std::os::raw::c_void,
    ),
>;
pub type Libgav1ReleaseInputBufferCallback = ::std::option::Option<
    unsafe extern "C" fn(
        callback_private_data: *mut ::std::os::raw::c_void,
        buffer_private_data: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Libgav1DecoderSettings {
    pub threads: ::std::os::raw::c_int,
    pub frame_parallel: ::std::os::raw::c_int,
    pub blocking_dequeue: ::std::os::raw::c_int,
    pub on_frame_buffer_size_changed: Libgav1FrameBufferSizeChangedCallback,
    pub get_frame_buffer: Libgav1GetFrameBufferCallback,
    pub release_frame_buffer: Libgav1ReleaseFrameBufferCallback,
    pub release_input_buffer: Libgav1ReleaseInputBufferCallback,
    pub callback_private_data: *mut ::std::os::raw::c_void,
    pub output_all_layers: ::std::os::raw::c_int,
    pub operating_point: ::std::os::raw::c_int,
    pub post_filter_mask: u8,
}
extern "C" {
    pub fn Libgav1DecoderSettingsInitDefault(settings: *mut Libgav1DecoderSettings);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Libgav1Decoder {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Libgav1DecoderCreate(
        settings: *const Libgav1DecoderSettings,
        decoder_out: *mut *mut Libgav1Decoder,
    ) -> Libgav1StatusCode;
}
extern "C" {
    pub fn Libgav1DecoderDestroy(decoder: *mut Libgav1Decoder);
}
extern "C" {
    pub fn Libgav1DecoderEnqueueFrame(
        decoder: *mut Libgav1Decoder,
        data: *const u8,
        size: usize,
        user_private_data: i64,
        buffer_private_data: *mut ::std::os::raw::c_void,
    ) -> Libgav1StatusCode;
}
extern "C" {
    pub fn Libgav1DecoderDequeueFrame(
        decoder: *mut Libgav1Decoder,
        out_ptr: *mut *const Libgav1DecoderBuffer,
    ) -> Libgav1StatusCode;
}
