// @generated
// [#protodoc-title: Generic Proxy Route Action Configuration]

/// Configuration for the route match action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteAction {
    /// Route metadata.
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<super::super::super::super::super::super::config::core::v3::Metadata>,
    /// Route level config for L7 generic filters. The key should be the related :ref:`extension name
    /// <envoy_v3_api_field_config.core.v3.TypedExtensionConfig.name>` in the :ref:`generic filters
    /// <envoy_v3_api_field_extensions.filters.network.generic_proxy.v3.GenericProxy.filters>`.
    #[prost(map="string, message", tag="4")]
    pub per_filter_config: ::std::collections::HashMap<::prost::alloc::string::String, ::pbjson_types::Any>,
    #[prost(oneof="route_action::ClusterSpecifier", tags="1, 2")]
    pub cluster_specifier: ::core::option::Option<route_action::ClusterSpecifier>,
}
/// Nested message and enum types in `RouteAction`.
pub mod route_action {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterSpecifier {
        /// Indicates the upstream cluster to which the request should be routed.
        #[prost(string, tag="1")]
        Cluster(::prost::alloc::string::String),
        /// \[#not-implemented-hide:\]
        /// Multiple upstream clusters can be specified for a given route. The request is routed to one
        /// of the upstream clusters based on weights assigned to each cluster.
        /// Currently ClusterWeight only supports the name and weight fields.
        #[prost(message, tag="2")]
        WeightedClusters(super::super::super::super::super::super::super::config::route::v3::WeightedCluster),
    }
}
/// Encoded file descriptor set for the `envoy.extensions.filters.network.generic_proxy.action.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xdd, 0x11, 0x0a, 0x4d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x67, 0x65,
    0x6e, 0x65, 0x72, 0x69, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x2f, 0x76, 0x33, 0x2f, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x38, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x5f, 0x70, 0x72, 0x6f,
    0x78, 0x79, 0x2e, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f,
    0x76, 0x33, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x2c, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x72, 0x6f, 0x75, 0x74,
    0x65, 0x2f, 0x76, 0x33, 0x2f, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x6f,
    0x6e, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x61, 0x6e, 0x79,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x78, 0x64, 0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x33, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e,
    0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0xb9, 0x03, 0x0a, 0x0b, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x1a, 0x0a, 0x07, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x48, 0x00, 0x52, 0x07, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x12, 0x55, 0x0a, 0x11, 0x77,
    0x65, 0x69, 0x67, 0x68, 0x74, 0x65, 0x64, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x73,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x57,
    0x65, 0x69, 0x67, 0x68, 0x74, 0x65, 0x64, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x48, 0x00,
    0x52, 0x10, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x65, 0x64, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x73, 0x12, 0x3a, 0x0a, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x4d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0x52, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x86,
    0x01, 0x0a, 0x11, 0x70, 0x65, 0x72, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x5f, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x5a, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x67, 0x65,
    0x6e, 0x65, 0x72, 0x69, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x76, 0x33, 0x2e, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x50, 0x65, 0x72, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x0f, 0x70, 0x65, 0x72, 0x46, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x1a, 0x58, 0x0a, 0x14, 0x50, 0x65, 0x72, 0x46, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12,
    0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65,
    0x79, 0x12, 0x2a, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x14, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2e, 0x41, 0x6e, 0x79, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38,
    0x01, 0x42, 0x18, 0x0a, 0x11, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x03, 0xf8, 0x42, 0x01, 0x42, 0xd1, 0x01, 0xba, 0x80,
    0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0xd2, 0xc6, 0xa4, 0xe1, 0x06, 0x02, 0x08, 0x01, 0x0a, 0x46,
    0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x67,
    0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x33, 0x42, 0x0b, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x68, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
    0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x67,
    0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x2f, 0x76, 0x33, 0x3b, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x76, 0x33, 0x4a,
    0xf6, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x41, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x05, 0x00, 0x36, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x23, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x0b, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x00, 0x21,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x5f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x0e, 0x00, 0x5f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x2c, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x2c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x10, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x10, 0x00, 0x22, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x11, 0x00, 0x7f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12,
    0x03, 0x11, 0x00, 0x7f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x12, 0x00, 0x46, 0x0a, 0x0d,
    0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x12, 0x00, 0x46, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x13, 0x00, 0x40, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xea, 0xc8, 0x94, 0x6c,
    0x01, 0x12, 0x03, 0x13, 0x00, 0x40, 0x0a, 0x76, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x18, 0x00,
    0x2d, 0x01, 0x1a, 0x2b, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x6f, 0x75, 0x74, 0x65,
    0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x32,
    0x3d, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74,
    0x6c, 0x65, 0x3a, 0x20, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x20, 0x50, 0x72, 0x6f, 0x78,
    0x79, 0x20, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x20, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5d, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x18, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00,
    0x08, 0x00, 0x12, 0x04, 0x19, 0x02, 0x24, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00,
    0x01, 0x12, 0x03, 0x19, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x02, 0x12,
    0x03, 0x1a, 0x04, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x08, 0x00, 0x02, 0xaf, 0x08, 0x12,
    0x03, 0x1a, 0x04, 0x26, 0x0a, 0x54, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x04,
    0x17, 0x1a, 0x47, 0x20, 0x49, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74,
    0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62,
    0x65, 0x20, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1d, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1d, 0x15, 0x16, 0x0a, 0x8d, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x23,
    0x04, 0x3a, 0x1a, 0xff, 0x01, 0x20, 0x5b, 0x23, 0x6e, 0x6f, 0x74, 0x2d, 0x69, 0x6d, 0x70, 0x6c,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x2d, 0x68, 0x69, 0x64, 0x65, 0x3a, 0x5d, 0x0a, 0x20,
    0x4d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62,
    0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x2e, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x72, 0x6f,
    0x75, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x6f, 0x6e, 0x65, 0x0a, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x63, 0x6c, 0x75,
    0x73, 0x74, 0x65, 0x72, 0x73, 0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x77,
    0x65, 0x69, 0x67, 0x68, 0x74, 0x73, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x0a, 0x20, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x43, 0x6c, 0x75, 0x73,
    0x74, 0x65, 0x72, 0x57, 0x65, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x73,
    0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x20, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x23,
    0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x24, 0x35,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x38, 0x39, 0x0a, 0x1e,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x27, 0x02, 0x27, 0x1a, 0x11, 0x20, 0x52, 0x6f,
    0x75, 0x74, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x27, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x27, 0x1a, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x27, 0x25, 0x26, 0x0a, 0xa3, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x2c, 0x02, 0x39, 0x1a, 0x95, 0x02, 0x20, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x20,
    0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x4c, 0x37, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x20, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x73, 0x68, 0x6f,
    0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x6c, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x0a, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f,
    0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x54, 0x79, 0x70, 0x65,
    0x64, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x2e, 0x6e, 0x61, 0x6d, 0x65, 0x3e, 0x60, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x3a,
    0x72, 0x65, 0x66, 0x3a, 0x60, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x20, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x73, 0x0a, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f,
    0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x5f, 0x70, 0x72, 0x6f,
    0x78, 0x79, 0x2e, 0x76, 0x33, 0x2e, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x50, 0x72, 0x6f,
    0x78, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x3e, 0x60, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2c, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2c, 0x23, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x2c, 0x37, 0x38, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.network.generic_proxy.action.v3.serde.rs");
// @@protoc_insertion_point(module)