// @generated
// [#protodoc-title: Brotli Decompressor]
// [#extension: envoy.compression.brotli.decompressor]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brotli {
    /// If true, disables "canny" ring buffer allocation strategy.
    /// Ring buffer is allocated according to window size, despite the real size of the content.
    #[prost(bool, tag="1")]
    pub disable_ring_buffer_reallocation: bool,
    /// Value for decompressor's next output buffer. If not set, defaults to 4096.
    #[prost(message, optional, tag="2")]
    pub chunk_size: ::core::option::Option<::pbjson_types::UInt32Value>,
}
/// Encoded file descriptor set for the `envoy.extensions.compression.brotli.decompressor.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd9, 0x09, 0x0a, 0x40, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x2f, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2f, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72,
    0x65, 0x73, 0x73, 0x6f, 0x72, 0x2f, 0x76, 0x33, 0x2f, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x33, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x2e, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e, 0x64, 0x65, 0x63, 0x6f, 0x6d,
    0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x76, 0x33, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x77, 0x72, 0x61, 0x70,
    0x70, 0x65, 0x72, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61,
    0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x9c, 0x01, 0x0a, 0x06, 0x42, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x12, 0x47, 0x0a,
    0x20, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x62, 0x75,
    0x66, 0x66, 0x65, 0x72, 0x5f, 0x72, 0x65, 0x61, 0x6c, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x1d, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65,
    0x52, 0x69, 0x6e, 0x67, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x52, 0x65, 0x61, 0x6c, 0x6c, 0x6f,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x49, 0x0a, 0x0a, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x5f,
    0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x55, 0x49, 0x6e,
    0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x0c, 0xfa, 0x42, 0x09, 0x2a, 0x07, 0x18,
    0x80, 0x80, 0x04, 0x28, 0x80, 0x20, 0x52, 0x09, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x53, 0x69, 0x7a,
    0x65, 0x42, 0xc5, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x41, 0x69, 0x6f,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x63, 0x6f, 0x6d,
    0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e,
    0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x76, 0x33, 0x42,
    0x0b, 0x42, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x69,
    0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x2f, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2f, 0x64, 0x65, 0x63, 0x6f, 0x6d,
    0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2f, 0x76, 0x33, 0x3b, 0x64, 0x65, 0x63, 0x6f, 0x6d,
    0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x76, 0x33, 0x4a, 0x98, 0x05, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x19, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x3c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x04, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x27, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x09, 0x00, 0x5a, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09, 0x00, 0x5a, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03,
    0x0a, 0x00, 0x2c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04, 0x0c,
    0x00, 0x80, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x04, 0x0c, 0x00, 0x80, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80,
    0x99, 0x6a, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x69, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x12, 0x00, 0x19, 0x01, 0x32, 0x5d, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f,
    0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x42, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x20,
    0x44, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x5d, 0x0a, 0x20, 0x5b,
    0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x62, 0x72,
    0x6f, 0x74, 0x6c, 0x69, 0x2e, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f,
    0x72, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x12, 0x08, 0x0e, 0x0a,
    0xa4, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x2c, 0x1a, 0x96, 0x01,
    0x20, 0x49, 0x66, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c, 0x20, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c,
    0x65, 0x73, 0x20, 0x22, 0x63, 0x61, 0x6e, 0x6e, 0x79, 0x22, 0x20, 0x72, 0x69, 0x6e, 0x67, 0x20,
    0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x73, 0x74, 0x72, 0x61, 0x74, 0x65, 0x67, 0x79, 0x2e, 0x0a, 0x20, 0x52, 0x69, 0x6e,
    0x67, 0x20, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x6f,
    0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x20,
    0x74, 0x6f, 0x20, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x2c, 0x20,
    0x64, 0x65, 0x73, 0x70, 0x69, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x61, 0x6c,
    0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x15, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15,
    0x07, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x2a, 0x2b,
    0x0a, 0x59, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x18, 0x02, 0x60, 0x1a, 0x4c, 0x20,
    0x56, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70,
    0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x27, 0x73, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x6f, 0x75,
    0x74, 0x70, 0x75, 0x74, 0x20, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x2e, 0x20, 0x49, 0x66, 0x20,
    0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x34, 0x30, 0x39, 0x36, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x18, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x18, 0x1e, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x18, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03,
    0x18, 0x2d, 0x5f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x05, 0x12,
    0x03, 0x18, 0x2e, 0x5e, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.compression.brotli.decompressor.v3.serde.rs");
// @@protoc_insertion_point(module)