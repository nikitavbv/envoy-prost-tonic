// @generated
// [#protodoc-title: Direct response]
// Direct response :ref:`configuration overview <config_network_filters_direct_response>`.
// [#extension: envoy.filters.network.direct_response]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// Response data as a data source.
    #[prost(message, optional, tag="1")]
    pub response: ::core::option::Option<super::super::super::super::super::api::v2::core::DataSource>,
}
/// Encoded file descriptor set for the `envoy.config.filter.network.direct_response.v2` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x83, 0x08, 0x0a, 0x3b, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x2f, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2f, 0x76, 0x32, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x64, 0x69,
    0x72, 0x65, 0x63, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x76, 0x32,
    0x1a, 0x1c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x32, 0x2f, 0x63,
    0x6f, 0x72, 0x65, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e,
    0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d,
    0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x43, 0x0a,
    0x06, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x39, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x44, 0x61,
    0x74, 0x61, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x42, 0xf9, 0x01, 0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x35, 0x12, 0x33, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x64, 0x69,
    0x72, 0x65, 0x63, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x76, 0x33,
    0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x01, 0x0a, 0x3c, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6e, 0x65, 0x74, 0x77,
    0x6f, 0x72, 0x6b, 0x2e, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x2e, 0x76, 0x32, 0x42, 0x0b, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x67, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
    0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2f, 0x76, 0x32, 0x3b, 0x64, 0x69, 0x72,
    0x65, 0x63, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x76, 0x32, 0x4a, 0xed,
    0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x18, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x37, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x06, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x27, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x55, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x09, 0x00, 0x55, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x2c, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x2c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b,
    0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x7e, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0c,
    0x00, 0x7e, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04, 0x0d, 0x00, 0x0e, 0x3a, 0x0a, 0x0e, 0x0a,
    0x06, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12, 0x04, 0x0d, 0x00, 0x0e, 0x3a, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a,
    0x02, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0xbf, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x15,
    0x00, 0x18, 0x01, 0x32, 0xb2, 0x01, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f,
    0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x20,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x0a, 0x20, 0x44, 0x69, 0x72, 0x65, 0x63,
    0x74, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a,
    0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f,
    0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f,
    0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x3e,
    0x60, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a,
    0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x5f, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x15, 0x08, 0x0e, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x17, 0x02,
    0x26, 0x1a, 0x21, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x64, 0x61, 0x74,
    0x61, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x17,
    0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x19, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x24, 0x25, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.config.filter.network.direct_response.v2.serde.rs");
// @@protoc_insertion_point(module)