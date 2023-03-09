// @generated
/// \[#not-implemented-hide:\] Not configuration. Workaround c++ protobuf issue with importing
/// services: <https://github.com/google/protobuf/issues/4221>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtdsDummy {
}
/// RTDS resource type. This describes a layer in the runtime virtual filesystem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Runtime {
    /// Runtime resource name. This makes the Runtime a self-describing xDS
    /// resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub layer: ::core::option::Option<::pbjson_types::Struct>,
}
/// Encoded file descriptor set for the `envoy.service.runtime.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xbd, 0x12, 0x0a, 0x23, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2f, 0x72, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x72, 0x74,
    0x64, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x18, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x72, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x2e,
    0x76, 0x33, 0x1a, 0x2a, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x2f, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x2f, 0x76, 0x33, 0x2f, 0x64,
    0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c,
    0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x73, 0x74,
    0x72, 0x75, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x72, 0x65,
    0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64,
    0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x21, 0x75, 0x64, 0x70,
    0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x38, 0x0a, 0x09, 0x52, 0x74, 0x64, 0x73, 0x44,
    0x75, 0x6d, 0x6d, 0x79, 0x3a, 0x2b, 0x9a, 0xc5, 0x88, 0x1e, 0x26, 0x0a, 0x24, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x64, 0x69, 0x73, 0x63, 0x6f,
    0x76, 0x65, 0x72, 0x79, 0x2e, 0x76, 0x32, 0x2e, 0x52, 0x74, 0x64, 0x73, 0x44, 0x75, 0x6d, 0x6d,
    0x79, 0x22, 0x80, 0x01, 0x0a, 0x07, 0x52, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x12, 0x1b, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07, 0xfa, 0x42, 0x04,
    0x72, 0x02, 0x10, 0x01, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2d, 0x0a, 0x05, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75,
    0x63, 0x74, 0x52, 0x05, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x3a, 0x29, 0x9a, 0xc5, 0x88, 0x1e, 0x24,
    0x0a, 0x22, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x2e, 0x76, 0x32, 0x2e, 0x52, 0x75, 0x6e,
    0x74, 0x69, 0x6d, 0x65, 0x32, 0xc4, 0x03, 0x0a, 0x17, 0x52, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65,
    0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x12, 0x72, 0x0a, 0x0d, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x52, 0x75, 0x6e, 0x74, 0x69, 0x6d,
    0x65, 0x12, 0x2c, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x2e, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x2e, 0x76, 0x33, 0x2e, 0x44,
    0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
    0x2d, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x2e, 0x76, 0x33, 0x2e, 0x44, 0x69, 0x73,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00,
    0x28, 0x01, 0x30, 0x01, 0x12, 0x7b, 0x0a, 0x0c, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x52, 0x75, 0x6e,
    0x74, 0x69, 0x6d, 0x65, 0x12, 0x31, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72,
    0x76, 0x69, 0x63, 0x65, 0x2e, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x2e, 0x76,
    0x33, 0x2e, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x32, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72,
    0x79, 0x2e, 0x76, 0x33, 0x2e, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x76,
    0x65, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x28, 0x01, 0x30,
    0x01, 0x12, 0x8d, 0x01, 0x0a, 0x0c, 0x46, 0x65, 0x74, 0x63, 0x68, 0x52, 0x75, 0x6e, 0x74, 0x69,
    0x6d, 0x65, 0x12, 0x2c, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2e, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x2e, 0x76, 0x33, 0x2e,
    0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x2d, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x2e, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x2e, 0x76, 0x33, 0x2e, 0x44, 0x69,
    0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
    0x20, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x1a, 0x3a, 0x01, 0x2a, 0x22, 0x15, 0x2f, 0x76, 0x33, 0x2f,
    0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x3a, 0x72, 0x75, 0x6e, 0x74, 0x69, 0x6d,
    0x65, 0x1a, 0x28, 0x8a, 0xa4, 0x96, 0xf3, 0x07, 0x22, 0x0a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x72, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65,
    0x2e, 0x76, 0x33, 0x2e, 0x52, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x42, 0x88, 0x01, 0xba, 0x80,
    0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x26, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x2e, 0x72, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x76, 0x33, 0x42, 0x09,
    0x52, 0x74, 0x64, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x49, 0x67, 0x69, 0x74,
    0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f,
    0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c,
    0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x2f, 0x72, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x2f, 0x76, 0x33, 0x3b, 0x72, 0x75, 0x6e,
    0x74, 0x69, 0x6d, 0x65, 0x76, 0x33, 0x4a, 0xff, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x39,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x02, 0x00, 0x21, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x34,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x07, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x00,
    0x2a, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0a, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x0b, 0x00, 0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0c,
    0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x3f, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x0e, 0x00, 0x3f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00,
    0x2a, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x2a, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x10, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x10, 0x00,
    0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x11, 0x00, 0x60, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x0b, 0x12, 0x03, 0x11, 0x00, 0x60, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x12, 0x00, 0x46,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x12, 0x00, 0x46, 0x0a,
    0xa7, 0x01, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x18, 0x00, 0x27, 0x01, 0x1a, 0x2a, 0x20, 0x44,
    0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x52, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x72, 0x65, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x2e, 0x0a, 0x32, 0x6f, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x52, 0x75, 0x6e,
    0x74, 0x69, 0x6d, 0x65, 0x20, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x28, 0x52, 0x54, 0x44, 0x53, 0x29, 0x5d, 0x0a, 0x20,
    0x52, 0x54, 0x44, 0x53, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65,
    0x77, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x72, 0x75, 0x6e, 0x74, 0x69, 0x6d,
    0x65, 0x5f, 0x72, 0x74, 0x64, 0x73, 0x3e, 0x60, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01,
    0x12, 0x03, 0x18, 0x08, 0x1f, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x03, 0x12, 0x03, 0x19, 0x02,
    0x50, 0x0a, 0x0f, 0x0a, 0x08, 0x06, 0x00, 0x03, 0xc1, 0xe4, 0xb2, 0x7e, 0x01, 0x12, 0x03, 0x19,
    0x02, 0x50, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x1b, 0x02, 0x1d, 0x03,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x06, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1b, 0x1b, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x1c, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1c, 0x16, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x04,
    0x1f, 0x02, 0x21, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1f,
    0x06, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1f, 0x13, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1f, 0x1a, 0x3c, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x20, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x20, 0x16, 0x39, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x02, 0x12, 0x04, 0x23, 0x02, 0x26, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x23, 0x06, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x23, 0x13, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x23,
    0x3b, 0x59, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x24, 0x04, 0x3c,
    0x0a, 0x11, 0x0a, 0x0a, 0x06, 0x00, 0x02, 0x02, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x04, 0x12, 0x03,
    0x24, 0x04, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x25, 0x04,
    0x28, 0x0a, 0x11, 0x0a, 0x0a, 0x06, 0x00, 0x02, 0x02, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x07, 0x12,
    0x03, 0x25, 0x04, 0x28, 0x0a, 0xa1, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x2b, 0x00, 0x2e,
    0x01, 0x1a, 0x94, 0x01, 0x20, 0x5b, 0x23, 0x6e, 0x6f, 0x74, 0x2d, 0x69, 0x6d, 0x70, 0x6c, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x2d, 0x68, 0x69, 0x64, 0x65, 0x3a, 0x5d, 0x20, 0x4e, 0x6f,
    0x74, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x20, 0x57, 0x6f, 0x72, 0x6b, 0x61, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x63, 0x2b, 0x2b, 0x20,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x20, 0x69, 0x73, 0x73, 0x75, 0x65, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x3a, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f,
    0x2f, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x69, 0x73, 0x73, 0x75,
    0x65, 0x73, 0x2f, 0x34, 0x32, 0x32, 0x31, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x2b, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x2c, 0x02, 0x2d,
    0x2d, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x2c,
    0x02, 0x2d, 0x2d, 0x0a, 0x5b, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x31, 0x00, 0x39, 0x01, 0x1a,
    0x4f, 0x20, 0x52, 0x54, 0x44, 0x53, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x76, 0x69, 0x72, 0x74,
    0x75, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x2e, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x31, 0x08, 0x0f, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x07, 0x12, 0x03, 0x32, 0x02, 0x64, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x07, 0xd3,
    0x88, 0xe1, 0x03, 0x01, 0x12, 0x03, 0x32, 0x02, 0x64, 0x0a, 0x5d, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x36, 0x02, 0x3b, 0x1a, 0x50, 0x20, 0x52, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65,
    0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x2e, 0x20,
    0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x61, 0x6b, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x52,
    0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x61, 0x20, 0x73, 0x65, 0x6c, 0x66, 0x2d, 0x64, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x62, 0x69, 0x6e, 0x67, 0x20, 0x78, 0x44, 0x53, 0x0a, 0x20, 0x72, 0x65,
    0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x36, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x36, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36,
    0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x08, 0x12, 0x03, 0x36, 0x12, 0x3a,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x36, 0x13,
    0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x38, 0x02, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x38, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x38, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x38, 0x21, 0x22, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.service.runtime.v3.serde.rs");
include!("envoy.service.runtime.v3.tonic.rs");
// @@protoc_insertion_point(module)