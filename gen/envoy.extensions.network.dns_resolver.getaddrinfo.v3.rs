// @generated
// [#protodoc-title: getaddrinfo DNS resolver]
// [#extension: envoy.network.dns_resolver.getaddrinfo]

/// Configuration for getaddrinfo DNS resolver. This resolver will use the system's getaddrinfo()
/// function to resolve hosts.
///
/// .. attention::
///
///    This resolver uses a single background thread to do resolutions. As such, it is not currently
///    advised for use in situations requiring a high resolution rate. A thread pool can be added
///    in the future if needed.
///
/// .. attention::
///
///    Resolutions currently use a hard coded TTL of 60s because the getaddrinfo() API does not
///    provide the actual TTL. Configuration for this can be added in the future if needed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddrInfoDnsResolverConfig {
}
/// Encoded file descriptor set for the `envoy.extensions.network.dns_resolver.getaddrinfo.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf3, 0x09, 0x0a, 0x53, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x64, 0x6e,
    0x73, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2f, 0x67, 0x65, 0x74, 0x61, 0x64,
    0x64, 0x72, 0x69, 0x6e, 0x66, 0x6f, 0x2f, 0x76, 0x33, 0x2f, 0x67, 0x65, 0x74, 0x61, 0x64, 0x64,
    0x72, 0x69, 0x6e, 0x66, 0x6f, 0x5f, 0x64, 0x6e, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76,
    0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x34, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x2e, 0x64, 0x6e, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2e,
    0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x76, 0x33, 0x1a, 0x1d,
    0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1e, 0x0a,
    0x1c, 0x47, 0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x44, 0x6e, 0x73, 0x52,
    0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x42, 0xd6, 0x01,
    0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x42, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x2e, 0x64, 0x6e, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2e, 0x67, 0x65,
    0x74, 0x61, 0x64, 0x64, 0x72, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x76, 0x33, 0x42, 0x1b, 0x47, 0x65,
    0x74, 0x61, 0x64, 0x64, 0x72, 0x69, 0x6e, 0x66, 0x6f, 0x44, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x6f,
    0x6c, 0x76, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x69, 0x67, 0x69, 0x74,
    0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f,
    0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c,
    0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x64, 0x6e, 0x73,
    0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2f, 0x67, 0x65, 0x74, 0x61, 0x64, 0x64,
    0x72, 0x69, 0x6e, 0x66, 0x6f, 0x2f, 0x76, 0x33, 0x3b, 0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72,
    0x69, 0x6e, 0x66, 0x6f, 0x76, 0x33, 0x4a, 0xc5, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1d,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x02, 0x00, 0x3d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x5b, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x06, 0x00, 0x5b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x3c, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x07, 0x00, 0x3c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x08, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x08, 0x00, 0x22, 0x0a,
    0x09, 0x0a, 0x01, 0x08, 0x12, 0x04, 0x09, 0x00, 0x80, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x08, 0x0b,
    0x12, 0x04, 0x09, 0x00, 0x80, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x46,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0a, 0x00, 0x46, 0x0a,
    0xa0, 0x05, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1c, 0x00, 0x1d, 0x01, 0x1a, 0xae, 0x04, 0x20,
    0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72, 0x69, 0x6e, 0x66, 0x6f, 0x20, 0x44, 0x4e,
    0x53, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73,
    0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x75,
    0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x27, 0x73, 0x20,
    0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72, 0x69, 0x6e, 0x66, 0x6f, 0x28, 0x29, 0x0a, 0x20, 0x66,
    0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c,
    0x76, 0x65, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x20, 0x2e, 0x2e, 0x20, 0x61,
    0x74, 0x74, 0x65, 0x6e, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x3a, 0x0a, 0x0a, 0x20, 0x20, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x20, 0x75, 0x73, 0x65,
    0x73, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x67,
    0x72, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x72, 0x65, 0x61, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x64, 0x6f, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x20,
    0x41, 0x73, 0x20, 0x73, 0x75, 0x63, 0x68, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x6e,
    0x6f, 0x74, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x0a, 0x20, 0x20, 0x20,
    0x61, 0x64, 0x76, 0x69, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x75, 0x73, 0x65, 0x20,
    0x69, 0x6e, 0x20, 0x73, 0x69, 0x74, 0x75, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x69, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x68, 0x69, 0x67, 0x68, 0x20, 0x72,
    0x65, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x72, 0x61, 0x74, 0x65, 0x2e, 0x20,
    0x41, 0x20, 0x74, 0x68, 0x72, 0x65, 0x61, 0x64, 0x20, 0x70, 0x6f, 0x6f, 0x6c, 0x20, 0x63, 0x61,
    0x6e, 0x20, 0x62, 0x65, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x20, 0x69, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x20, 0x69, 0x66, 0x20, 0x6e,
    0x65, 0x65, 0x64, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x20, 0x2e, 0x2e, 0x20, 0x61, 0x74, 0x74, 0x65,
    0x6e, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x3a, 0x0a, 0x0a, 0x20, 0x20, 0x20, 0x52, 0x65, 0x73, 0x6f,
    0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c,
    0x79, 0x20, 0x75, 0x73, 0x65, 0x20, 0x61, 0x20, 0x68, 0x61, 0x72, 0x64, 0x20, 0x63, 0x6f, 0x64,
    0x65, 0x64, 0x20, 0x54, 0x54, 0x4c, 0x20, 0x6f, 0x66, 0x20, 0x36, 0x30, 0x73, 0x20, 0x62, 0x65,
    0x63, 0x61, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x65, 0x74, 0x61, 0x64, 0x64,
    0x72, 0x69, 0x6e, 0x66, 0x6f, 0x28, 0x29, 0x20, 0x41, 0x50, 0x49, 0x20, 0x64, 0x6f, 0x65, 0x73,
    0x20, 0x6e, 0x6f, 0x74, 0x0a, 0x20, 0x20, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x54, 0x54, 0x4c, 0x2e, 0x20,
    0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x61, 0x64,
    0x64, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x75, 0x74, 0x75, 0x72,
    0x65, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x65, 0x64, 0x2e, 0x0a, 0x32, 0x63, 0x20,
    0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65,
    0x3a, 0x20, 0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72, 0x69, 0x6e, 0x66, 0x6f, 0x20, 0x44, 0x4e,
    0x53, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x64, 0x6e, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x6f,
    0x6c, 0x76, 0x65, 0x72, 0x2e, 0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72, 0x69, 0x6e, 0x66, 0x6f,
    0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x24, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)