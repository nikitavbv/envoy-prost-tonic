// @generated
// [#protodoc-title: Tap]
// [#extension: envoy.transport_sockets.tap]

/// Configuration for tap transport socket. This wraps another transport socket, providing the
/// ability to interpose and record in plain text any traffic that is surfaced to Envoy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tap {
    /// Common configuration for the tap transport socket.
    #[prost(message, optional, tag="1")]
    pub common_config: ::core::option::Option<super::super::super::common::tap::v3::CommonExtensionConfig>,
    /// The underlying transport socket being wrapped.
    #[prost(message, optional, tag="2")]
    pub transport_socket: ::core::option::Option<super::super::super::super::config::core::v3::TransportSocket>,
}
/// Encoded file descriptor set for the `envoy.extensions.transport_sockets.tap.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xc3, 0x0b, 0x0a, 0x33, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x5f,
    0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x2f, 0x74, 0x61, 0x70, 0x2f, 0x76, 0x33, 0x2f, 0x74,
    0x61, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x29, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x70, 0x6f, 0x72, 0x74, 0x5f, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x2e, 0x74, 0x61, 0x70,
    0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x2b, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x74, 0x61,
    0x70, 0x2f, 0x76, 0x33, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x21, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xfd, 0x01, 0x0a,
    0x03, 0x54, 0x61, 0x70, 0x12, 0x64, 0x0a, 0x0d, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x35, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x63,
    0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x74, 0x61, 0x70, 0x2e, 0x76, 0x33, 0x2e, 0x43, 0x6f, 0x6d,
    0x6d, 0x6f, 0x6e, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x0c, 0x63, 0x6f,
    0x6d, 0x6d, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x5a, 0x0a, 0x10, 0x74, 0x72,
    0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x54, 0x72, 0x61, 0x6e,
    0x73, 0x70, 0x6f, 0x72, 0x74, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x42, 0x08, 0xfa, 0x42, 0x05,
    0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x0f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74,
    0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x3a, 0x34, 0x9a, 0xc5, 0x88, 0x1e, 0x2f, 0x0a, 0x2d, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e, 0x74, 0x61, 0x70,
    0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x54, 0x61, 0x70, 0x42, 0xa5, 0x01, 0xba,
    0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x37, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72,
    0x74, 0x5f, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x2e, 0x74, 0x61, 0x70, 0x2e, 0x76, 0x33,
    0x42, 0x08, 0x54, 0x61, 0x70, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x56, 0x67, 0x69,
    0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72,
    0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70,
    0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x5f,
    0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x2f, 0x74, 0x61, 0x70, 0x2f, 0x76, 0x33, 0x3b, 0x74,
    0x61, 0x70, 0x76, 0x33, 0x4a, 0x87, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x20, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x02, 0x00, 0x32, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x35, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x07, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08, 0x00, 0x2b, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x09, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0b, 0x00, 0x50, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0b, 0x00, 0x50, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12,
    0x03, 0x0c, 0x00, 0x29, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x22, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0d, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0e, 0x00, 0x6d, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0e, 0x00, 0x6d, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99,
    0x6a, 0x02, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0x84, 0x02, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x16, 0x00, 0x20, 0x01, 0x1a, 0xb2, 0x01, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x61, 0x70, 0x20, 0x74, 0x72,
    0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e, 0x20,
    0x54, 0x68, 0x69, 0x73, 0x20, 0x77, 0x72, 0x61, 0x70, 0x73, 0x20, 0x61, 0x6e, 0x6f, 0x74, 0x68,
    0x65, 0x72, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x73, 0x6f, 0x63,
    0x6b, 0x65, 0x74, 0x2c, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x68, 0x65, 0x0a, 0x20, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x69,
    0x6e, 0x74, 0x65, 0x72, 0x70, 0x6f, 0x73, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x63,
    0x6f, 0x72, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x70, 0x6c, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x65, 0x78,
    0x74, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x74, 0x72, 0x61, 0x66, 0x66, 0x69, 0x63, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x0a, 0x32, 0x43, 0x20, 0x5b, 0x23, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x54, 0x61,
    0x70, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a,
    0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74,
    0x5f, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x2e, 0x74, 0x61, 0x70, 0x5d, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x16, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00,
    0x07, 0x12, 0x04, 0x17, 0x02, 0x18, 0x36, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x00, 0x07, 0xd3, 0x88,
    0xe1, 0x03, 0x01, 0x12, 0x04, 0x17, 0x02, 0x18, 0x36, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x04, 0x1b, 0x02, 0x1c, 0x34, 0x1a, 0x34, 0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e,
    0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x70, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x70, 0x6f, 0x72, 0x74, 0x20, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1b, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x26, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1b, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x1c, 0x06, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08,
    0x11, 0x12, 0x03, 0x1c, 0x07, 0x32, 0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x1f, 0x02, 0x64, 0x1a, 0x30, 0x20, 0x54, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x6c,
    0x79, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x73,
    0x6f, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x72, 0x61, 0x70,
    0x70, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x1f, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x21,
    0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1f, 0x34, 0x35, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x1f, 0x36, 0x63, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x11, 0x12, 0x03, 0x1f, 0x37, 0x62, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.transport_sockets.tap.v3.serde.rs");
// @@protoc_insertion_point(module)