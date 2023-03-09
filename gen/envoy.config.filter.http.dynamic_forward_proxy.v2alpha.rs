// @generated
// [#protodoc-title: Dynamic forward proxy]

/// Configuration for the dynamic forward proxy HTTP filter. See the :ref:`architecture overview
/// <arch_overview_http_dynamic_forward_proxy>` for more information.
/// [#extension: envoy.filters.http.dynamic_forward_proxy]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfig {
    /// The DNS cache configuration that the filter will attach to. Note this configuration must
    /// match that of associated :ref:`dynamic forward proxy cluster configuration
    /// <envoy_api_field_config.cluster.dynamic_forward_proxy.v2alpha.ClusterConfig.dns_cache_config>`.
    #[prost(message, optional, tag="1")]
    pub dns_cache_config: ::core::option::Option<super::super::super::super::common::dynamic_forward_proxy::v2alpha::DnsCacheConfig>,
}
/// Per route Configuration for the dynamic forward proxy HTTP filter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerRouteConfig {
    #[prost(oneof="per_route_config::HostRewriteSpecifier", tags="1, 2")]
    pub host_rewrite_specifier: ::core::option::Option<per_route_config::HostRewriteSpecifier>,
}
/// Nested message and enum types in `PerRouteConfig`.
pub mod per_route_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HostRewriteSpecifier {
        /// Indicates that before DNS lookup, the host header will be swapped with
        /// this value. If not set or empty, the original host header value
        /// will be used and no rewrite will happen.
        ///
        /// Note: this rewrite affects both DNS lookup and host header forwarding. However, this
        /// option shouldn't be used with
        /// :ref:`HCM host rewrite <envoy_api_field_route.RouteAction.host_rewrite>` given that the
        /// value set here would be used for DNS lookups whereas the value set in the HCM would be used
        /// for host header forwarding which is not the desired outcome.
        #[prost(string, tag="1")]
        HostRewrite(::prost::alloc::string::String),
        /// Indicates that before DNS lookup, the host header will be swapped with
        /// the value of this header. If not set or empty, the original host header
        /// value will be used and no rewrite will happen.
        ///
        /// Note: this rewrite affects both DNS lookup and host header forwarding. However, this
        /// option shouldn't be used with
        /// :ref:`HCM host rewrite header <envoy_api_field_route.RouteAction.auto_host_rewrite_header>`
        /// given that the value set here would be used for DNS lookups whereas the value set in the HCM
        /// would be used for host header forwarding which is not the desired outcome.
        ///
        /// .. note::
        ///
        ///    If the header appears multiple times only the first value is used.
        #[prost(string, tag="2")]
        AutoHostRewriteHeader(::prost::alloc::string::String),
    }
}
/// Encoded file descriptor set for the `envoy.config.filter.http.dynamic_forward_proxy.v2alpha` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xc0, 0x19, 0x0a, 0x52, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x64, 0x79,
    0x6e, 0x61, 0x6d, 0x69, 0x63, 0x5f, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72,
    0x6f, 0x78, 0x79, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2f, 0x64, 0x79, 0x6e, 0x61,
    0x6d, 0x69, 0x63, 0x5f, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x78,
    0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x36, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74,
    0x70, 0x2e, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x5f, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72,
    0x64, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x1a,
    0x41, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f,
    0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x5f, 0x66, 0x6f, 0x72,
    0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0x2f, 0x64, 0x6e, 0x73, 0x5f, 0x63, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x1e, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x85, 0x01, 0x0a, 0x0c, 0x46,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x75, 0x0a, 0x10, 0x64,
    0x6e, 0x73, 0x5f, 0x63, 0x61, 0x63, 0x68, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x41, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x64, 0x79, 0x6e, 0x61,
    0x6d, 0x69, 0x63, 0x5f, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x78,
    0x79, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x44, 0x6e, 0x73, 0x43, 0x61, 0x63,
    0x68, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x8a, 0x01, 0x02,
    0x10, 0x01, 0x52, 0x0e, 0x64, 0x6e, 0x73, 0x43, 0x61, 0x63, 0x68, 0x65, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x22, 0xc5, 0x01, 0x0a, 0x0e, 0x50, 0x65, 0x72, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x41, 0x0a, 0x0c, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x72, 0x65,
    0x77, 0x72, 0x69, 0x74, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x1c, 0xf2, 0x98, 0xfe,
    0x8f, 0x05, 0x16, 0x0a, 0x14, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74,
    0x65, 0x5f, 0x6c, 0x69, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x0b, 0x68, 0x6f, 0x73,
    0x74, 0x52, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x12, 0x56, 0x0a, 0x18, 0x61, 0x75, 0x74, 0x6f,
    0x5f, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x1b, 0xf2, 0x98, 0xfe, 0x8f,
    0x05, 0x15, 0x0a, 0x13, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65,
    0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x48, 0x00, 0x52, 0x15, 0x61, 0x75, 0x74, 0x6f, 0x48,
    0x6f, 0x73, 0x74, 0x52, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x42, 0x18, 0x0a, 0x16, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65,
    0x5f, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x72, 0x42, 0x87, 0x02, 0xf2, 0x98, 0xfe,
    0x8f, 0x05, 0x38, 0x12, 0x36, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74,
    0x74, 0x70, 0x2e, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x5f, 0x66, 0x6f, 0x72, 0x77, 0x61,
    0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x76, 0x33, 0xba, 0x80, 0xc8, 0xd1, 0x06,
    0x02, 0x10, 0x01, 0x0a, 0x44, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f,
    0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x64, 0x79, 0x6e, 0x61,
    0x6d, 0x69, 0x63, 0x5f, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x78,
    0x79, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x42, 0x18, 0x44, 0x79, 0x6e, 0x61, 0x6d,
    0x69, 0x63, 0x46, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x50, 0x72, 0x6f, 0x78, 0x79, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x5d, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
    0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x5f, 0x66,
    0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x76, 0x32, 0x61,
    0x6c, 0x70, 0x68, 0x61, 0x4a, 0xb4, 0x12, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x3d, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x02, 0x00, 0x3f, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x4b, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x07, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08, 0x00, 0x21, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x5d, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x0a, 0x00, 0x5d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x39, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x39, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0c, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0c, 0x00, 0x22, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x74, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03,
    0x0d, 0x00, 0x74, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04, 0x0e, 0x00, 0x0f, 0x3d, 0x0a, 0x0e,
    0x0a, 0x06, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12, 0x04, 0x0e, 0x00, 0x0f, 0x3d, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x10, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99,
    0x6a, 0x02, 0x12, 0x03, 0x10, 0x00, 0x46, 0x0a, 0x92, 0x02, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x17, 0x00, 0x1d, 0x01, 0x1a, 0xd9, 0x01, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x79,
    0x6e, 0x61, 0x6d, 0x69, 0x63, 0x20, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x20, 0x70, 0x72,
    0x6f, 0x78, 0x79, 0x20, 0x48, 0x54, 0x54, 0x50, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e,
    0x20, 0x53, 0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x61,
    0x72, 0x63, 0x68, 0x69, 0x74, 0x65, 0x63, 0x74, 0x75, 0x72, 0x65, 0x20, 0x6f, 0x76, 0x65, 0x72,
    0x76, 0x69, 0x65, 0x77, 0x0a, 0x20, 0x3c, 0x61, 0x72, 0x63, 0x68, 0x5f, 0x6f, 0x76, 0x65, 0x72,
    0x76, 0x69, 0x65, 0x77, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69,
    0x63, 0x5f, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x3e,
    0x60, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63,
    0x5f, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x5d, 0x0a,
    0x32, 0x2a, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69,
    0x74, 0x6c, 0x65, 0x3a, 0x20, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x20, 0x66, 0x6f, 0x72,
    0x77, 0x61, 0x72, 0x64, 0x20, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x17, 0x08, 0x14, 0x0a, 0x96, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x04, 0x1b, 0x02, 0x1c, 0x34, 0x1a, 0x87, 0x02, 0x20, 0x54, 0x68, 0x65, 0x20, 0x44,
    0x4e, 0x53, 0x20, 0x63, 0x61, 0x63, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x61, 0x74, 0x74, 0x61,
    0x63, 0x68, 0x20, 0x74, 0x6f, 0x2e, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6d,
    0x75, 0x73, 0x74, 0x0a, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x6f, 0x66, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x3a, 0x72,
    0x65, 0x66, 0x3a, 0x60, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x20, 0x66, 0x6f, 0x72, 0x77,
    0x61, 0x72, 0x64, 0x20, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a,
    0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x5f, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64,
    0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x43,
    0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x64, 0x6e, 0x73,
    0x5f, 0x63, 0x61, 0x63, 0x68, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x3e, 0x60, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1b, 0x02, 0x35, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x36, 0x46, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x49, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x1c, 0x06, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02,
    0x00, 0x08, 0xaf, 0x08, 0x11, 0x12, 0x03, 0x1c, 0x07, 0x32, 0x0a, 0x50, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x20, 0x00, 0x3d, 0x01, 0x1a, 0x44, 0x20, 0x50, 0x65, 0x72, 0x20, 0x72, 0x6f, 0x75,
    0x74, 0x65, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63,
    0x20, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x20, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x20, 0x48,
    0x54, 0x54, 0x50, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x20, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00,
    0x12, 0x04, 0x21, 0x02, 0x3c, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x01, 0x12,
    0x03, 0x21, 0x08, 0x1e, 0x0a, 0xab, 0x04, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x2b,
    0x04, 0x5f, 0x1a, 0x9d, 0x04, 0x20, 0x49, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x44, 0x4e, 0x53, 0x20,
    0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x6f, 0x73, 0x74,
    0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20,
    0x73, 0x77, 0x61, 0x70, 0x70, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x72, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2c, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x68, 0x6f, 0x73, 0x74,
    0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x0a, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x6e, 0x6f, 0x20, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x68, 0x61, 0x70, 0x70, 0x65, 0x6e, 0x2e, 0x0a, 0x0a, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x3a, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x61, 0x66, 0x66,
    0x65, 0x63, 0x74, 0x73, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x44, 0x4e, 0x53, 0x20, 0x6c, 0x6f,
    0x6f, 0x6b, 0x75, 0x70, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x2e,
    0x20, 0x48, 0x6f, 0x77, 0x65, 0x76, 0x65, 0x72, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x0a, 0x20,
    0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x6e, 0x27, 0x74,
    0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x3a,
    0x72, 0x65, 0x66, 0x3a, 0x60, 0x48, 0x43, 0x4d, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x72, 0x65,
    0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x61, 0x70, 0x69,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x2e, 0x52, 0x6f, 0x75,
    0x74, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x72, 0x65,
    0x77, 0x72, 0x69, 0x74, 0x65, 0x3e, 0x60, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x68, 0x65, 0x72, 0x65, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x44, 0x4e, 0x53, 0x20, 0x6c, 0x6f, 0x6f,
    0x6b, 0x75, 0x70, 0x73, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x48, 0x43, 0x4d, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x0a, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x20,
    0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x64, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x20, 0x6f, 0x75, 0x74, 0x63, 0x6f, 0x6d, 0x65,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b, 0x04, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x0b, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x08, 0x12, 0x03, 0x2b, 0x1c, 0x5e, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01,
    0x02, 0x00, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x01, 0x12, 0x03, 0x2b, 0x1d, 0x5d, 0x0a, 0xa0, 0x05,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x04, 0x3a, 0x04, 0x3b, 0x4a, 0x1a, 0x91, 0x05, 0x20,
    0x49, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x62,
    0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x44, 0x4e, 0x53, 0x20, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70,
    0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73, 0x77, 0x61, 0x70, 0x70, 0x65,
    0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x72, 0x20,
    0x65, 0x6d, 0x70, 0x74, 0x79, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x61, 0x6c, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x0a,
    0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6e, 0x6f, 0x20, 0x72, 0x65, 0x77, 0x72, 0x69,
    0x74, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x68, 0x61, 0x70, 0x70, 0x65, 0x6e, 0x2e, 0x0a,
    0x0a, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x3a, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x77,
    0x72, 0x69, 0x74, 0x65, 0x20, 0x61, 0x66, 0x66, 0x65, 0x63, 0x74, 0x73, 0x20, 0x62, 0x6f, 0x74,
    0x68, 0x20, 0x44, 0x4e, 0x53, 0x20, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72,
    0x77, 0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x2e, 0x20, 0x48, 0x6f, 0x77, 0x65, 0x76, 0x65, 0x72,
    0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x0a, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x6e, 0x27, 0x74, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x48, 0x43, 0x4d,
    0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x2e, 0x52, 0x6f, 0x75, 0x74,
    0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x61, 0x75, 0x74, 0x6f, 0x5f, 0x68, 0x6f, 0x73,
    0x74, 0x5f, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x3e, 0x60, 0x0a, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x68, 0x65, 0x72,
    0x65, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x44, 0x4e, 0x53, 0x20, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x73, 0x20,
    0x77, 0x68, 0x65, 0x72, 0x65, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x48, 0x43, 0x4d,
    0x0a, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20,
    0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68,
    0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x73, 0x69,
    0x72, 0x65, 0x64, 0x20, 0x6f, 0x75, 0x74, 0x63, 0x6f, 0x6d, 0x65, 0x2e, 0x0a, 0x0a, 0x20, 0x2e,
    0x2e, 0x20, 0x6e, 0x6f, 0x74, 0x65, 0x3a, 0x3a, 0x0a, 0x0a, 0x20, 0x20, 0x20, 0x49, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x61, 0x70, 0x70, 0x65, 0x61,
    0x72, 0x73, 0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74,
    0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3a, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x0b, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3a, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x08, 0x12, 0x03, 0x3b, 0x08, 0x49, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02, 0x01,
    0x08, 0x8e, 0xe3, 0xff, 0x51, 0x01, 0x12, 0x03, 0x3b, 0x09, 0x48, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
include!("envoy.config.filter.http.dynamic_forward_proxy.v2alpha.serde.rs");
// @@protoc_insertion_point(module)