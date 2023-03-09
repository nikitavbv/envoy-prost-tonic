// @generated
// [#protodoc-title: Original Src Filter]
// Use the Original source address on upstream connections.
// [#extension: envoy.filters.listener.original_src]

/// The Original Src filter binds upstream connections to the original source address determined
/// for the connection. This address could come from something like the Proxy Protocol filter, or it
/// could come from trusted http headers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OriginalSrc {
    /// Whether to bind the port to the one used in the original downstream connection.
    /// \[#not-implemented-hide:\]
    #[prost(bool, tag="1")]
    pub bind_port: bool,
    /// Sets the SO_MARK option on the upstream connection's socket to the provided value. Used to
    /// ensure that non-local addresses may be routed back through envoy when binding to the original
    /// source address. The option will not be applied if the mark is 0.
    #[prost(uint32, tag="2")]
    pub mark: u32,
}
/// Encoded file descriptor set for the `envoy.config.filter.listener.original_src.v2alpha1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xab, 0x0c, 0x0a, 0x45, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65,
    0x72, 0x2f, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x73, 0x72, 0x63, 0x2f, 0x76,
    0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x2f, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c,
    0x5f, 0x73, 0x72, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x32, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e,
    0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61,
    0x6c, 0x5f, 0x73, 0x72, 0x63, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x1a, 0x1e,
    0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d,
    0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3e, 0x0a,
    0x0b, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x53, 0x72, 0x63, 0x12, 0x1b, 0x0a, 0x09,
    0x62, 0x69, 0x6e, 0x64, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x08, 0x62, 0x69, 0x6e, 0x64, 0x50, 0x6f, 0x72, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6d, 0x61, 0x72,
    0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x6d, 0x61, 0x72, 0x6b, 0x42, 0xf2, 0x01,
    0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x33, 0x12, 0x31, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x61, 0x6c, 0x5f, 0x73, 0x72, 0x63, 0x2e, 0x76, 0x33, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10,
    0x01, 0x0a, 0x40, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x6f, 0x72,
    0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x73, 0x72, 0x63, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0x31, 0x42, 0x10, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x53, 0x72, 0x63,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x59, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e,
    0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67,
    0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x2f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2f, 0x6f, 0x72, 0x69,
    0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x73, 0x72, 0x63, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0x31, 0x4a, 0xb1, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1f, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00,
    0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00,
    0x59, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x07, 0x00, 0x59, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x08, 0x00, 0x31, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x08, 0x00,
    0x31, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x0a, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x70,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0a, 0x00, 0x70, 0x0a, 0x09, 0x0a, 0x01, 0x08,
    0x12, 0x04, 0x0b, 0x00, 0x0c, 0x38, 0x0a, 0x0e, 0x0a, 0x06, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02,
    0x12, 0x04, 0x0b, 0x00, 0x0c, 0x38, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a,
    0x8c, 0x03, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x16, 0x00, 0x1f, 0x01, 0x1a, 0xe7, 0x01, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x53, 0x72, 0x63,
    0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x62, 0x69, 0x6e, 0x64, 0x73, 0x20, 0x75, 0x70,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x61, 0x6c, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x64, 0x0a, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x63,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x63, 0x6f, 0x6d, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x73,
    0x6f, 0x6d, 0x65, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x50, 0x72, 0x6f, 0x78, 0x79, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x74, 0x0a, 0x20,
    0x63, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x63, 0x6f, 0x6d, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x74, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64, 0x20, 0x68, 0x74, 0x74, 0x70, 0x20, 0x68, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x32, 0x95, 0x01, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x4f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x61, 0x6c, 0x20, 0x53, 0x72, 0x63, 0x20, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x5d,
    0x0a, 0x20, 0x55, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x61, 0x6c, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x20, 0x6f, 0x6e, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x63, 0x6f,
    0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e,
    0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x73, 0x72, 0x63, 0x5d, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x16, 0x08, 0x13, 0x0a, 0x78, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x15, 0x1a, 0x6b, 0x20, 0x57, 0x68, 0x65, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x69, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x6f,
    0x72, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x61, 0x6c, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x63, 0x6f,
    0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x6e, 0x6f, 0x74,
    0x2d, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x2d, 0x68, 0x69, 0x64,
    0x65, 0x3a, 0x5d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x19,
    0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x07, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x13, 0x14, 0x0a, 0x8b,
    0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x02, 0x12, 0x1a, 0xfd, 0x01, 0x20,
    0x53, 0x65, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x4f, 0x5f, 0x4d, 0x41, 0x52, 0x4b,
    0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75,
    0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x27, 0x73, 0x20, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x2e, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x65, 0x6e, 0x73, 0x75,
    0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x6c, 0x6f, 0x63, 0x61,
    0x6c, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x20, 0x6d, 0x61, 0x79, 0x20,
    0x62, 0x65, 0x20, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x64, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x20, 0x74,
    0x68, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77, 0x68, 0x65,
    0x6e, 0x20, 0x62, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x0a, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62,
    0x65, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6d, 0x61, 0x72, 0x6b, 0x20, 0x69, 0x73, 0x20, 0x30, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x1e, 0x10, 0x11, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.config.filter.listener.original_src.v2alpha1.serde.rs");
// @@protoc_insertion_point(module)