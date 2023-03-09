// @generated
// [#protodoc-title: TLS Inspector Filter]
// Allows detecting whether the transport appears to be TLS or plaintext.
// [#extension: envoy.filters.listener.tls_inspector]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsInspector {
}
/// Encoded file descriptor set for the `envoy.config.filter.listener.tls_inspector.v2` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xc4, 0x06, 0x0a, 0x41, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65,
    0x72, 0x2f, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2f,
    0x76, 0x32, 0x2f, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6c, 0x69, 0x73, 0x74,
    0x65, 0x6e, 0x65, 0x72, 0x2e, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x2e, 0x76, 0x32, 0x1a, 0x1e, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x0e, 0x0a, 0x0c, 0x54, 0x6c, 0x73, 0x49, 0x6e, 0x73, 0x70, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x42, 0xfa, 0x01, 0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x34, 0x12, 0x32, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72,
    0x2e, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x76,
    0x33, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x01, 0x0a, 0x3b, 0x69, 0x6f, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6c, 0x69, 0x73,
    0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x2e, 0x76, 0x32, 0x42, 0x11, 0x54, 0x6c, 0x73, 0x49, 0x6e, 0x73, 0x70, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x64, 0x67, 0x69, 0x74,
    0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f,
    0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c,
    0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72,
    0x2f, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2f, 0x76,
    0x32, 0x3b, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x76,
    0x32, 0x4a, 0xfb, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x14, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x36,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x05, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x54,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x07, 0x00, 0x54, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x08, 0x00, 0x32, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x08, 0x00, 0x32,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a,
    0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x7b, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0a, 0x00, 0x7b, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12,
    0x04, 0x0b, 0x00, 0x0c, 0x39, 0x0a, 0x0e, 0x0a, 0x06, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12,
    0x04, 0x0b, 0x00, 0x0c, 0x39, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0xb2,
    0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x13, 0x00, 0x14, 0x01, 0x32, 0xa5, 0x01, 0x20, 0x5b,
    0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a,
    0x20, 0x54, 0x4c, 0x53, 0x20, 0x49, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x46,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x5d, 0x0a, 0x20, 0x41, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x64,
    0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x61,
    0x70, 0x70, 0x65, 0x61, 0x72, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x54, 0x4c, 0x53,
    0x20, 0x6f, 0x72, 0x20, 0x70, 0x6c, 0x61, 0x69, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x2e, 0x0a, 0x20,
    0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65,
    0x6e, 0x65, 0x72, 0x2e, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x13, 0x08, 0x14, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)