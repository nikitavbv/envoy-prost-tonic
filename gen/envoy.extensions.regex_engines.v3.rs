// @generated
// [#protodoc-title: Google RE2]
// [#extension: envoy.regex_engines.google_re2]

/// Google's `RE2 <<https://github.com/google/re2>`_> regex engine. The regex string must adhere to
/// the documented `syntax <<https://github.com/google/re2/wiki/Syntax>`_.> The engine is designed
/// to complete execution in linear time as well as limit the amount of memory used.
///
/// Envoy emits two stats for tracking the program size of regexes: the histogram ``re2.program_size``,
/// which records the program size, and the counter ``re2.exceeded_warn_level``, which is incremented
/// each time the program size exceeds the warn level threshold.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleRe2 {
}
/// Encoded file descriptor set for the `envoy.extensions.regex_engines.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xcb, 0x08, 0x0a, 0x32, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x72, 0x65, 0x67, 0x65, 0x78, 0x5f, 0x65, 0x6e, 0x67, 0x69,
    0x6e, 0x65, 0x73, 0x2f, 0x76, 0x33, 0x2f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x5f, 0x72, 0x65,
    0x32, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x21, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x72, 0x65, 0x67, 0x65, 0x78, 0x5f,
    0x65, 0x6e, 0x67, 0x69, 0x6e, 0x65, 0x73, 0x2e, 0x76, 0x33, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61,
    0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x0b, 0x0a, 0x09, 0x47, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x52, 0x45, 0x32, 0x42, 0xa5, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10,
    0x02, 0x0a, 0x2f, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x2e, 0x72, 0x65, 0x67, 0x65, 0x78, 0x5f, 0x65, 0x6e, 0x67, 0x69, 0x6e, 0x65, 0x73, 0x2e,
    0x76, 0x33, 0x42, 0x0e, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x52, 0x65, 0x32, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x50, 0x01, 0x5a, 0x58, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d,
    0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x72, 0x65,
    0x67, 0x65, 0x78, 0x5f, 0x65, 0x6e, 0x67, 0x69, 0x6e, 0x65, 0x73, 0x2f, 0x76, 0x33, 0x3b, 0x72,
    0x65, 0x67, 0x65, 0x78, 0x5f, 0x65, 0x6e, 0x67, 0x69, 0x6e, 0x65, 0x73, 0x76, 0x33, 0x4a, 0x95,
    0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x17, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x2a, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06,
    0x00, 0x48, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x06, 0x00, 0x48, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x07,
    0x00, 0x2f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x0a, 0x12, 0x03, 0x08, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00,
    0x6f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x09, 0x00, 0x6f, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0a, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02,
    0x12, 0x03, 0x0a, 0x00, 0x46, 0x0a, 0xf2, 0x04, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x16, 0x00,
    0x17, 0x01, 0x1a, 0x96, 0x04, 0x20, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x27, 0x73, 0x20, 0x60,
    0x52, 0x45, 0x32, 0x20, 0x3c, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x67, 0x69, 0x74,
    0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x72,
    0x65, 0x32, 0x3e, 0x60, 0x5f, 0x20, 0x72, 0x65, 0x67, 0x65, 0x78, 0x20, 0x65, 0x6e, 0x67, 0x69,
    0x6e, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x65, 0x67, 0x65, 0x78, 0x20, 0x73, 0x74,
    0x72, 0x69, 0x6e, 0x67, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x61, 0x64, 0x68, 0x65, 0x72, 0x65,
    0x20, 0x74, 0x6f, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e,
    0x74, 0x65, 0x64, 0x20, 0x60, 0x73, 0x79, 0x6e, 0x74, 0x61, 0x78, 0x20, 0x3c, 0x68, 0x74, 0x74,
    0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f,
    0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x72, 0x65, 0x32, 0x2f, 0x77, 0x69, 0x6b, 0x69, 0x2f,
    0x53, 0x79, 0x6e, 0x74, 0x61, 0x78, 0x3e, 0x60, 0x5f, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65,
    0x6e, 0x67, 0x69, 0x6e, 0x65, 0x20, 0x69, 0x73, 0x20, 0x64, 0x65, 0x73, 0x69, 0x67, 0x6e, 0x65,
    0x64, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x20, 0x65,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x20, 0x6c, 0x69, 0x6e, 0x65,
    0x61, 0x72, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x61, 0x73, 0x20, 0x77, 0x65, 0x6c, 0x6c, 0x20,
    0x61, 0x73, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x6d, 0x6f,
    0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x65, 0x6d, 0x69, 0x74,
    0x73, 0x20, 0x74, 0x77, 0x6f, 0x20, 0x73, 0x74, 0x61, 0x74, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x74, 0x72, 0x61, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f,
    0x67, 0x72, 0x61, 0x6d, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x67,
    0x65, 0x78, 0x65, 0x73, 0x3a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67,
    0x72, 0x61, 0x6d, 0x20, 0x60, 0x60, 0x72, 0x65, 0x32, 0x2e, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61,
    0x6d, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x60, 0x60, 0x2c, 0x0a, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68,
    0x20, 0x72, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f,
    0x67, 0x72, 0x61, 0x6d, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x60, 0x60, 0x72, 0x65, 0x32,
    0x2e, 0x65, 0x78, 0x63, 0x65, 0x65, 0x64, 0x65, 0x64, 0x5f, 0x77, 0x61, 0x72, 0x6e, 0x5f, 0x6c,
    0x65, 0x76, 0x65, 0x6c, 0x60, 0x60, 0x2c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x73,
    0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x0a, 0x20, 0x65, 0x61,
    0x63, 0x68, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x67,
    0x72, 0x61, 0x6d, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x65, 0x78, 0x63, 0x65, 0x65, 0x64, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x61, 0x72, 0x6e, 0x20, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20,
    0x74, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64, 0x2e, 0x0a, 0x32, 0x4d, 0x20, 0x5b, 0x23,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20,
    0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x20, 0x52, 0x45, 0x32, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x72, 0x65, 0x67, 0x65, 0x78, 0x5f, 0x65, 0x6e, 0x67, 0x69, 0x6e, 0x65, 0x73, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x5f, 0x72, 0x65, 0x32, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x16, 0x08, 0x11, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)