// @generated
// [#protodoc-title: Client TLS authentication]
// Client TLS authentication
// :ref:`configuration overview <config_network_filters_client_ssl_auth>`.
// [#extension: envoy.filters.network.client_ssl_auth]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientSslAuth {
    /// The :ref:`cluster manager <arch_overview_cluster_manager>` cluster that runs
    /// the authentication service. The filter will connect to the service every 60s to fetch the list
    /// of principals. The service must support the expected :ref:`REST API
    /// <config_network_filters_client_ssl_auth_rest_api>`.
    #[prost(string, tag="1")]
    pub auth_api_cluster: ::prost::alloc::string::String,
    /// The prefix to use when emitting :ref:`statistics
    /// <config_network_filters_client_ssl_auth_stats>`.
    #[prost(string, tag="2")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// Time in milliseconds between principal refreshes from the
    /// authentication service. Default is 60000 (60s). The actual fetch time
    /// will be this value plus a random jittered value between
    /// 0-refresh_delay_ms milliseconds.
    #[prost(message, optional, tag="3")]
    pub refresh_delay: ::core::option::Option<::pbjson_types::Duration>,
    /// An optional list of IP address and subnet masks that should be white
    /// listed for access by the filter. If no list is provided, there is no
    /// IP allowlist.
    #[prost(message, repeated, tag="4")]
    pub ip_white_list: ::prost::alloc::vec::Vec<super::super::super::super::super::config::core::v3::CidrRange>,
}
/// Encoded file descriptor set for the `envoy.extensions.filters.network.client_ssl_auth.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xa5, 0x13, 0x0a, 0x51, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x2f, 0x76, 0x33,
    0x2f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x61, 0x75, 0x74, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x33, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f,
    0x73, 0x73, 0x6c, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x76, 0x33, 0x1a, 0x22, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76,
    0x33, 0x2f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
    0x2f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1e, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x2f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x21,
    0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xd2, 0x02, 0x0a, 0x0d, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x53, 0x4c, 0x41, 0x75, 0x74, 0x68, 0x12, 0x37, 0x0a, 0x10,
    0x61, 0x75, 0x74, 0x68, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x0d, 0xfa, 0x42, 0x0a, 0x72, 0x08, 0x10, 0x01, 0xc8,
    0x01, 0x00, 0xc0, 0x01, 0x02, 0x52, 0x0e, 0x61, 0x75, 0x74, 0x68, 0x41, 0x70, 0x69, 0x43, 0x6c,
    0x75, 0x73, 0x74, 0x65, 0x72, 0x12, 0x28, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x74, 0x5f, 0x70, 0x72,
    0x65, 0x66, 0x69, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x72,
    0x02, 0x10, 0x01, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x50, 0x72, 0x65, 0x66, 0x69, 0x78, 0x12,
    0x3e, 0x0a, 0x0d, 0x72, 0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x5f, 0x64, 0x65, 0x6c, 0x61, 0x79,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x52, 0x0c, 0x72, 0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x44, 0x65, 0x6c, 0x61, 0x79, 0x12,
    0x59, 0x0a, 0x0d, 0x69, 0x70, 0x5f, 0x77, 0x68, 0x69, 0x74, 0x65, 0x5f, 0x6c, 0x69, 0x73, 0x74,
    0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x43, 0x69,
    0x64, 0x72, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x42, 0x14, 0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x0e, 0x0a,
    0x0c, 0x69, 0x70, 0x5f, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x6c, 0x69, 0x73, 0x74, 0x52, 0x0b, 0x69,
    0x70, 0x57, 0x68, 0x69, 0x74, 0x65, 0x4c, 0x69, 0x73, 0x74, 0x3a, 0x43, 0x9a, 0xc5, 0x88, 0x1e,
    0x3e, 0x0a, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x76,
    0x32, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x53, 0x4c, 0x41, 0x75, 0x74, 0x68, 0x42,
    0xcf, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x41, 0x69, 0x6f, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x76, 0x33, 0x42, 0x12, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x73, 0x6c, 0x41, 0x75, 0x74, 0x68, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x50, 0x01, 0x5a, 0x6c, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x2f, 0x76, 0x33, 0x3b,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x76,
    0x33, 0x4a, 0xac, 0x0c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x32, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x3c,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x06, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x08, 0x00,
    0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x0a, 0x00, 0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0b,
    0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x5a, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x0d, 0x00, 0x5a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00,
    0x33, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x33, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0f, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0f, 0x00,
    0x22, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04, 0x10, 0x00, 0x83, 0x01, 0x0a, 0x0a, 0x0a, 0x02,
    0x08, 0x0b, 0x12, 0x04, 0x10, 0x00, 0x83, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x11,
    0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x11, 0x00,
    0x46, 0x0a, 0xd4, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x18, 0x00, 0x32, 0x01, 0x32, 0xc7,
    0x01, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74,
    0x6c, 0x65, 0x3a, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x54, 0x4c, 0x53, 0x20, 0x61,
    0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5d, 0x0a, 0x20,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x54, 0x4c, 0x53, 0x20, 0x61, 0x75, 0x74, 0x68, 0x65,
    0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a,
    0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f,
    0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x3e,
    0x60, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a,
    0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x73,
    0x6c, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x18, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x19, 0x02, 0x1a,
    0x45, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x19,
    0x02, 0x1a, 0x45, 0x0a, 0xb7, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x20, 0x02,
    0x21, 0x61, 0x1a, 0xa8, 0x02, 0x20, 0x54, 0x68, 0x65, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60,
    0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x20,
    0x3c, 0x61, 0x72, 0x63, 0x68, 0x5f, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x5f, 0x63,
    0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x3e, 0x60,
    0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x72, 0x75,
    0x6e, 0x73, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x36, 0x30, 0x73,
    0x20, 0x74, 0x6f, 0x20, 0x66, 0x65, 0x74, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69,
    0x73, 0x74, 0x0a, 0x20, 0x6f, 0x66, 0x20, 0x70, 0x72, 0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c,
    0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x6d,
    0x75, 0x73, 0x74, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x52,
    0x45, 0x53, 0x54, 0x20, 0x41, 0x50, 0x49, 0x0a, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x5f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x61, 0x75, 0x74, 0x68,
    0x5f, 0x72, 0x65, 0x73, 0x74, 0x5f, 0x61, 0x70, 0x69, 0x3e, 0x60, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x09, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x20, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x21, 0x06, 0x60, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08,
    0x0e, 0x12, 0x03, 0x21, 0x07, 0x5f, 0x0a, 0x71, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x25, 0x02, 0x42, 0x1a, 0x64, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78,
    0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x65, 0x6d, 0x69,
    0x74, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x73, 0x74, 0x61, 0x74,
    0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x0a, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x5f,
    0x73, 0x74, 0x61, 0x74, 0x73, 0x3e, 0x60, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x25, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x25, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x25, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x25, 0x19,
    0x41, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x25,
    0x1a, 0x40, 0x0a, 0xeb, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x02, 0x2d,
    0x1a, 0xdd, 0x01, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x69, 0x6c, 0x6c,
    0x69, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e,
    0x20, 0x70, 0x72, 0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x66, 0x72, 0x65,
    0x73, 0x68, 0x65, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x61,
    0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x36, 0x30, 0x30, 0x30, 0x30, 0x20, 0x28, 0x36, 0x30, 0x73, 0x29, 0x2e, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x66, 0x65, 0x74, 0x63, 0x68, 0x20,
    0x74, 0x69, 0x6d, 0x65, 0x0a, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x70, 0x6c, 0x75, 0x73, 0x20, 0x61, 0x20,
    0x72, 0x61, 0x6e, 0x64, 0x6f, 0x6d, 0x20, 0x6a, 0x69, 0x74, 0x74, 0x65, 0x72, 0x65, 0x64, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x0a, 0x20, 0x30,
    0x2d, 0x72, 0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x5f, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x5f, 0x6d,
    0x73, 0x20, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2b, 0x02, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x1b, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x2b, 0x2c, 0x0a, 0xaa, 0x01, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x04, 0x30, 0x02, 0x31, 0x41, 0x1a, 0x9b, 0x01, 0x20, 0x41, 0x6e, 0x20,
    0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x49, 0x50, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x73, 0x75, 0x62, 0x6e, 0x65, 0x74, 0x20, 0x6d, 0x61, 0x73, 0x6b, 0x73, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x77, 0x68, 0x69, 0x74,
    0x65, 0x0a, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x72,
    0x65, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x0a, 0x20, 0x49, 0x50, 0x20, 0x61, 0x6c, 0x6c, 0x6f,
    0x77, 0x6c, 0x69, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03,
    0x30, 0x0b, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x30, 0x24,
    0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x30, 0x34, 0x35, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x08, 0x12, 0x03, 0x31, 0x06, 0x40, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x00, 0x02, 0x03, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x01, 0x12, 0x03, 0x31, 0x07, 0x3f,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.network.client_ssl_auth.v3.serde.rs");
// @@protoc_insertion_point(module)