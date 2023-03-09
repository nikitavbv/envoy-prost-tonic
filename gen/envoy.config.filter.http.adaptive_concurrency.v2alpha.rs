// @generated
// [#protodoc-title: Adaptive Concurrency]
// Adaptive Concurrency Control :ref:`configuration overview
// <config_http_filters_adaptive_concurrency>`.
// [#extension: envoy.filters.http.adaptive_concurrency]

/// Configuration parameters for the gradient controller.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GradientControllerConfig {
    /// The percentile to use when summarizing aggregated samples. Defaults to p50.
    #[prost(message, optional, tag="1")]
    pub sample_aggregate_percentile: ::core::option::Option<super::super::super::super::super::r#type::Percent>,
    #[prost(message, optional, tag="2")]
    pub concurrency_limit_params: ::core::option::Option<gradient_controller_config::ConcurrencyLimitCalculationParams>,
    #[prost(message, optional, tag="3")]
    pub min_rtt_calc_params: ::core::option::Option<gradient_controller_config::MinimumRttCalculationParams>,
}
/// Nested message and enum types in `GradientControllerConfig`.
pub mod gradient_controller_config {
    /// Parameters controlling the periodic recalculation of the concurrency limit from sampled request
    /// latencies.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConcurrencyLimitCalculationParams {
        /// The allowed upper-bound on the calculated concurrency limit. Defaults to 1000.
        #[prost(message, optional, tag="2")]
        pub max_concurrency_limit: ::core::option::Option<::pbjson_types::UInt32Value>,
        /// The period of time samples are taken to recalculate the concurrency limit.
        #[prost(message, optional, tag="3")]
        pub concurrency_update_interval: ::core::option::Option<::pbjson_types::Duration>,
    }
    /// Parameters controlling the periodic minRTT recalculation.
    /// [#next-free-field: 6]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MinimumRttCalculationParams {
        /// The time interval between recalculating the minimum request round-trip time.
        #[prost(message, optional, tag="1")]
        pub interval: ::core::option::Option<::pbjson_types::Duration>,
        /// The number of requests to aggregate/sample during the minRTT recalculation window before
        /// updating. Defaults to 50.
        #[prost(message, optional, tag="2")]
        pub request_count: ::core::option::Option<::pbjson_types::UInt32Value>,
        /// Randomized time delta that will be introduced to the start of the minRTT calculation window.
        /// This is represented as a percentage of the interval duration. Defaults to 15%.
        ///
        /// Example: If the interval is 10s and the jitter is 15%, the next window will begin
        /// somewhere in the range (10s - 11.5s).
        #[prost(message, optional, tag="3")]
        pub jitter: ::core::option::Option<super::super::super::super::super::super::r#type::Percent>,
        /// The concurrency limit set while measuring the minRTT. Defaults to 3.
        #[prost(message, optional, tag="4")]
        pub min_concurrency: ::core::option::Option<::pbjson_types::UInt32Value>,
        /// Amount added to the measured minRTT to add stability to the concurrency limit during natural
        /// variability in latency. This is expressed as a percentage of the measured value and can be
        /// adjusted to allow more or less tolerance to the sampled latency values.
        ///
        /// Defaults to 25%.
        #[prost(message, optional, tag="5")]
        pub buffer: ::core::option::Option<super::super::super::super::super::super::r#type::Percent>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdaptiveConcurrency {
    /// If set to false, the adaptive concurrency filter will operate as a pass-through filter. If the
    /// message is unspecified, the filter will be enabled.
    #[prost(message, optional, tag="2")]
    pub enabled: ::core::option::Option<super::super::super::super::super::api::v2::core::RuntimeFeatureFlag>,
    #[prost(oneof="adaptive_concurrency::ConcurrencyControllerConfig", tags="1")]
    pub concurrency_controller_config: ::core::option::Option<adaptive_concurrency::ConcurrencyControllerConfig>,
}
/// Nested message and enum types in `AdaptiveConcurrency`.
pub mod adaptive_concurrency {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConcurrencyControllerConfig {
        /// Gradient concurrency control will be used.
        #[prost(message, tag="1")]
        GradientControllerConfig(super::GradientControllerConfig),
    }
}
/// Encoded file descriptor set for the `envoy.config.filter.http.adaptive_concurrency.v2alpha` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xe1, 0x27, 0x0a, 0x50, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x61, 0x64,
    0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x63, 0x79, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2f, 0x61, 0x64, 0x61, 0x70, 0x74,
    0x69, 0x76, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x35, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e,
    0x61, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x63, 0x79, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x1a, 0x1c, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x32, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f,
    0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x18, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x8f, 0x08, 0x0a,
    0x18, 0x47, 0x72, 0x61, 0x64, 0x69, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x6c, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x53, 0x0a, 0x1b, 0x73, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x5f, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x65,
    0x72, 0x63, 0x65, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x50, 0x65, 0x72, 0x63,
    0x65, 0x6e, 0x74, 0x52, 0x19, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x41, 0x67, 0x67, 0x72, 0x65,
    0x67, 0x61, 0x74, 0x65, 0x50, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x12, 0xb5,
    0x01, 0x0a, 0x18, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x71, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x61, 0x64, 0x61,
    0x70, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63,
    0x79, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x47, 0x72, 0x61, 0x64, 0x69, 0x65,
    0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x2e, 0x43, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x4c, 0x69,
    0x6d, 0x69, 0x74, 0x43, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x61,
    0x72, 0x61, 0x6d, 0x73, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x16,
    0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x4c, 0x69, 0x6d, 0x69, 0x74,
    0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x12, 0xa4, 0x01, 0x0a, 0x13, 0x6d, 0x69, 0x6e, 0x5f, 0x72,
    0x74, 0x74, 0x5f, 0x63, 0x61, 0x6c, 0x63, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x6b, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e,
    0x61, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x63, 0x79, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x47, 0x72, 0x61,
    0x64, 0x69, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x52, 0x54, 0x54,
    0x43, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x61, 0x72, 0x61, 0x6d,
    0x73, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x10, 0x6d, 0x69, 0x6e,
    0x52, 0x74, 0x74, 0x43, 0x61, 0x6c, 0x63, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x1a, 0xe5, 0x01,
    0x0a, 0x21, 0x43, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x4c, 0x69, 0x6d,
    0x69, 0x74, 0x43, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x61, 0x72,
    0x61, 0x6d, 0x73, 0x12, 0x59, 0x0a, 0x15, 0x6d, 0x61, 0x78, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2e, 0x55, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65,
    0x42, 0x07, 0xfa, 0x42, 0x04, 0x2a, 0x02, 0x20, 0x00, 0x52, 0x13, 0x6d, 0x61, 0x78, 0x43, 0x6f,
    0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x65,
    0x0a, 0x1b, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x0a,
    0xfa, 0x42, 0x07, 0xaa, 0x01, 0x04, 0x08, 0x01, 0x2a, 0x00, 0x52, 0x19, 0x63, 0x6f, 0x6e, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x76, 0x61, 0x6c, 0x1a, 0xd6, 0x02, 0x0a, 0x1b, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x75,
    0x6d, 0x52, 0x54, 0x54, 0x43, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50,
    0x61, 0x72, 0x61, 0x6d, 0x73, 0x12, 0x41, 0x0a, 0x08, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61,
    0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x42, 0x0a, 0xfa, 0x42, 0x07, 0xaa, 0x01, 0x04, 0x08, 0x01, 0x2a, 0x00, 0x52, 0x08,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x12, 0x4a, 0x0a, 0x0d, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2e, 0x55, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x07, 0xfa,
    0x42, 0x04, 0x2a, 0x02, 0x20, 0x00, 0x52, 0x0c, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x43,
    0x6f, 0x75, 0x6e, 0x74, 0x12, 0x2b, 0x0a, 0x06, 0x6a, 0x69, 0x74, 0x74, 0x65, 0x72, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x74, 0x79, 0x70,
    0x65, 0x2e, 0x50, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x52, 0x06, 0x6a, 0x69, 0x74, 0x74, 0x65,
    0x72, 0x12, 0x4e, 0x0a, 0x0f, 0x6d, 0x69, 0x6e, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x63, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x55, 0x49, 0x6e,
    0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x2a, 0x02, 0x20,
    0x00, 0x52, 0x0e, 0x6d, 0x69, 0x6e, 0x43, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63,
    0x79, 0x12, 0x2b, 0x0a, 0x06, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x13, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x50,
    0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x52, 0x06, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x22, 0x98,
    0x02, 0x0a, 0x13, 0x41, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x43, 0x6f, 0x6e, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x12, 0x99, 0x01, 0x0a, 0x1a, 0x67, 0x72, 0x61, 0x64, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x5f, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x4f, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x61, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x5f,
    0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x2e, 0x76, 0x32, 0x61, 0x6c,
    0x70, 0x68, 0x61, 0x2e, 0x47, 0x72, 0x61, 0x64, 0x69, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x42, 0x08, 0xfa, 0x42,
    0x05, 0x8a, 0x01, 0x02, 0x10, 0x01, 0x48, 0x00, 0x52, 0x18, 0x67, 0x72, 0x61, 0x64, 0x69, 0x65,
    0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x12, 0x3f, 0x0a, 0x07, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e,
    0x76, 0x32, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x52, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x46,
    0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x46, 0x6c, 0x61, 0x67, 0x52, 0x07, 0x65, 0x6e, 0x61, 0x62,
    0x6c, 0x65, 0x64, 0x42, 0x24, 0x0a, 0x1d, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x63, 0x79, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x5f, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x12, 0x03, 0xf8, 0x42, 0x01, 0x42, 0x84, 0x02, 0xf2, 0x98, 0xfe, 0x8f,
    0x05, 0x37, 0x12, 0x35, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74,
    0x70, 0x2e, 0x61, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x2e, 0x76, 0x33, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10,
    0x01, 0x0a, 0x43, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x61, 0x64, 0x61, 0x70, 0x74, 0x69,
    0x76, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x2e, 0x76,
    0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x42, 0x18, 0x41, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65,
    0x43, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x50, 0x01, 0x5a, 0x5c, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x68,
    0x74, 0x74, 0x70, 0x2f, 0x61, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x63, 0x6f, 0x6e,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61,
    0x4a, 0xc9, 0x18, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x5e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x3e, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x05, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x28,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x0a, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0b, 0x00,
    0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0c, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0e, 0x00, 0x5c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0e, 0x00,
    0x5c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x39, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x08, 0x12, 0x03, 0x0f, 0x00, 0x39, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x10, 0x00, 0x22,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x10, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x11, 0x00, 0x73, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x11, 0x00, 0x73,
    0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04, 0x12, 0x00, 0x13, 0x3c, 0x0a, 0x0e, 0x0a, 0x06, 0x08,
    0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12, 0x04, 0x12, 0x00, 0x13, 0x3c, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x14, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12,
    0x03, 0x14, 0x00, 0x46, 0x0a, 0x8f, 0x02, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1c, 0x00, 0x50,
    0x01, 0x1a, 0x37, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x72, 0x61, 0x64, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x0a, 0x32, 0xc9, 0x01, 0x20, 0x5b, 0x23,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20,
    0x41, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x20, 0x43, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x63, 0x79, 0x5d, 0x0a, 0x20, 0x41, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x20,
    0x43, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x43, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77,
    0x0a, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x61, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x5f,
    0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x3e, 0x60, 0x2e, 0x0a, 0x20,
    0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e,
    0x61, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x63, 0x79, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1c,
    0x08, 0x20, 0x0a, 0x7b, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x1f, 0x02, 0x28, 0x03,
    0x1a, 0x6d, 0x20, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x65,
    0x72, 0x69, 0x6f, 0x64, 0x69, 0x63, 0x20, 0x72, 0x65, 0x63, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x66, 0x72,
    0x6f, 0x6d, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x64, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x0a, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x0a, 0x2b, 0x0a, 0x5f, 0x0a,
    0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x21, 0x04, 0x5e, 0x1a, 0x50, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x64, 0x20, 0x75, 0x70, 0x70, 0x65, 0x72,
    0x2d, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61,
    0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x63, 0x79, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61,
    0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x31, 0x30, 0x30, 0x30, 0x2e, 0x0a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x21, 0x04, 0x1f, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x20, 0x35, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x38, 0x39, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x21, 0x3a, 0x5d, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x05, 0x12, 0x03, 0x21, 0x3b,
    0x5c, 0x0a, 0x5c, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x24, 0x04, 0x27,
    0x07, 0x1a, 0x4c, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x20, 0x61,
    0x72, 0x65, 0x20, 0x74, 0x61, 0x6b, 0x65, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x63, 0x61,
    0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x24, 0x04, 0x1c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x24, 0x1d, 0x38, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x24, 0x3b, 0x3c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x08, 0x12, 0x04, 0x24, 0x3d, 0x27, 0x06,
    0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x15, 0x12, 0x04,
    0x24, 0x3e, 0x27, 0x05, 0x0a, 0x60, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x01, 0x12, 0x04, 0x2c, 0x02,
    0x47, 0x03, 0x1a, 0x52, 0x20, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x69, 0x63, 0x20, 0x6d, 0x69, 0x6e, 0x52, 0x54, 0x54, 0x20,
    0x72, 0x65, 0x63, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20,
    0x5b, 0x23, 0x6e, 0x65, 0x78, 0x74, 0x2d, 0x66, 0x72, 0x65, 0x65, 0x2d, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x3a, 0x20, 0x36, 0x5d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x01, 0x01, 0x12,
    0x03, 0x2c, 0x0a, 0x25, 0x0a, 0x5e, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x12, 0x04,
    0x2e, 0x04, 0x31, 0x07, 0x1a, 0x4e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e,
    0x20, 0x72, 0x65, 0x63, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x2d, 0x74, 0x72, 0x69, 0x70, 0x20, 0x74, 0x69,
    0x6d, 0x65, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x2e, 0x04, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x2e, 0x1d, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x2e, 0x28, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x08, 0x12,
    0x04, 0x2e, 0x2a, 0x31, 0x06, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x08,
    0xaf, 0x08, 0x15, 0x12, 0x04, 0x2e, 0x2b, 0x31, 0x05, 0x0a, 0x84, 0x01, 0x0a, 0x06, 0x04, 0x00,
    0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x35, 0x04, 0x56, 0x1a, 0x75, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x2f,
    0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x64, 0x75, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6d, 0x69, 0x6e, 0x52, 0x54, 0x54, 0x20, 0x72, 0x65, 0x63, 0x61, 0x6c, 0x63, 0x75,
    0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x20, 0x62, 0x65,
    0x66, 0x6f, 0x72, 0x65, 0x0a, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x2e, 0x20,
    0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x35, 0x30, 0x2e, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x35, 0x04, 0x1f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x35, 0x20, 0x2d,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x35, 0x30, 0x31,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x08, 0x12, 0x03, 0x35, 0x32, 0x55,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x05, 0x12, 0x03,
    0x35, 0x33, 0x54, 0x0a, 0xb9, 0x02, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x3c, 0x04, 0x1c, 0x1a, 0xa9, 0x02, 0x20, 0x52, 0x61, 0x6e, 0x64, 0x6f, 0x6d, 0x69, 0x7a, 0x65,
    0x64, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x64,
    0x75, 0x63, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x69, 0x6e, 0x52, 0x54, 0x54, 0x20,
    0x63, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x6e, 0x64,
    0x6f, 0x77, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x70,
    0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x70, 0x65,
    0x72, 0x63, 0x65, 0x6e, 0x74, 0x61, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x20, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x31,
    0x35, 0x25, 0x2e, 0x0a, 0x0a, 0x20, 0x45, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x3a, 0x20, 0x49,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x20, 0x69,
    0x73, 0x20, 0x31, 0x30, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6a, 0x69,
    0x74, 0x74, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x31, 0x35, 0x25, 0x2c, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x0a, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x77, 0x68, 0x65,
    0x72, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20,
    0x28, 0x31, 0x30, 0x73, 0x20, 0x2d, 0x20, 0x31, 0x31, 0x2e, 0x35, 0x73, 0x29, 0x2e, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x3c, 0x04, 0x10, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x11, 0x17, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3c, 0x1a, 0x1b, 0x0a,
    0x55, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03, 0x12, 0x03, 0x3f, 0x04, 0x58, 0x1a, 0x46,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79,
    0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77, 0x68, 0x69, 0x6c, 0x65,
    0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d,
    0x69, 0x6e, 0x52, 0x54, 0x54, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x33, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x3f, 0x04, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x3f, 0x20, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x3f, 0x32, 0x33, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03,
    0x08, 0x12, 0x03, 0x3f, 0x34, 0x57, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03,
    0x08, 0xaf, 0x08, 0x05, 0x12, 0x03, 0x3f, 0x35, 0x56, 0x0a, 0xa6, 0x02, 0x0a, 0x06, 0x04, 0x00,
    0x03, 0x01, 0x02, 0x04, 0x12, 0x03, 0x46, 0x04, 0x1c, 0x1a, 0x96, 0x02, 0x20, 0x41, 0x6d, 0x6f,
    0x75, 0x6e, 0x74, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x64, 0x20, 0x6d, 0x69, 0x6e, 0x52, 0x54, 0x54,
    0x20, 0x74, 0x6f, 0x20, 0x61, 0x64, 0x64, 0x20, 0x73, 0x74, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74,
    0x79, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x63, 0x79, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x64, 0x75, 0x72, 0x69, 0x6e,
    0x67, 0x20, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x61, 0x6c, 0x0a, 0x20, 0x76, 0x61, 0x72, 0x69, 0x61,
    0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63,
    0x79, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e,
    0x74, 0x61, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x61, 0x73,
    0x75, 0x72, 0x65, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x63,
    0x61, 0x6e, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x61, 0x64, 0x6a, 0x75, 0x73, 0x74, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x6f, 0x72,
    0x20, 0x6c, 0x65, 0x73, 0x73, 0x20, 0x74, 0x6f, 0x6c, 0x65, 0x72, 0x61, 0x6e, 0x63, 0x65, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x64, 0x20, 0x6c,
    0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x2e, 0x0a, 0x0a,
    0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x32, 0x35, 0x25,
    0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x06, 0x12, 0x03, 0x46,
    0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x46,
    0x11, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x46,
    0x1a, 0x1b, 0x0a, 0x5a, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x02, 0x2f, 0x1a,
    0x4d, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x69, 0x6c, 0x65,
    0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x73, 0x75, 0x6d,
    0x6d, 0x61, 0x72, 0x69, 0x7a, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61,
    0x74, 0x65, 0x64, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x20, 0x44, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x35, 0x30, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4a, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4a, 0x0f, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x4a, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x04, 0x4c, 0x02, 0x4d, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x4c, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4c,
    0x24, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4c, 0x3f, 0x40,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x4d, 0x06, 0x33, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x11, 0x12, 0x03, 0x4d, 0x07, 0x32, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x4f, 0x02, 0x64, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x4f, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x1e, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x4f, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12,
    0x03, 0x4f, 0x36, 0x63, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x02, 0x08, 0xaf, 0x08, 0x11,
    0x12, 0x03, 0x4f, 0x37, 0x62, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x52, 0x00, 0x5e,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x52, 0x08, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x01, 0x08, 0x00, 0x12, 0x04, 0x53, 0x02, 0x59, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x08, 0x00, 0x01, 0x12, 0x03, 0x53, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08,
    0x00, 0x02, 0x12, 0x03, 0x54, 0x04, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x08, 0x00, 0x02,
    0xaf, 0x08, 0x12, 0x03, 0x54, 0x04, 0x26, 0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x04, 0x57, 0x04, 0x58, 0x36, 0x1a, 0x2c, 0x20, 0x47, 0x72, 0x61, 0x64, 0x69, 0x65, 0x6e, 0x74,
    0x20, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x57, 0x04,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x1d, 0x37, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x57, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x08, 0x12, 0x03, 0x58, 0x08, 0x35, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x01, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x11, 0x12, 0x03, 0x58, 0x09, 0x34, 0x0a, 0xa3, 0x01, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x5d, 0x02, 0x2d, 0x1a, 0x95, 0x01, 0x20, 0x49, 0x66,
    0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x2c, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x61, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x65, 0x20, 0x61, 0x73, 0x20, 0x61,
    0x20, 0x70, 0x61, 0x73, 0x73, 0x2d, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x20, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x75, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x69,
    0x66, 0x69, 0x65, 0x64, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x5d, 0x02, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5d, 0x21, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5d, 0x2b, 0x2c, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)