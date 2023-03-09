// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamMetricsResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamMetricsMessage {
    /// Identifier data effectively is a structured metadata. As a performance optimization this will
    /// only be sent in the first message on the stream.
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<stream_metrics_message::Identifier>,
    /// A list of metric entries
    #[prost(message, repeated, tag="2")]
    pub envoy_metrics: ::prost::alloc::vec::Vec<super::super::super::super::io::prometheus::client::MetricFamily>,
}
/// Nested message and enum types in `StreamMetricsMessage`.
pub mod stream_metrics_message {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identifier {
        /// The node sending metrics over the stream.
        #[prost(message, optional, tag="1")]
        pub node: ::core::option::Option<super::super::super::super::config::core::v3::Node>,
    }
}
/// Encoded file descriptor set for the `envoy.service.metrics.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd5, 0x11, 0x0a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2f, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x2f, 0x76, 0x33, 0x2f, 0x6d, 0x65,
    0x74, 0x72, 0x69, 0x63, 0x73, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x18, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2e, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f, 0x72, 0x65,
    0x2f, 0x76, 0x33, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x22,
    0x69, 0x6f, 0x2f, 0x70, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2f, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x2f, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x21, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4e, 0x0a,
    0x15, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x3a, 0x35, 0x9a, 0xc5, 0x88, 0x1e, 0x30, 0x0a, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x6d, 0x65, 0x74,
    0x72, 0x69, 0x63, 0x73, 0x2e, 0x76, 0x32, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x65,
    0x74, 0x72, 0x69, 0x63, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0xfa, 0x02,
    0x0a, 0x14, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x59, 0x0a, 0x0a, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69,
    0x66, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x39, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x6d, 0x65, 0x74, 0x72, 0x69,
    0x63, 0x73, 0x2e, 0x76, 0x33, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x65, 0x74, 0x72,
    0x69, 0x63, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x49, 0x64, 0x65, 0x6e, 0x74,
    0x69, 0x66, 0x69, 0x65, 0x72, 0x52, 0x0a, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x12, 0x47, 0x0a, 0x0d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x6d, 0x65, 0x74, 0x72, 0x69,
    0x63, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x69, 0x6f, 0x2e, 0x70, 0x72,
    0x6f, 0x6d, 0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e,
    0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x46, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x52, 0x0c, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x1a, 0x87, 0x01, 0x0a, 0x0a, 0x49,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x38, 0x0a, 0x04, 0x6e, 0x6f, 0x64,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x4e,
    0x6f, 0x64, 0x65, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x04, 0x6e,
    0x6f, 0x64, 0x65, 0x3a, 0x3f, 0x9a, 0xc5, 0x88, 0x1e, 0x3a, 0x0a, 0x38, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63,
    0x73, 0x2e, 0x76, 0x32, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x65, 0x74, 0x72, 0x69,
    0x63, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69,
    0x66, 0x69, 0x65, 0x72, 0x3a, 0x34, 0x9a, 0xc5, 0x88, 0x1e, 0x2f, 0x0a, 0x2d, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x6d, 0x65, 0x74, 0x72, 0x69,
    0x63, 0x73, 0x2e, 0x76, 0x32, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x65, 0x74, 0x72,
    0x69, 0x63, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x32, 0x86, 0x01, 0x0a, 0x0e, 0x4d,
    0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x74, 0x0a,
    0x0d, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x12, 0x2e,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x6d,
    0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x2e, 0x76, 0x33, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x1a, 0x2f,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x6d,
    0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x2e, 0x76, 0x33, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
    0x00, 0x28, 0x01, 0x42, 0x92, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x26,
    0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x6d, 0x65, 0x74, 0x72,
    0x69, 0x63, 0x73, 0x2e, 0x76, 0x33, 0x42, 0x13, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x53,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x49, 0x67,
    0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70,
    0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d,
    0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x2f, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x2f, 0x76, 0x33, 0x3b, 0x6d,
    0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x76, 0x33, 0x4a, 0xf5, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x34, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x21, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04,
    0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x2c, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x08, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x09, 0x00, 0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0a, 0x00, 0x21, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x3f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x0c, 0x00, 0x3f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x34, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x34, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e,
    0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0e, 0x00, 0x22, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x60, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0f,
    0x00, 0x60, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x10, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06,
    0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x10, 0x00, 0x46, 0x0a, 0xd2, 0x01, 0x0a, 0x02,
    0x06, 0x00, 0x12, 0x04, 0x16, 0x00, 0x1b, 0x01, 0x1a, 0x9f, 0x01, 0x20, 0x53, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e,
    0x67, 0x20, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x75, 0x6d, 0x65,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x20, 0x64, 0x61,
    0x74, 0x61, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x75, 0x73, 0x65, 0x73, 0x20, 0x50, 0x72, 0x6f, 0x6d,
    0x65, 0x74, 0x68, 0x65, 0x75, 0x73, 0x20, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x0a, 0x20, 0x64,
    0x61, 0x74, 0x61, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x73,
    0x74, 0x61, 0x6e, 0x64, 0x61, 0x72, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65,
    0x73, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x20, 0x69, 0x6e, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x32, 0x24, 0x20, 0x5b, 0x23, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x4d,
    0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5d, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x16, 0x08, 0x16, 0x0a, 0xb0, 0x01, 0x0a,
    0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x19, 0x02, 0x1a, 0x03, 0x1a, 0xa1, 0x01, 0x20, 0x45,
    0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
    0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x65, 0x76, 0x65, 0x72,
    0x2e, 0x20, 0x49, 0x74, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x65, 0x78,
    0x70, 0x65, 0x63, 0x74, 0x20, 0x61, 0x6e, 0x79, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x73,
    0x20, 0x6e, 0x6f, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62,
    0x65, 0x20, 0x64, 0x6f, 0x6e, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61,
    0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x06, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x19, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x19, 0x1b, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x19, 0x3a, 0x4f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1d,
    0x00, 0x20, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x1d, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x1e, 0x02, 0x1f, 0x37, 0x0a, 0x10, 0x0a, 0x08,
    0x04, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x1e, 0x02, 0x1f, 0x37, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x22, 0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x22, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x01, 0x07, 0x12, 0x04, 0x23,
    0x02, 0x24, 0x36, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x01, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12,
    0x04, 0x23, 0x02, 0x24, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12, 0x04, 0x26,
    0x02, 0x2c, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12, 0x03, 0x26, 0x0a,
    0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x07, 0x12, 0x04, 0x27, 0x04, 0x28, 0x43,
    0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04,
    0x27, 0x04, 0x28, 0x43, 0x0a, 0x3a, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x2b, 0x04, 0x4f, 0x1a, 0x2b, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x73,
    0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x20, 0x6f,
    0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2b, 0x04, 0x17,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x18, 0x1c,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x1f, 0x20,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x2b, 0x21, 0x4e,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x11, 0x12, 0x03,
    0x2b, 0x22, 0x4d, 0x0a, 0x9f, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x30, 0x02,
    0x1c, 0x1a, 0x91, 0x01, 0x20, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x20,
    0x64, 0x61, 0x74, 0x61, 0x20, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x6c, 0x79,
    0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x64,
    0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x20, 0x41, 0x73, 0x20, 0x61, 0x20,
    0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69,
    0x6d, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x77, 0x69,
    0x6c, 0x6c, 0x0a, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x30, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x30, 0x0d,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x30, 0x1a, 0x1b, 0x0a,
    0x27, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x33, 0x02, 0x3f, 0x1a, 0x1a, 0x20, 0x41,
    0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x20,
    0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x33, 0x0b, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x33,
    0x2d, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x33, 0x3d, 0x3e,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.service.metrics.v3.serde.rs");
include!("envoy.service.metrics.v3.tonic.rs");
// @@protoc_insertion_point(module)