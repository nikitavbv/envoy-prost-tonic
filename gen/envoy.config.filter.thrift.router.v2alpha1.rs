// @generated
// [#protodoc-title: Router]
// Thrift router :ref:`configuration overview <config_thrift_filters_router>`.
// [#extension: envoy.filters.thrift.router]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Router {
}
/// Encoded file descriptor set for the `envoy.config.filter.thrift.router.v2alpha1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x83, 0x05, 0x0a, 0x37, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x2f,
    0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x2f,
    0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2a, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x2e, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x2e, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x2e,
    0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x08, 0x0a, 0x06, 0x52, 0x6f, 0x75, 0x74, 0x65,
    0x72, 0x42, 0xa4, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x38, 0x69, 0x6f,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e,
    0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x2e, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x2e, 0x76, 0x32,
    0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x42, 0x0b, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x51, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
    0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x2f, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x2f, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x2f,
    0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x4a, 0xc3, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x11, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x33, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04,
    0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x51, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x06, 0x00, 0x51, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00,
    0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x07, 0x00, 0x2c, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x08, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x08, 0x00,
    0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x68, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x0b, 0x12, 0x03, 0x09, 0x00, 0x68, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x46,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0a, 0x00, 0x46, 0x0a,
    0xa0, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x10, 0x00, 0x11, 0x01, 0x32, 0x93, 0x01, 0x20,
    0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65,
    0x3a, 0x20, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x5d, 0x0a, 0x20, 0x54, 0x68, 0x72, 0x69, 0x66,
    0x74, 0x20, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65,
    0x72, 0x76, 0x69, 0x65, 0x77, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x74, 0x68,
    0x72, 0x69, 0x66, 0x74, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x72, 0x6f, 0x75,
    0x74, 0x65, 0x72, 0x3e, 0x60, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x73, 0x2e, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x2e, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72,
    0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0e, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.config.filter.thrift.router.v2alpha1.serde.rs");
// @@protoc_insertion_point(module)