// @generated
// [#protodoc-title: Local rate limit]
// Local rate limit :ref:`configuration overview <config_network_filters_local_rate_limit>`.
// [#extension: envoy.filters.network.local_ratelimit]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalRateLimit {
    /// The prefix to use when emitting :ref:`statistics
    /// <config_network_filters_local_rate_limit_stats>`.
    #[prost(string, tag="1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// The token bucket configuration to use for rate limiting connections that are processed by the
    /// filter's filter chain. Each incoming connection processed by the filter consumes a single
    /// token. If the token is available, the connection will be allowed. If no tokens are available,
    /// the connection will be immediately closed.
    ///
    /// .. note::
    ///    In the current implementation each filter and filter chain has an independent rate limit, unless
    ///    a shared rate limit is configured via :ref:`share_key <envoy_v3_api_field_extensions.filters.network.local_ratelimit.v3.LocalRateLimit.share_key>`.
    ///
    /// .. note::
    ///    In the current implementation the token bucket's :ref:`fill_interval
    ///    <envoy_v3_api_field_type.v3.TokenBucket.fill_interval>` must be >= 50ms to avoid too aggressive
    ///    refills.
    #[prost(message, optional, tag="2")]
    pub token_bucket: ::core::option::Option<super::super::super::super::super::r#type::v3::TokenBucket>,
    /// Runtime flag that controls whether the filter is enabled or not. If not specified, defaults
    /// to enabled.
    #[prost(message, optional, tag="3")]
    pub runtime_enabled: ::core::option::Option<super::super::super::super::super::config::core::v3::RuntimeFeatureFlag>,
    /// Specifies that the token bucket used for rate limiting should be shared with other local_rate_limit filters
    /// with a matching :ref:`token_bucket <envoy_v3_api_field_extensions.filters.network.local_ratelimit.v3.LocalRateLimit.token_bucket>`
    /// and ``share_key`` configuration. All fields of ``token_bucket`` must match exactly for the token bucket to be shared. If this
    /// field is empty, this filter will not share a token bucket with any other filter.
    #[prost(string, tag="4")]
    pub share_key: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `envoy.extensions.filters.network.local_ratelimit.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb7, 0x17, 0x0a, 0x4a, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x6e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x61, 0x74, 0x65,
    0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2f, 0x76, 0x33, 0x2f, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72,
    0x61, 0x74, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x33, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72,
    0x6b, 0x2e, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x6c, 0x69, 0x6d, 0x69,
    0x74, 0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x74, 0x79, 0x70,
    0x65, 0x2f, 0x76, 0x33, 0x2f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x62, 0x75, 0x63, 0x6b, 0x65,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e,
    0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x21, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e,
    0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0xbf, 0x02, 0x0a, 0x0e, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x61, 0x74, 0x65,
    0x4c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x28, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x74, 0x5f, 0x70, 0x72,
    0x65, 0x66, 0x69, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x72,
    0x02, 0x10, 0x01, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x50, 0x72, 0x65, 0x66, 0x69, 0x78, 0x12,
    0x47, 0x0a, 0x0c, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x74, 0x79,
    0x70, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x42, 0x75, 0x63, 0x6b, 0x65,
    0x74, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x0b, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x51, 0x0a, 0x0f, 0x72, 0x75, 0x6e, 0x74,
    0x69, 0x6d, 0x65, 0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x28, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x52, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65,
    0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x46, 0x6c, 0x61, 0x67, 0x52, 0x0e, 0x72, 0x75, 0x6e,
    0x74, 0x69, 0x6d, 0x65, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x73,
    0x68, 0x61, 0x72, 0x65, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08,
    0x73, 0x68, 0x61, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x3a, 0x4a, 0x9a, 0xc5, 0x88, 0x1e, 0x45, 0x0a,
    0x43, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6c, 0x6f, 0x63,
    0x61, 0x6c, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x76, 0x32,
    0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x61, 0x74, 0x65, 0x4c,
    0x69, 0x6d, 0x69, 0x74, 0x42, 0xd0, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a,
    0x41, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e,
    0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e,
    0x76, 0x33, 0x42, 0x13, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d,
    0x69, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x6c, 0x67, 0x69, 0x74, 0x68, 0x75,
    0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e,
    0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f,
    0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x2f, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x6c, 0x69, 0x6d,
    0x69, 0x74, 0x2f, 0x76, 0x33, 0x3b, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x61, 0x74, 0x65,
    0x6c, 0x69, 0x6d, 0x69, 0x74, 0x76, 0x33, 0x4a, 0xf8, 0x10, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x35, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x3c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00,
    0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x2a, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08,
    0x00, 0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x09, 0x00, 0x21, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x5a, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0b,
    0x00, 0x5a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x34, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x34, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00,
    0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0d, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x01,
    0x08, 0x12, 0x04, 0x0e, 0x00, 0x83, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x04, 0x0e,
    0x00, 0x83, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0xc2, 0x01, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x15, 0x00, 0x35, 0x01, 0x32, 0xb5, 0x01, 0x20, 0x5b, 0x23, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x4c,
    0x6f, 0x63, 0x61, 0x6c, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5d,
    0x0a, 0x20, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d,
    0x69, 0x74, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x20,
    0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x61,
    0x74, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x3e, 0x60, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e,
    0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5d,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x15, 0x08, 0x16, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x16, 0x02, 0x17, 0x4c, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x00,
    0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x16, 0x02, 0x17, 0x4c, 0x0a, 0x72, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x02, 0x42, 0x1a, 0x65, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x77, 0x68,
    0x65, 0x6e, 0x20, 0x65, 0x6d, 0x69, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x3a, 0x72, 0x65, 0x66,
    0x3a, 0x60, 0x73, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x0a, 0x20, 0x3c, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x61, 0x74, 0x65,
    0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x73, 0x3e, 0x60, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x08, 0x12, 0x03, 0x1b, 0x19, 0x41, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00,
    0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x1b, 0x1a, 0x40, 0x0a, 0x9d, 0x06, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x55, 0x1a, 0x8f, 0x06, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74,
    0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x69,
    0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x61, 0x72, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x65,
    0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x27, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x2e,
    0x20, 0x45, 0x61, 0x63, 0x68, 0x20, 0x69, 0x6e, 0x63, 0x6f, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x63,
    0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73,
    0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x75, 0x6d, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e,
    0x67, 0x6c, 0x65, 0x0a, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x61, 0x76, 0x61, 0x69,
    0x6c, 0x61, 0x62, 0x6c, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x65, 0x64, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x20, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c,
    0x65, 0x2c, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6d, 0x6d, 0x65, 0x64,
    0x69, 0x61, 0x74, 0x65, 0x6c, 0x79, 0x20, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x64, 0x2e, 0x0a, 0x0a,
    0x20, 0x2e, 0x2e, 0x20, 0x6e, 0x6f, 0x74, 0x65, 0x3a, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x49, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6d, 0x70,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x61, 0x63, 0x68,
    0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x68, 0x61, 0x73, 0x20, 0x61, 0x6e, 0x20,
    0x69, 0x6e, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x61, 0x74, 0x65,
    0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2c, 0x20, 0x75, 0x6e, 0x6c, 0x65, 0x73, 0x73, 0x0a, 0x20,
    0x20, 0x20, 0x61, 0x20, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20,
    0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75,
    0x72, 0x65, 0x64, 0x20, 0x76, 0x69, 0x61, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x73, 0x68,
    0x61, 0x72, 0x65, 0x5f, 0x6b, 0x65, 0x79, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76,
    0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x61, 0x74,
    0x65, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x76, 0x33, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x52,
    0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x73, 0x68, 0x61, 0x72, 0x65, 0x5f, 0x6b,
    0x65, 0x79, 0x3e, 0x60, 0x2e, 0x0a, 0x0a, 0x20, 0x2e, 0x2e, 0x20, 0x6e, 0x6f, 0x74, 0x65, 0x3a,
    0x3a, 0x0a, 0x20, 0x20, 0x20, 0x49, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x74, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x62, 0x75, 0x63,
    0x6b, 0x65, 0x74, 0x27, 0x73, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x66, 0x69, 0x6c, 0x6c,
    0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x0a, 0x20, 0x20, 0x20, 0x3c, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x42, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x2e, 0x66, 0x69, 0x6c, 0x6c, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76,
    0x61, 0x6c, 0x3e, 0x60, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x3e, 0x3d, 0x20,
    0x35, 0x30, 0x6d, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x74, 0x6f,
    0x6f, 0x20, 0x61, 0x67, 0x67, 0x72, 0x65, 0x73, 0x73, 0x69, 0x76, 0x65, 0x0a, 0x20, 0x20, 0x20,
    0x72, 0x65, 0x66, 0x69, 0x6c, 0x6c, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x2a, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2a, 0x16, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2a, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x2a, 0x27,
    0x54, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x11, 0x12, 0x03, 0x2a,
    0x28, 0x53, 0x0a, 0x77, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2e, 0x02, 0x38, 0x1a,
    0x6a, 0x20, 0x52, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x66, 0x6c, 0x61, 0x67, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x20, 0x77, 0x68, 0x65,
    0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20,
    0x69, 0x73, 0x20, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f,
    0x74, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66,
    0x69, 0x65, 0x64, 0x2c, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x0a, 0x20, 0x74,
    0x6f, 0x20, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2e, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x2e, 0x24, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x2e, 0x36, 0x37, 0x0a, 0xd0, 0x03, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x34, 0x02, 0x17, 0x1a, 0xc2, 0x03, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20,
    0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x72, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x68,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f,
    0x72, 0x61, 0x74, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x73, 0x0a, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x69, 0x6e, 0x67, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x5f,
    0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33,
    0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x72, 0x61, 0x74, 0x65,
    0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x76, 0x33, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x61,
    0x74, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x3e, 0x60, 0x0a, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x60, 0x60, 0x73, 0x68,
    0x61, 0x72, 0x65, 0x5f, 0x6b, 0x65, 0x79, 0x60, 0x60, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x60, 0x60, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x62,
    0x75, 0x63, 0x6b, 0x65, 0x74, 0x60, 0x60, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x20, 0x65, 0x78, 0x61, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20,
    0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x49, 0x66,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x0a, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20,
    0x65, 0x6d, 0x70, 0x74, 0x79, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x68, 0x61, 0x72,
    0x65, 0x20, 0x61, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x34, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x34, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x34,
    0x15, 0x16, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.network.local_ratelimit.v3.serde.rs");
// @@protoc_insertion_point(module)