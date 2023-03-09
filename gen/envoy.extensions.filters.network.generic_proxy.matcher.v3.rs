// @generated
// [#protodoc-title: Generic Proxy Route Matcher Configuration]

/// Used to match request service of the downstream request. Only applicable if a service provided
/// by the application protocol.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceMatchInput {
}
/// Used to match request method of the downstream request. Only applicable if a method provided
/// by the application protocol.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodMatchInput {
}
/// Used to match an arbitrary property of the downstream request.
/// These properties are populated by the codecs of application protocols.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyMatchInput {
    /// The property name to match on.
    #[prost(string, tag="1")]
    pub property_name: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `envoy.extensions.filters.network.generic_proxy.matcher.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x86, 0x0b, 0x0a, 0x4f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x67, 0x65,
    0x6e, 0x65, 0x72, 0x69, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x6d, 0x61, 0x74, 0x63,
    0x68, 0x65, 0x72, 0x2f, 0x76, 0x33, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x39, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x5f, 0x70,
    0x72, 0x6f, 0x78, 0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x76, 0x33, 0x1a,
    0x1f, 0x78, 0x64, 0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x76, 0x33, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x13, 0x0a, 0x11, 0x53, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x22, 0x12, 0x0a,
    0x10, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x22, 0x42, 0x0a, 0x12, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x4d, 0x61, 0x74,
    0x63, 0x68, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x12, 0x2c, 0x0a, 0x0d, 0x70, 0x72, 0x6f, 0x70, 0x65,
    0x72, 0x74, 0x79, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07,
    0xfa, 0x42, 0x04, 0x72, 0x02, 0x10, 0x01, 0x52, 0x0c, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74,
    0x79, 0x4e, 0x61, 0x6d, 0x65, 0x42, 0xd5, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02,
    0xd2, 0xc6, 0xa4, 0xe1, 0x06, 0x02, 0x08, 0x01, 0x0a, 0x47, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63,
    0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x76,
    0x33, 0x42, 0x0c, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50,
    0x01, 0x5a, 0x6a, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x67, 0x65, 0x6e, 0x65, 0x72,
    0x69, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72,
    0x2f, 0x76, 0x33, 0x3b, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x76, 0x33, 0x4a, 0xd1, 0x06,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x21, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x42, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06,
    0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x21, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x60, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09,
    0x00, 0x60, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x2d, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x2d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00,
    0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x01,
    0x08, 0x12, 0x04, 0x0c, 0x00, 0x81, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x04, 0x0c,
    0x00, 0x81, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0e, 0x00, 0x40, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xea, 0xc8, 0x94, 0x6c, 0x01,
    0x12, 0x03, 0x0e, 0x00, 0x40, 0x0a, 0xca, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x14, 0x00,
    0x15, 0x01, 0x1a, 0x7e, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x20, 0x4f, 0x6e,
    0x6c, 0x79, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x69, 0x66,
    0x20, 0x61, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69,
    0x64, 0x65, 0x64, 0x0a, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x70, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x2e, 0x0a, 0x32, 0x3e, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d,
    0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x20, 0x50,
    0x72, 0x6f, 0x78, 0x79, 0x20, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x20, 0x4d, 0x61, 0x74, 0x63, 0x68,
    0x65, 0x72, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x19, 0x0a, 0x88,
    0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x19, 0x00, 0x1a, 0x01, 0x1a, 0x7c, 0x20, 0x55, 0x73,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x2e, 0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x69, 0x66, 0x20, 0x61, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f,
    0x64, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x0a, 0x20, 0x62, 0x79, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x19, 0x08, 0x18, 0x0a, 0x95, 0x01, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1e, 0x00,
    0x21, 0x01, 0x1a, 0x88, 0x01, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79,
    0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x73, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x70,
    0x65, 0x72, 0x74, 0x69, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x70, 0x6f, 0x70, 0x75, 0x6c,
    0x61, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x64, 0x65,
    0x63, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x1a, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x20, 0x02, 0x44, 0x1a, 0x20, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f,
    0x70, 0x65, 0x72, 0x74, 0x79, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x20, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x20, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x20, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20,
    0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x08, 0x12, 0x03, 0x20, 0x1b, 0x43,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x20, 0x1c,
    0x42, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)