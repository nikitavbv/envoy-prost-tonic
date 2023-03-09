// @generated
// [#protodoc-title: TLS Inspector Filter]
// Allows detecting whether the transport appears to be TLS or plaintext.
// [#extension: envoy.filters.listener.tls_inspector]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsInspector {
    /// Populate ``JA3`` fingerprint hash using data from the TLS Client Hello packet. Default is false.
    #[prost(message, optional, tag="1")]
    pub enable_ja3_fingerprinting: ::core::option::Option<::pbjson_types::BoolValue>,
}
/// Encoded file descriptor set for the `envoy.extensions.filters.listener.tls_inspector.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x89, 0x09, 0x0a, 0x46, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x6c, 0x69,
    0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2f, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x2f, 0x76, 0x33, 0x2f, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x32, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x74,
    0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x76, 0x33, 0x1a,
    0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
    0x2f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x21,
    0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0xa9, 0x01, 0x0a, 0x0c, 0x54, 0x6c, 0x73, 0x49, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x12, 0x56, 0x0a, 0x19, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x6a, 0x61, 0x33,
    0x5f, 0x66, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x42, 0x6f, 0x6f, 0x6c, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x52, 0x17, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x4a, 0x61, 0x33, 0x46, 0x69, 0x6e, 0x67,
    0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x3a, 0x41, 0x9a, 0xc5, 0x88, 0x1e,
    0x3c, 0x0a, 0x3a, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e,
    0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x76, 0x32,
    0x2e, 0x54, 0x6c, 0x73, 0x49, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x42, 0xca, 0x01,
    0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x40, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e,
    0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x76, 0x33, 0x42, 0x11, 0x54, 0x6c, 0x73, 0x49,
    0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a,
    0x69, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f,
    0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x2f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2f, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e,
    0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2f, 0x76, 0x33, 0x3b, 0x74, 0x6c, 0x73, 0x5f, 0x69,
    0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x76, 0x33, 0x4a, 0xa7, 0x04, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x19, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x04, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x27, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x2b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x09, 0x00, 0x59, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09, 0x00, 0x59, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x32, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12,
    0x03, 0x0a, 0x00, 0x32, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04,
    0x0c, 0x00, 0x80, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x04, 0x0c, 0x00, 0x80, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87,
    0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0xb2, 0x01, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x13, 0x00, 0x19, 0x01, 0x32, 0xa5, 0x01, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x54, 0x4c, 0x53, 0x20,
    0x49, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x5d, 0x0a, 0x20, 0x41, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x54, 0x4c, 0x53, 0x20, 0x6f, 0x72, 0x20, 0x70,
    0x6c, 0x61, 0x69, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x74,
    0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x5d, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x13, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00,
    0x07, 0x12, 0x04, 0x14, 0x02, 0x15, 0x43, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x00, 0x07, 0xd3, 0x88,
    0xe1, 0x03, 0x01, 0x12, 0x04, 0x14, 0x02, 0x15, 0x43, 0x0a, 0x6f, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x18, 0x02, 0x3a, 0x1a, 0x62, 0x20, 0x50, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x74,
    0x65, 0x20, 0x60, 0x60, 0x4a, 0x41, 0x33, 0x60, 0x60, 0x20, 0x66, 0x69, 0x6e, 0x67, 0x65, 0x72,
    0x70, 0x72, 0x69, 0x6e, 0x74, 0x20, 0x68, 0x61, 0x73, 0x68, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67,
    0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x54,
    0x4c, 0x53, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20,
    0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20,
    0x69, 0x73, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x18, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x18, 0x1c, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x18, 0x38, 0x39, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.listener.tls_inspector.v3.serde.rs");
// @@protoc_insertion_point(module)