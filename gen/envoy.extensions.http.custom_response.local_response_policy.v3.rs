// @generated
// [#protodoc-title: Local Response Policy for Custom Response]
// [#extension: envoy.http.custom_response.local_response_policy]

/// Custom response policy to serve a locally stored response to the
/// downstream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalResponsePolicy {
    /// Optional new local reply body text. It will be used
    /// in the `%LOCAL_REPLY_BODY%` command operator in the `body_format`.
    #[prost(message, optional, tag="1")]
    pub body: ::core::option::Option<super::super::super::super::super::config::core::v3::DataSource>,
    /// Optional body format to be used for this response. If `body_format` is  not
    /// provided, and `body` is, the contents of `body` will be used to populate
    /// the body of the local reply without formatting.
    #[prost(message, optional, tag="2")]
    pub body_format: ::core::option::Option<super::super::super::super::super::config::core::v3::SubstitutionFormatString>,
    /// The new response status code if specified.
    #[prost(message, optional, tag="3")]
    pub status_code: ::core::option::Option<::pbjson_types::UInt32Value>,
    /// HTTP headers to add to the response. This allows the
    /// response policy to append, to add or to override headers of
    /// the original response for local body, or the custom response from the
    /// remote body, before it is sent to a downstream client.
    #[prost(message, repeated, tag="4")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<super::super::super::super::super::config::core::v3::HeaderValueOption>,
}
/// Encoded file descriptor set for the `envoy.extensions.http.custom_response.local_response_policy.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xaf, 0x12, 0x0a, 0x5a, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x63, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2f, 0x6c, 0x6f, 0x63, 0x61, 0x6c,
    0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79,
    0x2f, 0x76, 0x33, 0x2f, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x3e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x2e, 0x76, 0x33, 0x1a,
    0x1f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f,
    0x72, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x35, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63,
    0x6f, 0x72, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x73, 0x75, 0x62, 0x73, 0x74, 0x69, 0x74, 0x75, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x5f, 0x73, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x78, 0x64, 0x73, 0x2f, 0x61, 0x6e, 0x6e,
    0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x33, 0x2f, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0xd3, 0x02, 0x0a, 0x13, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x12, 0x34, 0x0a, 0x04, 0x62, 0x6f, 0x64, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x44, 0x61,
    0x74, 0x61, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x04, 0x62, 0x6f, 0x64, 0x79, 0x12, 0x4f,
    0x0a, 0x0b, 0x62, 0x6f, 0x64, 0x79, 0x5f, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x53, 0x75, 0x62, 0x73, 0x74,
    0x69, 0x74, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x53, 0x74, 0x72,
    0x69, 0x6e, 0x67, 0x52, 0x0a, 0x62, 0x6f, 0x64, 0x79, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x12,
    0x4a, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x55, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x42, 0x0b, 0xfa, 0x42, 0x08, 0x2a, 0x06, 0x10, 0xd8, 0x04, 0x28, 0xc8, 0x01, 0x52,
    0x0a, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x69, 0x0a, 0x17, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x5f,
    0x74, 0x6f, 0x5f, 0x61, 0x64, 0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65,
    0x2e, 0x76, 0x33, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x4f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x09, 0xfa, 0x42, 0x06, 0x92, 0x01, 0x03, 0x10, 0xe8, 0x07,
    0x52, 0x14, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x73, 0x54, 0x6f, 0x41, 0x64, 0x64, 0x42, 0xf9, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10,
    0x02, 0xd2, 0xc6, 0xa4, 0xe1, 0x06, 0x02, 0x08, 0x01, 0x0a, 0x4c, 0x69, 0x6f, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x63,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x6c,
    0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x70, 0x6f,
    0x6c, 0x69, 0x63, 0x79, 0x2e, 0x76, 0x33, 0x42, 0x18, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x50, 0x01, 0x5a, 0x7d, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x68, 0x74, 0x74,
    0x70, 0x2f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x2f, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x2f, 0x76, 0x33, 0x3b, 0x6c, 0x6f, 0x63, 0x61, 0x6c,
    0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79,
    0x76, 0x33, 0x4a, 0xe5, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2d, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00,
    0x47, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x3f, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07,
    0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x00, 0x29, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x0b, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x0c, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x65, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x0e, 0x00, 0x65, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f,
    0x00, 0x39, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x39, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x10, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x10,
    0x00, 0x22, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04, 0x11, 0x00, 0x94, 0x01, 0x0a, 0x0a, 0x0a,
    0x02, 0x08, 0x0b, 0x12, 0x04, 0x11, 0x00, 0x94, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x12, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x12,
    0x00, 0x46, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x13, 0x00, 0x40, 0x0a, 0x0d, 0x0a, 0x06,
    0x08, 0xea, 0xc8, 0x94, 0x6c, 0x01, 0x12, 0x03, 0x13, 0x00, 0x40, 0x0a, 0xdb, 0x01, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x1a, 0x00, 0x2d, 0x01, 0x1a, 0x4f, 0x20, 0x43, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63,
    0x79, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x20, 0x61, 0x20, 0x6c, 0x6f, 0x63,
    0x61, 0x6c, 0x6c, 0x79, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x64, 0x6f, 0x77,
    0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x0a, 0x32, 0x7e, 0x20, 0x5b, 0x23, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x4c, 0x6f,
    0x63, 0x61, 0x6c, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x50, 0x6f, 0x6c,
    0x69, 0x63, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x68, 0x74, 0x74,
    0x70, 0x2e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x2e, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x1a, 0x08, 0x1b, 0x0a, 0x86, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x1d, 0x02, 0x25, 0x1a, 0x79, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x6e,
    0x65, 0x77, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x20, 0x62,
    0x6f, 0x64, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x60, 0x25, 0x4c, 0x4f, 0x43, 0x41, 0x4c, 0x5f, 0x52, 0x45, 0x50, 0x4c, 0x59, 0x5f,
    0x42, 0x4f, 0x44, 0x59, 0x25, 0x60, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x20, 0x6f,
    0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x60,
    0x62, 0x6f, 0x64, 0x79, 0x5f, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x60, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1d, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x1c, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x23, 0x24, 0x0a, 0xd6, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x22, 0x02, 0x3a, 0x1a, 0xc8, 0x01, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x61, 0x6c, 0x20, 0x62, 0x6f, 0x64, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x74,
    0x6f, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x20, 0x49, 0x66, 0x20,
    0x60, 0x62, 0x6f, 0x64, 0x79, 0x5f, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x60, 0x20, 0x69, 0x73,
    0x20, 0x20, 0x6e, 0x6f, 0x74, 0x0a, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x2c,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x60, 0x62, 0x6f, 0x64, 0x79, 0x60, 0x20, 0x69, 0x73, 0x2c, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x6f, 0x66, 0x20,
    0x60, 0x62, 0x6f, 0x64, 0x79, 0x60, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x0a,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6f, 0x64, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x20, 0x77, 0x69, 0x74,
    0x68, 0x6f, 0x75, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x22, 0x02, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x2a, 0x35, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x38, 0x39, 0x0a, 0x39, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x25, 0x02, 0x5d, 0x1a, 0x2c, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e,
    0x65, 0x77, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x20, 0x69, 0x66, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69,
    0x66, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x25, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x25,
    0x1e, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x25, 0x2c, 0x2d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x25, 0x2e, 0x5c, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x02, 0x08, 0xaf, 0x08, 0x05, 0x12, 0x03, 0x25, 0x2f, 0x5b, 0x0a,
    0x81, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x2b, 0x02, 0x2c, 0x36, 0x1a, 0xf2,
    0x01, 0x20, 0x48, 0x54, 0x54, 0x50, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x20, 0x74,
    0x6f, 0x20, 0x61, 0x64, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x6f,
    0x77, 0x73, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x70, 0x70, 0x65, 0x6e,
    0x64, 0x2c, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x64, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x74, 0x6f, 0x20,
    0x6f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73,
    0x20, 0x6f, 0x66, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61,
    0x6c, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6c,
    0x6f, 0x63, 0x61, 0x6c, 0x20, 0x62, 0x6f, 0x64, 0x79, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x72, 0x65, 0x6d, 0x6f,
    0x74, 0x65, 0x20, 0x62, 0x6f, 0x64, 0x79, 0x2c, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20,
    0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20,
    0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2b, 0x0b, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2b, 0x2c, 0x43, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2b, 0x46, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x08, 0x12, 0x03, 0x2c, 0x06, 0x35, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02,
    0x03, 0x08, 0xaf, 0x08, 0x12, 0x12, 0x03, 0x2c, 0x07, 0x34, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33,
];
include!("envoy.extensions.http.custom_response.local_response_policy.v3.serde.rs");
// @@protoc_insertion_point(module)