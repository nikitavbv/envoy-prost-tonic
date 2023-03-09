// @generated
// [#protodoc-title: Kafka Mesh]
// Kafka Mesh :ref:`configuration overview <config_network_filters_kafka_mesh>`.
// [#extension: envoy.filters.network.kafka_mesh]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KafkaMesh {
    /// Envoy's host that's advertised to clients.
    /// Has the same meaning as corresponding Kafka broker properties.
    /// Usually equal to filter chain's listener config, but needs to be reachable by clients
    /// (so 0.0.0.0 will not work).
    #[prost(string, tag="1")]
    pub advertised_host: ::prost::alloc::string::String,
    /// Envoy's port that's advertised to clients.
    #[prost(int32, tag="2")]
    pub advertised_port: i32,
    /// Upstream clusters this filter will connect to.
    #[prost(message, repeated, tag="3")]
    pub upstream_clusters: ::prost::alloc::vec::Vec<KafkaClusterDefinition>,
    /// Rules that will decide which cluster gets which request.
    #[prost(message, repeated, tag="4")]
    pub forwarding_rules: ::prost::alloc::vec::Vec<ForwardingRule>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KafkaClusterDefinition {
    /// Cluster name.
    #[prost(string, tag="1")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Kafka cluster address.
    #[prost(string, tag="2")]
    pub bootstrap_servers: ::prost::alloc::string::String,
    /// Default number of partitions present in this cluster.
    /// This is especially important for clients that do not specify partition in their payloads and depend on this value for hashing.
    #[prost(int32, tag="3")]
    pub partition_count: i32,
    /// Custom configuration passed to Kafka producer.
    #[prost(map="string, string", tag="4")]
    pub producer_config: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardingRule {
    /// Cluster name.
    #[prost(string, tag="1")]
    pub target_cluster: ::prost::alloc::string::String,
    #[prost(oneof="forwarding_rule::Trigger", tags="2")]
    pub trigger: ::core::option::Option<forwarding_rule::Trigger>,
}
/// Nested message and enum types in `ForwardingRule`.
pub mod forwarding_rule {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trigger {
        /// Intended place for future types of forwarding rules.
        #[prost(string, tag="2")]
        TopicPrefix(::prost::alloc::string::String),
    }
}
/// Encoded file descriptor set for the `envoy.extensions.filters.network.kafka_mesh.v3alpha` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf4, 0x18, 0x0a, 0x4c, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x6b, 0x61,
    0x66, 0x6b, 0x61, 0x5f, 0x6d, 0x65, 0x73, 0x68, 0x2f, 0x76, 0x33, 0x61, 0x6c, 0x70, 0x68, 0x61,
    0x2f, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x6d, 0x65, 0x73, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x33, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77,
    0x6f, 0x72, 0x6b, 0x2e, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x6d, 0x65, 0x73, 0x68, 0x2e, 0x76,
    0x33, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x1a, 0x1f, 0x78, 0x64, 0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x33, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e,
    0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0xd9, 0x02, 0x0a, 0x09, 0x4b, 0x61, 0x66, 0x6b, 0x61, 0x4d, 0x65, 0x73, 0x68, 0x12, 0x30, 0x0a,
    0x0f, 0x61, 0x64, 0x76, 0x65, 0x72, 0x74, 0x69, 0x73, 0x65, 0x64, 0x5f, 0x68, 0x6f, 0x73, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x10, 0x01, 0x52,
    0x0e, 0x61, 0x64, 0x76, 0x65, 0x72, 0x74, 0x69, 0x73, 0x65, 0x64, 0x48, 0x6f, 0x73, 0x74, 0x12,
    0x30, 0x0a, 0x0f, 0x61, 0x64, 0x76, 0x65, 0x72, 0x74, 0x69, 0x73, 0x65, 0x64, 0x5f, 0x70, 0x6f,
    0x72, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x1a, 0x02, 0x20,
    0x00, 0x52, 0x0e, 0x61, 0x64, 0x76, 0x65, 0x72, 0x74, 0x69, 0x73, 0x65, 0x64, 0x50, 0x6f, 0x72,
    0x74, 0x12, 0x78, 0x0a, 0x11, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f, 0x63, 0x6c,
    0x75, 0x73, 0x74, 0x65, 0x72, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x4b, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e,
    0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x6d, 0x65, 0x73, 0x68, 0x2e, 0x76, 0x33, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0x2e, 0x4b, 0x61, 0x66, 0x6b, 0x61, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x44,
    0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x10, 0x75, 0x70, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x73, 0x12, 0x6e, 0x0a, 0x10, 0x66,
    0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x18,
    0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x43, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x6d,
    0x65, 0x73, 0x68, 0x2e, 0x76, 0x33, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x46, 0x6f, 0x72, 0x77,
    0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x75, 0x6c, 0x65, 0x52, 0x0f, 0x66, 0x6f, 0x72, 0x77,
    0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x75, 0x6c, 0x65, 0x73, 0x22, 0xfa, 0x02, 0x0a, 0x16,
    0x4b, 0x61, 0x66, 0x6b, 0x61, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x44, 0x65, 0x66, 0x69,
    0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2a, 0x0a, 0x0c, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07, 0xfa, 0x42,
    0x04, 0x72, 0x02, 0x10, 0x01, 0x52, 0x0b, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x4e, 0x61,
    0x6d, 0x65, 0x12, 0x34, 0x0a, 0x11, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x5f,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07, 0xfa,
    0x42, 0x04, 0x72, 0x02, 0x10, 0x01, 0x52, 0x10, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61,
    0x70, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x12, 0x30, 0x0a, 0x0f, 0x70, 0x61, 0x72, 0x74,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x05, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x1a, 0x02, 0x20, 0x00, 0x52, 0x0e, 0x70, 0x61, 0x72, 0x74,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x88, 0x01, 0x0a, 0x0f, 0x70,
    0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x04,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x5f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x6d, 0x65,
    0x73, 0x68, 0x2e, 0x76, 0x33, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x4b, 0x61, 0x66, 0x6b, 0x61,
    0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x44, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x50, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x0e, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x72, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x1a, 0x41, 0x0a, 0x13, 0x50, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65,
    0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03,
    0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14,
    0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x22, 0x67, 0x0a, 0x0e, 0x46, 0x6f, 0x72, 0x77,
    0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x75, 0x6c, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x74, 0x61,
    0x72, 0x67, 0x65, 0x74, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0d, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x12, 0x23, 0x0a, 0x0c, 0x74, 0x6f, 0x70, 0x69, 0x63, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x69,
    0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0b, 0x74, 0x6f, 0x70, 0x69, 0x63,
    0x50, 0x72, 0x65, 0x66, 0x69, 0x78, 0x42, 0x09, 0x0a, 0x07, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65,
    0x72, 0x42, 0xc1, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0xd2, 0xc6, 0xa4, 0xe1,
    0x06, 0x02, 0x08, 0x01, 0x0a, 0x41, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72,
    0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x6d, 0x65, 0x73, 0x68, 0x2e,
    0x76, 0x33, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x42, 0x0e, 0x4b, 0x61, 0x66, 0x6b, 0x61, 0x4d, 0x65,
    0x73, 0x68, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x5a, 0x67, 0x69, 0x74, 0x68, 0x75,
    0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e,
    0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f,
    0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x2f, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x6d, 0x65, 0x73, 0x68, 0x2f, 0x76, 0x33,
    0x61, 0x6c, 0x70, 0x68, 0x61, 0x4a, 0x87, 0x0f, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x3c, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x02, 0x00, 0x3c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x07, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x5a, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09, 0x00, 0x5a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0a, 0x00, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x2f, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12,
    0x03, 0x0b, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x71, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0c, 0x00, 0x71, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0d, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0d,
    0x00, 0x46, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x40, 0x0a, 0x0d, 0x0a, 0x06,
    0x08, 0xea, 0xc8, 0x94, 0x6c, 0x01, 0x12, 0x03, 0x0e, 0x00, 0x40, 0x0a, 0xab, 0x01, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x14, 0x00, 0x23, 0x01, 0x32, 0x9e, 0x01, 0x20, 0x5b, 0x23, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x4b, 0x61,
    0x66, 0x6b, 0x61, 0x20, 0x4d, 0x65, 0x73, 0x68, 0x5d, 0x0a, 0x20, 0x4b, 0x61, 0x66, 0x6b, 0x61,
    0x20, 0x4d, 0x65, 0x73, 0x68, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69,
    0x65, 0x77, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x6e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x6b, 0x61, 0x66, 0x6b, 0x61,
    0x5f, 0x6d, 0x65, 0x73, 0x68, 0x3e, 0x60, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6b, 0x61, 0x66,
    0x6b, 0x61, 0x5f, 0x6d, 0x65, 0x73, 0x68, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x14, 0x08, 0x11, 0x0a, 0xee, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x19, 0x02, 0x46, 0x1a, 0xe0, 0x01, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x27, 0x73, 0x20, 0x68,
    0x6f, 0x73, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x27, 0x73, 0x20, 0x61, 0x64, 0x76, 0x65, 0x72,
    0x74, 0x69, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73,
    0x2e, 0x0a, 0x20, 0x48, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20,
    0x6d, 0x65, 0x61, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x73, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x4b, 0x61, 0x66, 0x6b, 0x61, 0x20, 0x62,
    0x72, 0x6f, 0x6b, 0x65, 0x72, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x69, 0x65, 0x73,
    0x2e, 0x0a, 0x20, 0x55, 0x73, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x65, 0x71, 0x75, 0x61, 0x6c,
    0x20, 0x74, 0x6f, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e,
    0x27, 0x73, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x2c, 0x20, 0x62, 0x75, 0x74, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x61, 0x63, 0x68, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x62, 0x79,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x0a, 0x20, 0x28, 0x73, 0x6f, 0x20, 0x30, 0x2e,
    0x30, 0x2e, 0x30, 0x2e, 0x30, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x77,
    0x6f, 0x72, 0x6b, 0x29, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x19, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19,
    0x09, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x1b, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x19, 0x1d, 0x45, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x19, 0x1e, 0x44, 0x0a,
    0x39, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x02, 0x3f, 0x1a, 0x2c, 0x20, 0x45,
    0x6e, 0x76, 0x6f, 0x79, 0x27, 0x73, 0x20, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x27, 0x73, 0x20, 0x61, 0x64, 0x76, 0x65, 0x72, 0x74, 0x69, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x1c, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x1c, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x1c, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x1c,
    0x1c, 0x3e, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x03, 0x12, 0x03,
    0x1c, 0x1d, 0x3d, 0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x02, 0x38,
    0x1a, 0x30, 0x20, 0x55, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x63, 0x6c, 0x75, 0x73,
    0x74, 0x65, 0x72, 0x73, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x20, 0x74, 0x6f,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1f, 0x0b, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x22, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1f, 0x36, 0x37, 0x0a, 0x47, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x22, 0x02, 0x2f, 0x1a, 0x3a, 0x20, 0x52, 0x75, 0x6c, 0x65, 0x73, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x64, 0x65, 0x63, 0x69, 0x64, 0x65,
    0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x20, 0x67,
    0x65, 0x74, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x22, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x22, 0x0b, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x22, 0x1a, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x22, 0x2d, 0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x25, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x25, 0x08, 0x1e, 0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x27, 0x02, 0x43,
    0x1a, 0x0f, 0x20, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x27, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x09, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x27, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x08, 0x12, 0x03, 0x27, 0x1a, 0x42, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02,
    0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x27, 0x1b, 0x41, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x48, 0x1a, 0x18, 0x20, 0x4b, 0x61, 0x66, 0x6b, 0x61, 0x20,
    0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2a, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x09, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x08, 0x12, 0x03, 0x2a, 0x1f, 0x47, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02,
    0x01, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x2a, 0x20, 0x46, 0x0a, 0xc5, 0x01, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x02, 0x12, 0x03, 0x2e, 0x02, 0x3f, 0x1a, 0xb7, 0x01, 0x20, 0x44, 0x65, 0x66, 0x61,
    0x75, 0x6c, 0x74, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x70, 0x61,
    0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x65, 0x73, 0x70, 0x65, 0x63,
    0x69, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x61, 0x6e, 0x74, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x64, 0x6f, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20,
    0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x69, 0x72, 0x20, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x68, 0x61, 0x73, 0x68, 0x69, 0x6e, 0x67,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2e, 0x02, 0x07,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x08, 0x12, 0x03, 0x2e, 0x1c, 0x3e, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01,
    0x02, 0x02, 0x08, 0xaf, 0x08, 0x03, 0x12, 0x03, 0x2e, 0x1d, 0x3d, 0x0a, 0x3d, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x03, 0x12, 0x03, 0x31, 0x02, 0x2a, 0x1a, 0x30, 0x20, 0x43, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x70, 0x61, 0x73, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x4b, 0x61, 0x66, 0x6b, 0x61, 0x20,
    0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x06, 0x12, 0x03, 0x31, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x31, 0x16, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x31, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x34, 0x00, 0x3c, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x34, 0x08, 0x16, 0x0a, 0x1c, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x36, 0x02, 0x1c, 0x1a, 0x0f, 0x20, 0x43, 0x6c, 0x75, 0x73,
    0x74, 0x65, 0x72, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x36, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x36, 0x09, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x36, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x08, 0x00, 0x12, 0x04, 0x38, 0x02,
    0x3b, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x08, 0x00, 0x01, 0x12, 0x03, 0x38, 0x08, 0x0f,
    0x0a, 0x43, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x04, 0x1c, 0x1a, 0x36, 0x20,
    0x49, 0x6e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x20, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20,
    0x6f, 0x66, 0x20, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x75,
    0x6c, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x3a, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x0b,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3a, 0x1a, 0x1b, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.network.kafka_mesh.v3alpha.serde.rs");
// @@protoc_insertion_point(module)