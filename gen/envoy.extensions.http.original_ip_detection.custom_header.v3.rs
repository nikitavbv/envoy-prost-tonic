// @generated
// [#protodoc-title: Custom header original IP detection extension]

/// This extension allows for the original downstream remote IP to be detected
/// by reading the value from a configured header name. If the value is successfully parsed
/// as an IP, it'll be treated as the effective downstream remote address and seen as such
/// by all filters. See :ref:`original_ip_detection_extensions
/// <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.original_ip_detection_extensions>`
/// for an overview of how extensions operate and what happens when an extension fails
/// to detect the remote IP.
///
/// [#extension: envoy.http.original_ip_detection.custom_header]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomHeaderConfig {
    /// The header name containing the original downstream remote address, if present.
    ///
    /// Note: in the case of a multi-valued header, only the first value is tried and the rest are ignored.
    #[prost(string, tag="1")]
    pub header_name: ::prost::alloc::string::String,
    /// If set to true, the extension could decide that the detected address should be treated as
    /// trusted by the HCM. If the address is considered :ref:`trusted<config_http_conn_man_headers_x-forwarded-for_trusted_client_address>`,
    /// it might be used as input to determine if the request is internal (among other things).
    #[prost(bool, tag="2")]
    pub allow_extension_to_set_address_as_trusted: bool,
    /// If this is set, the request will be rejected when detection fails using it as the HTTP response status.
    ///
    /// .. note::
    ///    If this is set to < 400 or > 511, the default status 403 will be used instead.
    #[prost(message, optional, tag="3")]
    pub reject_with_status: ::core::option::Option<super::super::super::super::super::r#type::v3::HttpStatus>,
}
/// Encoded file descriptor set for the `envoy.extensions.http.original_ip_detection.custom_header.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xbc, 0x13, 0x0a, 0x50, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x70, 0x5f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x2f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2f, 0x76,
    0x33, 0x2f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x6f, 0x72, 0x69,
    0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x70, 0x5f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f,
    0x76, 0x33, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xe5, 0x01, 0x0a,
    0x12, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x12, 0x2e, 0x0a, 0x0b, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x0d, 0xfa, 0x42, 0x0a, 0x72, 0x08, 0x10,
    0x01, 0xc8, 0x01, 0x01, 0xc0, 0x01, 0x01, 0x52, 0x0a, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x4e,
    0x61, 0x6d, 0x65, 0x12, 0x56, 0x0a, 0x29, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x5f, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x6f, 0x5f, 0x73, 0x65, 0x74, 0x5f, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x61, 0x73, 0x5f, 0x74, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x23, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x45, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x54, 0x6f, 0x53, 0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x41, 0x73, 0x54, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64, 0x12, 0x47, 0x0a, 0x12, 0x72,
    0x65, 0x6a, 0x65, 0x63, 0x74, 0x5f, 0x77, 0x69, 0x74, 0x68, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x48, 0x74, 0x74, 0x70, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x52, 0x10, 0x72, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x57, 0x69, 0x74, 0x68, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x42, 0xde, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a,
    0x4a, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x68, 0x74, 0x74, 0x70, 0x2e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x70,
    0x5f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x63, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x76, 0x33, 0x42, 0x11, 0x43, 0x75, 0x73,
    0x74, 0x6f, 0x6d, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
    0x5a, 0x73, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x70, 0x5f, 0x64, 0x65, 0x74, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x2f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x68, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x2f, 0x76, 0x33, 0x3b, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x68, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x76, 0x33, 0x4a, 0xff, 0x0d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2b, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x02, 0x00, 0x45, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x07, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x63, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09, 0x00, 0x63, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0a, 0x00, 0x32, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x32, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12,
    0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04, 0x0c, 0x00, 0x8a, 0x01, 0x0a,
    0x0a, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x04, 0x0c, 0x00, 0x8a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12,
    0x03, 0x0d, 0x00, 0x46, 0x0a, 0xbb, 0x05, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1a, 0x00, 0x2b,
    0x01, 0x1a, 0xea, 0x04, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x64, 0x6f, 0x77, 0x6e,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x49, 0x50,
    0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x65, 0x64, 0x0a,
    0x20, 0x62, 0x79, 0x20, 0x72, 0x65, 0x61, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x64, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x6e,
    0x61, 0x6d, 0x65, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x66, 0x75, 0x6c, 0x6c,
    0x79, 0x20, 0x70, 0x61, 0x72, 0x73, 0x65, 0x64, 0x0a, 0x20, 0x61, 0x73, 0x20, 0x61, 0x6e, 0x20,
    0x49, 0x50, 0x2c, 0x20, 0x69, 0x74, 0x27, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x74, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x66, 0x66, 0x65,
    0x63, 0x74, 0x69, 0x76, 0x65, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x65, 0x6e, 0x20, 0x61, 0x73, 0x20, 0x73, 0x75, 0x63, 0x68,
    0x0a, 0x20, 0x62, 0x79, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x2e, 0x20, 0x53, 0x65, 0x65, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x70, 0x5f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x20, 0x3c, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x68, 0x74, 0x74,
    0x70, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x61, 0x6e,
    0x61, 0x67, 0x65, 0x72, 0x2e, 0x76, 0x33, 0x2e, 0x48, 0x74, 0x74, 0x70, 0x43, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x6f, 0x72,
    0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x70, 0x5f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x3e, 0x60,
    0x0a, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65,
    0x77, 0x20, 0x6f, 0x66, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x77, 0x68, 0x61, 0x74, 0x20, 0x68, 0x61, 0x70, 0x70, 0x65, 0x6e, 0x73, 0x20, 0x77, 0x68, 0x65,
    0x6e, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x66,
    0x61, 0x69, 0x6c, 0x73, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x49, 0x50, 0x2e, 0x0a, 0x0a,
    0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61,
    0x6c, 0x5f, 0x69, 0x70, 0x5f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x63,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5d, 0x0a, 0x32, 0x42,
    0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c,
    0x65, 0x3a, 0x20, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x49, 0x50, 0x20, 0x64, 0x65, 0x74,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x1a, 0x0a, 0xc5,
    0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x1e, 0x02, 0x1f, 0x5f, 0x1a, 0xb6, 0x01,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x2e, 0x0a,
    0x0a, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x3a, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x61, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x2d, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x64, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2c, 0x20, 0x6f, 0x6e,
    0x6c, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x72, 0x69, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x74, 0x20, 0x61, 0x72, 0x65, 0x20, 0x69, 0x67, 0x6e,
    0x6f, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x1e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e,
    0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x17, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x1f, 0x06, 0x5e, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x1f, 0x07, 0x5d, 0x0a,
    0xc9, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x24, 0x02, 0x35, 0x1a, 0xbb, 0x02,
    0x20, 0x49, 0x66, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x63,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x64, 0x65, 0x63, 0x69, 0x64, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20,
    0x74, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x73, 0x0a, 0x20, 0x74, 0x72, 0x75, 0x73,
    0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x48, 0x43, 0x4d, 0x2e, 0x20,
    0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x69,
    0x73, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x72, 0x65, 0x64, 0x20, 0x3a, 0x72, 0x65,
    0x66, 0x3a, 0x60, 0x74, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x5f, 0x6d, 0x61, 0x6e, 0x5f,
    0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x5f, 0x78, 0x2d, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72,
    0x64, 0x65, 0x64, 0x2d, 0x66, 0x6f, 0x72, 0x5f, 0x74, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64, 0x5f,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x3e, 0x60,
    0x2c, 0x0a, 0x20, 0x69, 0x74, 0x20, 0x6d, 0x69, 0x67, 0x68, 0x74, 0x20, 0x62, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x20, 0x28, 0x61, 0x6d, 0x6f, 0x6e, 0x67, 0x20, 0x6f, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x73, 0x29, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x24, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x24, 0x07, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x24, 0x33, 0x34, 0x0a, 0xd5, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x2a, 0x02, 0x2c, 0x1a, 0xc7, 0x01, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69,
    0x73, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x6a, 0x65, 0x63,
    0x74, 0x65, 0x64, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x73, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x69,
    0x74, 0x20, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x48, 0x54, 0x54, 0x50, 0x20, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x0a, 0x0a,
    0x20, 0x2e, 0x2e, 0x20, 0x6e, 0x6f, 0x74, 0x65, 0x3a, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x49, 0x66,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x3c, 0x20, 0x34, 0x30, 0x30, 0x20, 0x6f, 0x72, 0x20, 0x3e, 0x20, 0x35, 0x31, 0x31, 0x2c, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x20, 0x34, 0x30, 0x33, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x61, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2a, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x15, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x2a, 0x2a, 0x2b, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.http.original_ip_detection.custom_header.v3.serde.rs");
// @@protoc_insertion_point(module)