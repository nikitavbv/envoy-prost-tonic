// @generated
// [#protodoc-title: Rate limit]
// Rate limit :ref:`configuration overview <config_http_filters_rate_limit>`.
// [#extension: envoy.filters.http.ratelimit]

/// [#next-free-field: 8]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimit {
    /// The rate limit domain to use when calling the rate limit service.
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    /// Specifies the rate limit configurations to be applied with the same
    /// stage number. If not set, the default stage number is 0.
    ///
    /// .. note::
    ///
    ///   The filter supports a range of 0 - 10 inclusively for stage numbers.
    #[prost(uint32, tag="2")]
    pub stage: u32,
    /// The type of requests the filter should apply to. The supported
    /// types are *internal*, *external* or *both*. A request is considered internal if
    /// :ref:`x-envoy-internal<config_http_conn_man_headers_x-envoy-internal>` is set to true. If
    /// :ref:`x-envoy-internal<config_http_conn_man_headers_x-envoy-internal>` is not set or false, a
    /// request is considered external. The filter defaults to *both*, and it will apply to all request
    /// types.
    #[prost(string, tag="3")]
    pub request_type: ::prost::alloc::string::String,
    /// The timeout in milliseconds for the rate limit service RPC. If not
    /// set, this defaults to 20ms.
    #[prost(message, optional, tag="4")]
    pub timeout: ::core::option::Option<::pbjson_types::Duration>,
    /// The filter's behaviour in case the rate limiting service does
    /// not respond back. When it is set to true, Envoy will not allow traffic in case of
    /// communication failure between rate limiting service and the proxy.
    /// Defaults to false.
    #[prost(bool, tag="5")]
    pub failure_mode_deny: bool,
    /// Specifies whether a `RESOURCE_EXHAUSTED` gRPC code must be returned instead
    /// of the default `UNAVAILABLE` gRPC code for a rate limited gRPC call. The
    /// HTTP code will be 200 for a gRPC response.
    #[prost(bool, tag="6")]
    pub rate_limited_as_resource_exhausted: bool,
    /// Configuration for an external rate limit service provider. If not
    /// specified, any calls to the rate limit service will immediately return
    /// success.
    #[prost(message, optional, tag="7")]
    pub rate_limit_service: ::core::option::Option<super::super::super::super::ratelimit::v2::RateLimitServiceConfig>,
}
/// Encoded file descriptor set for the `envoy.config.filter.http.rate_limit.v2` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xc1, 0x19, 0x0a, 0x37, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x72, 0x61,
    0x74, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2f, 0x76, 0x32, 0x2f, 0x72, 0x61, 0x74, 0x65,
    0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x26, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69,
    0x74, 0x2e, 0x76, 0x32, 0x1a, 0x23, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x2f, 0x72, 0x61, 0x74, 0x65, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2f, 0x76, 0x32, 0x2f,
    0x72, 0x6c, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x75, 0x64, 0x70, 0x61, 0x2f,
    0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x69, 0x67, 0x72,
    0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f,
    0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0xa9, 0x03, 0x0a, 0x09, 0x52, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x12,
    0x1f, 0x0a, 0x06, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42,
    0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x20, 0x01, 0x52, 0x06, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e,
    0x12, 0x1d, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x42,
    0x07, 0xfa, 0x42, 0x04, 0x2a, 0x02, 0x18, 0x0a, 0x52, 0x05, 0x73, 0x74, 0x61, 0x67, 0x65, 0x12,
    0x44, 0x0a, 0x0c, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x09, 0x42, 0x21, 0xfa, 0x42, 0x1e, 0x72, 0x1c, 0x52, 0x08, 0x69, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x08, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x52, 0x04, 0x62, 0x6f, 0x74, 0x68, 0x52, 0x00, 0x52, 0x0b, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x33, 0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x52, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x12, 0x2a, 0x0a, 0x11, 0x66, 0x61,
    0x69, 0x6c, 0x75, 0x72, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x5f, 0x64, 0x65, 0x6e, 0x79, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x4d, 0x6f,
    0x64, 0x65, 0x44, 0x65, 0x6e, 0x79, 0x12, 0x4a, 0x0a, 0x22, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x5f, 0x65, 0x78, 0x68, 0x61, 0x75, 0x73, 0x74, 0x65, 0x64, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x1e, 0x72, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x41,
    0x73, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x45, 0x78, 0x68, 0x61, 0x75, 0x73, 0x74,
    0x65, 0x64, 0x12, 0x69, 0x0a, 0x12, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74,
    0x5f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x31,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x61,
    0x74, 0x65, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x76, 0x32, 0x2e, 0x52, 0x61, 0x74, 0x65, 0x4c,
    0x69, 0x6d, 0x69, 0x74, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x10, 0x72, 0x61, 0x74,
    0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x42, 0xde, 0x01,
    0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x2c, 0x12, 0x2a, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x61, 0x74, 0x65, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e,
    0x76, 0x33, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x01, 0x0a, 0x34, 0x69, 0x6f, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74,
    0x74, 0x70, 0x2e, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x76, 0x32,
    0x42, 0x0e, 0x52, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x50, 0x01, 0x5a, 0x5a, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x68,
    0x74, 0x74, 0x70, 0x2f, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2f, 0x76,
    0x32, 0x3b, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x76, 0x32, 0x4a, 0xab,
    0x12, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x42, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x2f, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x2d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x06, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x08, 0x00, 0x28, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x0a, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x4d, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0c, 0x00, 0x4d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0d, 0x00, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x2f, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03,
    0x0e, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x71, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0b, 0x12, 0x03, 0x0f, 0x00, 0x71, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04, 0x10,
    0x00, 0x11, 0x31, 0x0a, 0x0e, 0x0a, 0x06, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12, 0x04, 0x10,
    0x00, 0x11, 0x31, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x12, 0x00, 0x46, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x12, 0x00, 0x46, 0x0a, 0xbd, 0x01, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x19, 0x00, 0x42, 0x01, 0x1a, 0x17, 0x20, 0x5b, 0x23, 0x6e, 0x65,
    0x78, 0x74, 0x2d, 0x66, 0x72, 0x65, 0x65, 0x2d, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x3a, 0x20, 0x38,
    0x5d, 0x0a, 0x32, 0x97, 0x01, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63,
    0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x52, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d,
    0x69, 0x74, 0x5d, 0x0a, 0x20, 0x52, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20,
    0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x20, 0x3c, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x73, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x3e, 0x60, 0x2e, 0x0a,
    0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70,
    0x2e, 0x72, 0x61, 0x74, 0x65, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x19, 0x08, 0x11, 0x0a, 0x50, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x1b, 0x02, 0x3f, 0x1a, 0x43, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x61, 0x74, 0x65,
    0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x6f,
    0x20, 0x75, 0x73, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x69, 0x6e,
    0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1b, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1b, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x1b,
    0x14, 0x3e, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03,
    0x1b, 0x15, 0x3d, 0x0a, 0xe1, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x23, 0x02,
    0x39, 0x1a, 0xd3, 0x01, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x74, 0x6f, 0x20,
    0x62, 0x65, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x0a, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x20,
    0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73,
    0x65, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20,
    0x73, 0x74, 0x61, 0x67, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20,
    0x30, 0x2e, 0x0a, 0x0a, 0x20, 0x2e, 0x2e, 0x20, 0x6e, 0x6f, 0x74, 0x65, 0x3a, 0x3a, 0x0a, 0x0a,
    0x20, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x73, 0x75, 0x70,
    0x70, 0x6f, 0x72, 0x74, 0x73, 0x20, 0x61, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x30, 0x20, 0x2d, 0x20, 0x31, 0x30, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x73, 0x69, 0x76,
    0x65, 0x6c, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x20, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x23, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x23, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x11,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x23, 0x13, 0x38, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x05, 0x12, 0x03, 0x23, 0x14, 0x37,
    0x0a, 0xc3, 0x03, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x2b, 0x02, 0x2c, 0x54, 0x1a,
    0xb4, 0x03, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x20,
    0x74, 0x6f, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65,
    0x64, 0x0a, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x2a, 0x69, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2a, 0x2c, 0x20, 0x2a, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x2a, 0x20, 0x6f, 0x72, 0x20, 0x2a, 0x62, 0x6f, 0x74, 0x68, 0x2a, 0x2e, 0x20, 0x41,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x73,
    0x69, 0x64, 0x65, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20,
    0x69, 0x66, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x78, 0x2d, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2d, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x5f, 0x6d, 0x61, 0x6e, 0x5f,
    0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x5f, 0x78, 0x2d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2d,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x3e, 0x60, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2e, 0x20, 0x49, 0x66, 0x0a, 0x20, 0x3a,
    0x72, 0x65, 0x66, 0x3a, 0x60, 0x78, 0x2d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2d, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74,
    0x70, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x5f, 0x6d, 0x61, 0x6e, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x73, 0x5f, 0x78, 0x2d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2d, 0x69, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x3e, 0x60, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74,
    0x20, 0x6f, 0x72, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x2c, 0x20, 0x61, 0x0a, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x64, 0x65,
    0x72, 0x65, 0x64, 0x20, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x2a, 0x62, 0x6f, 0x74, 0x68, 0x2a, 0x2c, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x69, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x20, 0x74,
    0x6f, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x2b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b,
    0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x18, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x2c, 0x06, 0x53, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x02, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x2c, 0x07, 0x52, 0x0a,
    0x6e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x30, 0x02, 0x27, 0x1a, 0x61, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x69,
    0x6c, 0x6c, 0x69, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x52, 0x50, 0x43, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f,
    0x74, 0x0a, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x64, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x32, 0x30, 0x6d, 0x73, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x30, 0x02, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x30, 0x1b, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x30, 0x25, 0x26, 0x0a, 0xf8, 0x01, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x36, 0x02, 0x1d, 0x1a, 0xea, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x27, 0x73, 0x20, 0x62, 0x65, 0x68, 0x61, 0x76, 0x69, 0x6f, 0x75,
    0x72, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x61, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61,
    0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x0a, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x64, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x2e, 0x20, 0x57, 0x68, 0x65, 0x6e,
    0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x72,
    0x75, 0x65, 0x2c, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6e,
    0x6f, 0x74, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x74, 0x72, 0x61, 0x66, 0x66, 0x69, 0x63,
    0x20, 0x69, 0x6e, 0x20, 0x63, 0x61, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x75, 0x6e, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75,
    0x72, 0x65, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20,
    0x6c, 0x69, 0x6d, 0x69, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x0a,
    0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x61, 0x6c,
    0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x36,
    0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x36, 0x07, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x36, 0x1b, 0x1c, 0x0a, 0xd1,
    0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x3b, 0x02, 0x2e, 0x1a, 0xc3, 0x01, 0x20,
    0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x61, 0x20, 0x60, 0x52, 0x45, 0x53, 0x4f, 0x55, 0x52, 0x43, 0x45, 0x5f, 0x45, 0x58,
    0x48, 0x41, 0x55, 0x53, 0x54, 0x45, 0x44, 0x60, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x63, 0x6f,
    0x64, 0x65, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72,
    0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x61, 0x64, 0x0a, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x60, 0x55, 0x4e, 0x41,
    0x56, 0x41, 0x49, 0x4c, 0x41, 0x42, 0x4c, 0x45, 0x60, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x63,
    0x6f, 0x64, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x63, 0x61, 0x6c, 0x6c,
    0x2e, 0x20, 0x54, 0x68, 0x65, 0x0a, 0x20, 0x48, 0x54, 0x54, 0x50, 0x20, 0x63, 0x6f, 0x64, 0x65,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x32, 0x30, 0x30, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x61, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x3b, 0x02, 0x06,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x3b, 0x07, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x3b, 0x2c, 0x2d, 0x0a, 0xa4, 0x01, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x04, 0x40, 0x02, 0x41, 0x34, 0x1a, 0x95, 0x01, 0x20, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x61, 0x74,
    0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20,
    0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x74,
    0x0a, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2c, 0x20, 0x61, 0x6e, 0x79,
    0x20, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61,
    0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x69, 0x6d, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x6c,
    0x79, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x0a, 0x20, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x40, 0x02,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x40, 0x26, 0x38, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x40, 0x3b, 0x3c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x08, 0x12, 0x03, 0x41, 0x06, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x00, 0x02, 0x06, 0x08, 0xaf, 0x08, 0x11, 0x12, 0x03, 0x41, 0x07, 0x32, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.config.filter.http.rate_limit.v2.serde.rs");
// @@protoc_insertion_point(module)