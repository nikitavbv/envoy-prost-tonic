// @generated
// [#protodoc-title: Client-Side Weighted Round Robin Load Balancing Policy]
// \[#not-implemented-hide:\]

/// Configuration for the client_side_weighted_round_robin LB policy.
///
/// This policy differs from the built-in ROUND_ROBIN policy in terms of
/// how the endpoint weights are determined. In the ROUND_ROBIN policy,
/// the endpoint weights are sent by the control plane via EDS. However,
/// in this policy, the endpoint weights are instead determined via
/// qps and CPU utilization metrics sent by the endpoint using the Open
/// Request Cost Aggregation (ORCA) protocol. The weight of a given endpoint
/// is computed as qps / cpu_utilization.
///
/// See the :ref:`load balancing architecture overview<arch_overview_load_balancing_types>` for more information.
///
/// [#next-free-field: 6]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientSideWeightedRoundRobin {
    /// Whether to enable out-of-band utilization reporting collection from
    /// the endpoints. By default, per-request utilization reporting is used.
    #[prost(message, optional, tag="1")]
    pub enable_oob_load_report: ::core::option::Option<::pbjson_types::BoolValue>,
    /// Load reporting interval to request from the server. Note that the
    /// server may not provide reports as frequently as the client requests.
    /// Used only when enable_oob_load_report is true. Default is 10 seconds.
    #[prost(message, optional, tag="2")]
    pub oob_reporting_period: ::core::option::Option<::pbjson_types::Duration>,
    /// A given endpoint must report load metrics continuously for at least
    /// this long before the endpoint weight will be used. This avoids
    /// churn when the set of endpoint addresses changes. Takes effect
    /// both immediately after we establish a connection to an endpoint and
    /// after weight_expiration_period has caused us to stop using the most
    /// recent load metrics. Default is 10 seconds.
    #[prost(message, optional, tag="3")]
    pub blackout_period: ::core::option::Option<::pbjson_types::Duration>,
    /// If a given endpoint has not reported load metrics in this long,
    /// then we stop using the reported weight. This ensures that we do
    /// not continue to use very stale weights. Once we stop using a stale
    /// value, if we later start seeing fresh reports again, the
    /// blackout_period applies. Defaults to 3 minutes.
    #[prost(message, optional, tag="4")]
    pub weight_expiration_period: ::core::option::Option<::pbjson_types::Duration>,
    /// How often endpoint weights are recalculated. Default is 1 second.
    #[prost(message, optional, tag="5")]
    pub weight_update_period: ::core::option::Option<::pbjson_types::Duration>,
}
/// Encoded file descriptor set for the `envoy.extensions.load_balancing_policies.client_side_weighted_round_robin.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xab, 0x1a, 0x0a, 0x73, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x62, 0x61, 0x6c, 0x61, 0x6e,
    0x63, 0x69, 0x6e, 0x67, 0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x69, 0x65, 0x73, 0x2f, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x69, 0x64, 0x65, 0x5f, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74,
    0x65, 0x64, 0x5f, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x72, 0x6f, 0x62, 0x69, 0x6e, 0x2f, 0x76,
    0x33, 0x2f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x69, 0x64, 0x65, 0x5f, 0x77, 0x65,
    0x69, 0x67, 0x68, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x72, 0x6f, 0x62,
    0x69, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x4c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6c, 0x6f, 0x61, 0x64, 0x5f,
    0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x69, 0x6e, 0x67, 0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x69,
    0x65, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x69, 0x64, 0x65, 0x5f, 0x77,
    0x65, 0x69, 0x67, 0x68, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x72, 0x6f,
    0x62, 0x69, 0x6e, 0x2e, 0x76, 0x33, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e,
    0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa2, 0x03, 0x0a, 0x1c, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x53, 0x69, 0x64, 0x65, 0x57, 0x65, 0x69, 0x67, 0x68, 0x74, 0x65, 0x64, 0x52, 0x6f, 0x75, 0x6e,
    0x64, 0x52, 0x6f, 0x62, 0x69, 0x6e, 0x12, 0x4f, 0x0a, 0x16, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65,
    0x5f, 0x6f, 0x6f, 0x62, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x42, 0x6f, 0x6f, 0x6c, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x52, 0x13, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x4f, 0x6f, 0x62, 0x4c, 0x6f, 0x61,
    0x64, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x4b, 0x0a, 0x14, 0x6f, 0x6f, 0x62, 0x5f, 0x72,
    0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x12, 0x6f, 0x6f, 0x62, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x50, 0x65,
    0x72, 0x69, 0x6f, 0x64, 0x12, 0x42, 0x0a, 0x0f, 0x62, 0x6c, 0x61, 0x63, 0x6b, 0x6f, 0x75, 0x74,
    0x5f, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e,
    0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e,
    0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0e, 0x62, 0x6c, 0x61, 0x63, 0x6b, 0x6f,
    0x75, 0x74, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x12, 0x53, 0x0a, 0x18, 0x77, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x5f, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x65,
    0x72, 0x69, 0x6f, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x16, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x45, 0x78, 0x70,
    0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x12, 0x4b, 0x0a,
    0x14, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x70,
    0x65, 0x72, 0x69, 0x6f, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x12, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x55, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x42, 0xa2, 0x02, 0xba, 0x80, 0xc8,
    0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x5a, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70,
    0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x62, 0x61, 0x6c, 0x61, 0x6e,
    0x63, 0x69, 0x6e, 0x67, 0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x69, 0x65, 0x73, 0x2e, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x69, 0x64, 0x65, 0x5f, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74,
    0x65, 0x64, 0x5f, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x72, 0x6f, 0x62, 0x69, 0x6e, 0x2e, 0x76,
    0x33, 0x42, 0x21, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x69, 0x64, 0x65, 0x57, 0x65, 0x69,
    0x67, 0x68, 0x74, 0x65, 0x64, 0x52, 0x6f, 0x75, 0x6e, 0x64, 0x52, 0x6f, 0x62, 0x69, 0x6e, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x96, 0x01, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e,
    0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67,
    0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x69, 0x6e, 0x67, 0x5f,
    0x70, 0x6f, 0x6c, 0x69, 0x63, 0x69, 0x65, 0x73, 0x2f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f,
    0x73, 0x69, 0x64, 0x65, 0x5f, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x6f,
    0x75, 0x6e, 0x64, 0x5f, 0x72, 0x6f, 0x62, 0x69, 0x6e, 0x2f, 0x76, 0x33, 0x3b, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x73, 0x69, 0x64, 0x65, 0x5f, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x65,
    0x64, 0x5f, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x72, 0x6f, 0x62, 0x69, 0x6e, 0x76, 0x33, 0x4a,
    0xb4, 0x12, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x3a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x55, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x05, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x27, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x73, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x09, 0x00, 0x73, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x42, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x42, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09,
    0x0a, 0x01, 0x08, 0x12, 0x04, 0x0c, 0x00, 0xad, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x08, 0x0b, 0x12,
    0x04, 0x0c, 0x00, 0xad, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x88,
    0x06, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1f, 0x00, 0x3a, 0x01, 0x1a, 0x94, 0x05, 0x20, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x69, 0x64, 0x65,
    0x5f, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x5f,
    0x72, 0x6f, 0x62, 0x69, 0x6e, 0x20, 0x4c, 0x42, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x2e,
    0x0a, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x64,
    0x69, 0x66, 0x66, 0x65, 0x72, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x62, 0x75, 0x69, 0x6c, 0x74, 0x2d, 0x69, 0x6e, 0x20, 0x52, 0x4f, 0x55, 0x4e, 0x44, 0x5f, 0x52,
    0x4f, 0x42, 0x49, 0x4e, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x74,
    0x65, 0x72, 0x6d, 0x73, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74,
    0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x64,
    0x2e, 0x20, 0x49, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x52, 0x4f, 0x55, 0x4e, 0x44, 0x5f, 0x52,
    0x4f, 0x42, 0x49, 0x4e, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x2c, 0x0a, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x77, 0x65, 0x69, 0x67, 0x68,
    0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x79, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x65,
    0x20, 0x76, 0x69, 0x61, 0x20, 0x45, 0x44, 0x53, 0x2e, 0x20, 0x48, 0x6f, 0x77, 0x65, 0x76, 0x65,
    0x72, 0x2c, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x6f, 0x6c, 0x69,
    0x63, 0x79, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74,
    0x20, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x73,
    0x74, 0x65, 0x61, 0x64, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x64, 0x20,
    0x76, 0x69, 0x61, 0x0a, 0x20, 0x71, 0x70, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x43, 0x50, 0x55,
    0x20, 0x75, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6d, 0x65, 0x74,
    0x72, 0x69, 0x63, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x4f, 0x70, 0x65, 0x6e, 0x0a, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x43, 0x6f, 0x73, 0x74, 0x20, 0x41, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x28, 0x4f, 0x52, 0x43, 0x41, 0x29, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69,
    0x6e, 0x74, 0x0a, 0x20, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x75, 0x74, 0x65, 0x64, 0x20,
    0x61, 0x73, 0x20, 0x71, 0x70, 0x73, 0x20, 0x2f, 0x20, 0x63, 0x70, 0x75, 0x5f, 0x75, 0x74, 0x69,
    0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x20, 0x53, 0x65, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x62,
    0x61, 0x6c, 0x61, 0x6e, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69, 0x74, 0x65,
    0x63, 0x74, 0x75, 0x72, 0x65, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x3c, 0x61,
    0x72, 0x63, 0x68, 0x5f, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x5f, 0x6c, 0x6f, 0x61,
    0x64, 0x5f, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x73, 0x3e, 0x60, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x20, 0x5b, 0x23, 0x6e, 0x65,
    0x78, 0x74, 0x2d, 0x66, 0x72, 0x65, 0x65, 0x2d, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x3a, 0x20, 0x36,
    0x5d, 0x0a, 0x32, 0x65, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d,
    0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2d, 0x53, 0x69,
    0x64, 0x65, 0x20, 0x57, 0x65, 0x69, 0x67, 0x68, 0x74, 0x65, 0x64, 0x20, 0x52, 0x6f, 0x75, 0x6e,
    0x64, 0x20, 0x52, 0x6f, 0x62, 0x69, 0x6e, 0x20, 0x4c, 0x6f, 0x61, 0x64, 0x20, 0x42, 0x61, 0x6c,
    0x61, 0x6e, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x5d, 0x0a, 0x20,
    0x5b, 0x23, 0x6e, 0x6f, 0x74, 0x2d, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65,
    0x64, 0x2d, 0x68, 0x69, 0x64, 0x65, 0x3a, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x1f, 0x08, 0x24, 0x0a, 0x9a, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x22, 0x02, 0x37, 0x1a, 0x8c, 0x01, 0x20, 0x57, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74,
    0x6f, 0x20, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6f, 0x75, 0x74, 0x2d, 0x6f, 0x66, 0x2d,
    0x62, 0x61, 0x6e, 0x64, 0x20, 0x75, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x73, 0x2e, 0x20, 0x42, 0x79, 0x20, 0x64, 0x65,
    0x66, 0x61, 0x75, 0x6c, 0x74, 0x2c, 0x20, 0x70, 0x65, 0x72, 0x2d, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x75, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x72,
    0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x22, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x1c, 0x32, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x35, 0x36, 0x0a, 0xde, 0x01, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x27, 0x02, 0x34, 0x1a, 0xd0, 0x01, 0x20, 0x4c, 0x6f,
    0x61, 0x64, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x76, 0x61, 0x6c, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x2e, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x0a,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20,
    0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x20,
    0x61, 0x73, 0x20, 0x66, 0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x61, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x73, 0x2e, 0x0a, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x6c, 0x79,
    0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x6f, 0x6f, 0x62,
    0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x74, 0x72, 0x75, 0x65, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x73,
    0x20, 0x31, 0x30, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x27, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x1b, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x27, 0x32, 0x33, 0x0a, 0x8a, 0x03, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x2f, 0x02, 0x2f, 0x1a, 0xfc, 0x02, 0x20, 0x41, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e,
    0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x72,
    0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x6d, 0x65, 0x74, 0x72, 0x69,
    0x63, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x6f, 0x75, 0x73, 0x6c, 0x79, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x61, 0x74, 0x20, 0x6c, 0x65, 0x61, 0x73, 0x74, 0x0a, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x77, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x2e,
    0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x61, 0x76, 0x6f, 0x69, 0x64, 0x73, 0x0a, 0x20, 0x63, 0x68,
    0x75, 0x72, 0x6e, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x74,
    0x20, 0x6f, 0x66, 0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x61, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x2e, 0x20,
    0x54, 0x61, 0x6b, 0x65, 0x73, 0x20, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x0a, 0x20, 0x62, 0x6f,
    0x74, 0x68, 0x20, 0x69, 0x6d, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x6c, 0x79, 0x20, 0x61,
    0x66, 0x74, 0x65, 0x72, 0x20, 0x77, 0x65, 0x20, 0x65, 0x73, 0x74, 0x61, 0x62, 0x6c, 0x69, 0x73,
    0x68, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74,
    0x6f, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x61, 0x6e,
    0x64, 0x0a, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x5f,
    0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x65, 0x72, 0x69, 0x6f,
    0x64, 0x20, 0x68, 0x61, 0x73, 0x20, 0x63, 0x61, 0x75, 0x73, 0x65, 0x64, 0x20, 0x75, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x73, 0x74, 0x6f, 0x70, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6d, 0x6f, 0x73, 0x74, 0x0a, 0x20, 0x72, 0x65, 0x63, 0x65, 0x6e, 0x74, 0x20, 0x6c,
    0x6f, 0x61, 0x64, 0x20, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x2e, 0x20, 0x44, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x73, 0x20, 0x31, 0x30, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e,
    0x64, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2f,
    0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x1b, 0x2a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x2d, 0x2e, 0x0a, 0xbf,
    0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x36, 0x02, 0x38, 0x1a, 0xb1, 0x02, 0x20,
    0x49, 0x66, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f,
    0x69, 0x6e, 0x74, 0x20, 0x68, 0x61, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x72, 0x65, 0x70, 0x6f,
    0x72, 0x74, 0x65, 0x64, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63,
    0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x2c, 0x0a,
    0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x77, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x70, 0x20, 0x75, 0x73,
    0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64,
    0x20, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x65, 0x6e,
    0x73, 0x75, 0x72, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x65, 0x20, 0x64, 0x6f,
    0x0a, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x76, 0x65, 0x72, 0x79, 0x20, 0x73, 0x74, 0x61, 0x6c, 0x65,
    0x20, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x73, 0x2e, 0x20, 0x4f, 0x6e, 0x63, 0x65, 0x20, 0x77,
    0x65, 0x20, 0x73, 0x74, 0x6f, 0x70, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x73,
    0x74, 0x61, 0x6c, 0x65, 0x0a, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20,
    0x77, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x73,
    0x65, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x72, 0x65, 0x73, 0x68, 0x20, 0x72, 0x65, 0x70, 0x6f,
    0x72, 0x74, 0x73, 0x20, 0x61, 0x67, 0x61, 0x69, 0x6e, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20,
    0x62, 0x6c, 0x61, 0x63, 0x6b, 0x6f, 0x75, 0x74, 0x5f, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x20,
    0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x73, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x33, 0x20, 0x6d, 0x69, 0x6e, 0x75, 0x74, 0x65, 0x73, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x36, 0x02, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x36, 0x1b, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x36, 0x36, 0x37, 0x0a, 0x50, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x39, 0x02, 0x34, 0x1a, 0x43, 0x20, 0x48, 0x6f, 0x77, 0x20, 0x6f, 0x66,
    0x74, 0x65, 0x6e, 0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x77, 0x65, 0x69,
    0x67, 0x68, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x72, 0x65, 0x63, 0x61, 0x6c, 0x63, 0x75,
    0x6c, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x31, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x39, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x39, 0x1b, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x39, 0x32, 0x33, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)