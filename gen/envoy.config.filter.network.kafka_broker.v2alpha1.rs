// @generated
// [#protodoc-title: Kafka Broker]
// Kafka Broker :ref:`configuration overview <config_network_filters_kafka_broker>`.
// [#extension: envoy.filters.network.kafka_broker]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KafkaBroker {
    /// The prefix to use when emitting :ref:`statistics <config_network_filters_kafka_broker_stats>`.
    #[prost(string, tag="1")]
    pub stat_prefix: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `envoy.config.filter.network.kafka_broker.v2alpha1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xc6, 0x08, 0x0a, 0x44, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x2f, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x62, 0x72, 0x6f, 0x6b, 0x65, 0x72, 0x2f, 0x76, 0x32,
    0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x2f, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x62, 0x72, 0x6f,
    0x6b, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x31, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x62, 0x72, 0x6f,
    0x6b, 0x65, 0x72, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x1a, 0x1e, 0x75, 0x64,
    0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d,
    0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64,
    0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x37, 0x0a, 0x0b, 0x4b, 0x61, 0x66, 0x6b, 0x61, 0x42, 0x72, 0x6f,
    0x6b, 0x65, 0x72, 0x12, 0x28, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x74, 0x5f, 0x70, 0x72, 0x65, 0x66,
    0x69, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x20,
    0x01, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x50, 0x72, 0x65, 0x66, 0x69, 0x78, 0x42, 0xef, 0x01,
    0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x32, 0x12, 0x30, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x62,
    0x72, 0x6f, 0x6b, 0x65, 0x72, 0x2e, 0x76, 0x33, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x01,
    0x0a, 0x3f, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6b, 0x61, 0x66, 0x6b,
    0x61, 0x5f, 0x62, 0x72, 0x6f, 0x6b, 0x65, 0x72, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61,
    0x31, 0x42, 0x10, 0x4b, 0x61, 0x66, 0x6b, 0x61, 0x42, 0x72, 0x6f, 0x6b, 0x65, 0x72, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x58, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
    0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f,
    0x62, 0x72, 0x6f, 0x6b, 0x65, 0x72, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x4a,
    0xbf, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x17, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x3a, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x05, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x00, 0x21, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x58, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x08, 0x00, 0x58, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x31, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x09, 0x00, 0x31, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0a, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0a, 0x00, 0x22, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x6f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03,
    0x0b, 0x00, 0x6f, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04, 0x0c, 0x00, 0x0d, 0x37, 0x0a, 0x0e,
    0x0a, 0x06, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12, 0x04, 0x0c, 0x00, 0x0d, 0x37, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99,
    0x6a, 0x02, 0x12, 0x03, 0x0e, 0x00, 0x46, 0x0a, 0xb3, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x14, 0x00, 0x17, 0x01, 0x32, 0xa6, 0x01, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64,
    0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x4b, 0x61, 0x66, 0x6b, 0x61, 0x20,
    0x42, 0x72, 0x6f, 0x6b, 0x65, 0x72, 0x5d, 0x0a, 0x20, 0x4b, 0x61, 0x66, 0x6b, 0x61, 0x20, 0x42,
    0x72, 0x6f, 0x6b, 0x65, 0x72, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69,
    0x65, 0x77, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x6e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x6b, 0x61, 0x66, 0x6b, 0x61,
    0x5f, 0x62, 0x72, 0x6f, 0x6b, 0x65, 0x72, 0x3e, 0x60, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6b,
    0x61, 0x66, 0x6b, 0x61, 0x5f, 0x62, 0x72, 0x6f, 0x6b, 0x65, 0x72, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x13, 0x0a, 0x6d, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x16, 0x02, 0x44, 0x1a, 0x60, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x72, 0x65,
    0x66, 0x69, 0x78, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20,
    0x65, 0x6d, 0x69, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x73,
    0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x5f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x73, 0x5f, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x5f, 0x62, 0x72, 0x6f, 0x6b, 0x65, 0x72, 0x5f, 0x73,
    0x74, 0x61, 0x74, 0x73, 0x3e, 0x60, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x16, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x16, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16,
    0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x16, 0x19, 0x43,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x16, 0x1a,
    0x42, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.config.filter.network.kafka_broker.v2alpha1.serde.rs");
// @@protoc_insertion_point(module)