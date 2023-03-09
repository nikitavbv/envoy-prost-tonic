// @generated
// [#protodoc-title: IP tagging]
// IP tagging :ref:`configuration overview <config_http_filters_ip_tagging>`.
// [#extension: envoy.filters.http.ip_tagging]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpTagging {
    /// The type of request the filter should apply to.
    #[prost(enumeration="ip_tagging::RequestType", tag="1")]
    pub request_type: i32,
    /// [#comment:TODO(ccaraman): Extend functionality to load IP tags from file system.
    /// Tracked by issue <https://github.com/envoyproxy/envoy/issues/2695]>
    /// The set of IP tags for the filter.
    #[prost(message, repeated, tag="4")]
    pub ip_tags: ::prost::alloc::vec::Vec<ip_tagging::IpTag>,
}
/// Nested message and enum types in `IPTagging`.
pub mod ip_tagging {
    /// Supplies the IP tag name and the IP address subnets.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IpTag {
        /// Specifies the IP tag name to apply.
        #[prost(string, tag="1")]
        pub ip_tag_name: ::prost::alloc::string::String,
        /// A list of IP address subnets that will be tagged with
        /// ip_tag_name. Both IPv4 and IPv6 are supported.
        #[prost(message, repeated, tag="2")]
        pub ip_list: ::prost::alloc::vec::Vec<super::super::super::super::super::super::config::core::v3::CidrRange>,
    }
    /// The type of requests the filter should apply to. The supported types
    /// are internal, external or both. The
    /// :ref:`x-forwarded-for<config_http_conn_man_headers_x-forwarded-for_internal_origin>` header is
    /// used to determine if a request is internal and will result in
    /// :ref:`x-envoy-internal<config_http_conn_man_headers_x-envoy-internal>`
    /// being set. The filter defaults to both, and it will apply to all request types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RequestType {
        /// Both external and internal requests will be tagged. This is the default value.
        Both = 0,
        /// Only internal requests will be tagged.
        Internal = 1,
        /// Only external requests will be tagged.
        External = 2,
    }
    impl RequestType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RequestType::Both => "BOTH",
                RequestType::Internal => "INTERNAL",
                RequestType::External => "EXTERNAL",
            }
        }
    }
}
/// Encoded file descriptor set for the `envoy.extensions.filters.http.ip_tagging.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xdc, 0x16, 0x0a, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68, 0x74,
    0x74, 0x70, 0x2f, 0x69, 0x70, 0x5f, 0x74, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2f, 0x76, 0x33,
    0x2f, 0x69, 0x70, 0x5f, 0x74, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x2b, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70,
    0x2e, 0x69, 0x70, 0x5f, 0x74, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x33, 0x1a, 0x22,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f, 0x72,
    0x65, 0x2f, 0x76, 0x33, 0x2f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x21, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xee, 0x03,
    0x0a, 0x09, 0x49, 0x50, 0x54, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x12, 0x6f, 0x0a, 0x0c, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x42, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74,
    0x70, 0x2e, 0x69, 0x70, 0x5f, 0x74, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x33, 0x2e,
    0x49, 0x50, 0x54, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x54, 0x79, 0x70, 0x65, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x82, 0x01, 0x02, 0x10, 0x01, 0x52,
    0x0b, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x5f, 0x0a, 0x07,
    0x69, 0x70, 0x5f, 0x74, 0x61, 0x67, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x3c, 0x2e,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x69, 0x70,
    0x5f, 0x74, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x33, 0x2e, 0x49, 0x50, 0x54, 0x61,
    0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x49, 0x50, 0x54, 0x61, 0x67, 0x42, 0x08, 0xfa, 0x42, 0x05,
    0x92, 0x01, 0x02, 0x08, 0x01, 0x52, 0x06, 0x69, 0x70, 0x54, 0x61, 0x67, 0x73, 0x1a, 0xa0, 0x01,
    0x0a, 0x05, 0x49, 0x50, 0x54, 0x61, 0x67, 0x12, 0x1e, 0x0a, 0x0b, 0x69, 0x70, 0x5f, 0x74, 0x61,
    0x67, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x69, 0x70,
    0x54, 0x61, 0x67, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x38, 0x0a, 0x07, 0x69, 0x70, 0x5f, 0x6c, 0x69,
    0x73, 0x74, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e,
    0x43, 0x69, 0x64, 0x72, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x06, 0x69, 0x70, 0x4c, 0x69, 0x73,
    0x74, 0x3a, 0x3d, 0x9a, 0xc5, 0x88, 0x1e, 0x38, 0x0a, 0x36, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74,
    0x74, 0x70, 0x2e, 0x69, 0x70, 0x5f, 0x74, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x32,
    0x2e, 0x49, 0x50, 0x54, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x49, 0x50, 0x54, 0x61, 0x67,
    0x22, 0x33, 0x0a, 0x0b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x08, 0x0a, 0x04, 0x42, 0x4f, 0x54, 0x48, 0x10, 0x00, 0x12, 0x0c, 0x0a, 0x08, 0x49, 0x4e, 0x54,
    0x45, 0x52, 0x4e, 0x41, 0x4c, 0x10, 0x01, 0x12, 0x0c, 0x0a, 0x08, 0x45, 0x58, 0x54, 0x45, 0x52,
    0x4e, 0x41, 0x4c, 0x10, 0x02, 0x3a, 0x37, 0x9a, 0xc5, 0x88, 0x1e, 0x32, 0x0a, 0x30, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x69, 0x70, 0x5f, 0x74, 0x61, 0x67, 0x67, 0x69, 0x6e,
    0x67, 0x2e, 0x76, 0x32, 0x2e, 0x49, 0x50, 0x54, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x42, 0xb6,
    0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x39, 0x69, 0x6f, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x69, 0x70, 0x5f, 0x74, 0x61, 0x67, 0x67, 0x69, 0x6e,
    0x67, 0x2e, 0x76, 0x33, 0x42, 0x0e, 0x49, 0x70, 0x54, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x5f, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63,
    0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f,
    0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x69, 0x70, 0x5f,
    0x74, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2f, 0x76, 0x33, 0x3b, 0x69, 0x70, 0x5f, 0x74, 0x61,
    0x67, 0x67, 0x69, 0x6e, 0x67, 0x76, 0x33, 0x4a, 0xbd, 0x0f, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x3d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x34, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00,
    0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08,
    0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x52, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x0a, 0x00, 0x52, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00,
    0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x2f, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0c, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0c, 0x00,
    0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x76, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x0b, 0x12, 0x03, 0x0d, 0x00, 0x76, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x46,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0e, 0x00, 0x46, 0x0a,
    0xa5, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x14, 0x00, 0x3d, 0x01, 0x32, 0x98, 0x01, 0x20,
    0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65,
    0x3a, 0x20, 0x49, 0x50, 0x20, 0x74, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x5d, 0x0a, 0x20, 0x49,
    0x50, 0x20, 0x74, 0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76,
    0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68,
    0x74, 0x74, 0x70, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x69, 0x70, 0x5f, 0x74,
    0x61, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x3e, 0x60, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x69, 0x70, 0x5f, 0x74, 0x61,
    0x67, 0x67, 0x69, 0x6e, 0x67, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x14, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x15, 0x02, 0x16, 0x39,
    0x0a, 0x10, 0x0a, 0x08, 0x04, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x15, 0x02,
    0x16, 0x39, 0x0a, 0xb2, 0x03, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x1e, 0x02, 0x27,
    0x03, 0x1a, 0xa3, 0x03, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x61, 0x70, 0x70, 0x6c,
    0x79, 0x20, 0x74, 0x6f, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72,
    0x74, 0x65, 0x64, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x0a, 0x20, 0x61, 0x72, 0x65, 0x20, 0x69,
    0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2c, 0x20, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61,
    0x6c, 0x20, 0x6f, 0x72, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x0a, 0x20,
    0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x78, 0x2d, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x65,
    0x64, 0x2d, 0x66, 0x6f, 0x72, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74,
    0x70, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x5f, 0x6d, 0x61, 0x6e, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x73, 0x5f, 0x78, 0x2d, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x2d, 0x66,
    0x6f, 0x72, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x3e, 0x60, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x69, 0x73, 0x0a, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e,
    0x65, 0x20, 0x69, 0x66, 0x20, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x6e, 0x0a, 0x20, 0x3a,
    0x72, 0x65, 0x66, 0x3a, 0x60, 0x78, 0x2d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2d, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74,
    0x70, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x5f, 0x6d, 0x61, 0x6e, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x73, 0x5f, 0x78, 0x2d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2d, 0x69, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x3e, 0x60, 0x0a, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x65, 0x74,
    0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x64, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x2c, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x69, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x79,
    0x20, 0x74, 0x6f, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x1e, 0x07, 0x12, 0x0a, 0x5f, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x20, 0x04, 0x0d, 0x1a, 0x50, 0x20, 0x42, 0x6f, 0x74, 0x68, 0x20, 0x65, 0x78, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61,
    0x6c, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x62, 0x65, 0x20, 0x74, 0x61, 0x67, 0x67, 0x65, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20,
    0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x20, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x20, 0x0b, 0x0c, 0x0a, 0x37, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x23, 0x04, 0x11, 0x1a, 0x28, 0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x74, 0x61, 0x67, 0x67, 0x65, 0x64, 0x2e, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x04, 0x0c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x23, 0x0f, 0x10, 0x0a,
    0x37, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x26, 0x04, 0x11, 0x1a, 0x28,
    0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20,
    0x74, 0x61, 0x67, 0x67, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x26, 0x0f, 0x10, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00,
    0x12, 0x04, 0x2a, 0x02, 0x34, 0x03, 0x1a, 0x36, 0x20, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x69, 0x65,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x49, 0x50, 0x20, 0x74, 0x61, 0x67, 0x20, 0x6e, 0x61, 0x6d,
    0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x49, 0x50, 0x20, 0x61, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x20, 0x73, 0x75, 0x62, 0x6e, 0x65, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x0a, 0x0f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x03, 0x00, 0x07, 0x12, 0x04, 0x2b, 0x04, 0x2c, 0x41, 0x0a, 0x12, 0x0a, 0x0a, 0x04,
    0x00, 0x03, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x2b, 0x04, 0x2c, 0x41, 0x0a,
    0x34, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x04, 0x1b, 0x1a, 0x25,
    0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x49,
    0x50, 0x20, 0x74, 0x61, 0x67, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x70,
    0x70, 0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x2f, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x2f, 0x0b, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x2f, 0x19, 0x1a, 0x0a, 0x76, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x33, 0x04, 0x32, 0x1a, 0x67, 0x20, 0x41, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x49, 0x50, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x73, 0x75, 0x62, 0x6e,
    0x65, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65,
    0x20, 0x74, 0x61, 0x67, 0x67, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x69, 0x70,
    0x5f, 0x74, 0x61, 0x67, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x2e, 0x20, 0x42, 0x6f, 0x74, 0x68, 0x20,
    0x49, 0x50, 0x76, 0x34, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x49, 0x50, 0x76, 0x36, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x33, 0x04, 0x0c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x33, 0x0d, 0x25, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x33, 0x26, 0x2d, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x33, 0x30, 0x31, 0x0a, 0x3e, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x37, 0x02, 0x4e, 0x1a, 0x31, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x20, 0x74, 0x6f, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x37, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x37, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x37, 0x1f, 0x4d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08,
    0x10, 0x12, 0x03, 0x37, 0x20, 0x4c, 0x0a, 0xc7, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x3c, 0x02, 0x4a, 0x1a, 0xb9, 0x01, 0x20, 0x5b, 0x23, 0x63, 0x6f, 0x6d, 0x6d, 0x65, 0x6e,
    0x74, 0x3a, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x63, 0x63, 0x61, 0x72, 0x61, 0x6d, 0x61, 0x6e, 0x29,
    0x3a, 0x20, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x64, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x49,
    0x50, 0x20, 0x74, 0x61, 0x67, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x66, 0x69, 0x6c, 0x65,
    0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x2e, 0x0a, 0x20, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x65,
    0x64, 0x20, 0x62, 0x79, 0x20, 0x69, 0x73, 0x73, 0x75, 0x65, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73,
    0x3a, 0x2f, 0x2f, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x69,
    0x73, 0x73, 0x75, 0x65, 0x73, 0x2f, 0x32, 0x36, 0x39, 0x35, 0x5d, 0x0a, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x49, 0x50, 0x20, 0x74, 0x61, 0x67, 0x73, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3c, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3c, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x3c, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x08, 0x12, 0x03, 0x3c, 0x1d, 0x49, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf,
    0x08, 0x12, 0x12, 0x03, 0x3c, 0x1e, 0x48, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)