// @generated
// [#protodoc-title: Thrift]
// Thrift health checker :ref:`configuration overview <config_health_checkers_thrift>`.
// [#extension: envoy.health_checkers.thrift]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Thrift {
    /// Specifies the method name that will be set on each health check request dispatched to an upstream host.
    /// Note that method name is case sensitive.
    #[prost(string, tag="1")]
    pub method_name: ::prost::alloc::string::String,
    /// Configures the transport type to be used with the health checks. Note that
    /// :ref:`AUTO_TRANSPORT<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.TransportType.AUTO_TRANSPORT>`
    /// is not supported, and we don't honor :ref:`ThriftProtocolOptions<envoy_v3_api_msg_extensions.filters.network.thrift_proxy.v3.ThriftProtocolOptions>`
    /// since it's possible to set to :ref:`AUTO_TRANSPORT<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.TransportType.AUTO_TRANSPORT>`.
    /// [#extension-category: envoy.filters.network]
    #[prost(enumeration="super::super::super::filters::network::thrift_proxy::v3::TransportType", tag="2")]
    pub transport: i32,
    /// Configures the protocol type to be used with the health checks. Note that
    /// :ref:`AUTO_PROTOCOL<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.AUTO_PROTOCOL>`
    /// and :ref:`TWITTER<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.TWITTER>`
    /// are not supported, and we don't honor :ref:`ThriftProtocolOptions<envoy_v3_api_msg_extensions.filters.network.thrift_proxy.v3.ThriftProtocolOptions>`
    /// since it's possible to set to :ref:`AUTO_PROTOCOL<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.AUTO_PROTOCOL>`
    /// or :ref:`TWITTER<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.TWITTER>`.
    #[prost(enumeration="super::super::super::filters::network::thrift_proxy::v3::ProtocolType", tag="3")]
    pub protocol: i32,
}
/// Encoded file descriptor set for the `envoy.extensions.health_checkers.thrift.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x8a, 0x15, 0x0a, 0x37, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x5f, 0x63, 0x68, 0x65,
    0x63, 0x6b, 0x65, 0x72, 0x73, 0x2f, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x2f, 0x76, 0x33, 0x2f,
    0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2a, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x68,
    0x65, 0x61, 0x6c, 0x74, 0x68, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x65, 0x72, 0x73, 0x2e, 0x74,
    0x68, 0x72, 0x69, 0x66, 0x74, 0x2e, 0x76, 0x33, 0x1a, 0x43, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x74, 0x68, 0x72, 0x69, 0x66,
    0x74, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x76, 0x33, 0x2f, 0x74, 0x68, 0x72, 0x69, 0x66,
    0x74, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75,
    0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x81, 0x02, 0x0a, 0x06, 0x54, 0x68, 0x72, 0x69, 0x66, 0x74,
    0x12, 0x28, 0x0a, 0x0b, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x10, 0x01, 0x52, 0x0a,
    0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x67, 0x0a, 0x09, 0x74, 0x72,
    0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3f, 0x2e,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x2e, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x76, 0x33,
    0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x54, 0x79, 0x70, 0x65, 0x42, 0x08,
    0xfa, 0x42, 0x05, 0x82, 0x01, 0x02, 0x10, 0x01, 0x52, 0x09, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70,
    0x6f, 0x72, 0x74, 0x12, 0x64, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3e, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x5f,
    0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x76, 0x33, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f,
    0x6c, 0x54, 0x79, 0x70, 0x65, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x82, 0x01, 0x02, 0x10, 0x01, 0x52,
    0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x42, 0xad, 0x01, 0xba, 0x80, 0xc8, 0xd1,
    0x06, 0x02, 0x10, 0x02, 0x0a, 0x38, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72,
    0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x5f, 0x63, 0x68, 0x65, 0x63,
    0x6b, 0x65, 0x72, 0x73, 0x2e, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x2e, 0x76, 0x33, 0x42, 0x0b,
    0x54, 0x68, 0x72, 0x69, 0x66, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x5a, 0x67,
    0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70,
    0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d,
    0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x5f, 0x63, 0x68,
    0x65, 0x63, 0x6b, 0x65, 0x72, 0x73, 0x2f, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x2f, 0x76, 0x33,
    0x3b, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x76, 0x33, 0x4a, 0xe9, 0x0f, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x28, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x33, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x04, 0x00, 0x4d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x27, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x09, 0x00, 0x51, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09, 0x00, 0x51, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03,
    0x0a, 0x00, 0x2c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c,
    0x00, 0x71, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0c, 0x00, 0x71, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a,
    0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0xaa, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x13,
    0x00, 0x28, 0x01, 0x32, 0x9d, 0x01, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f,
    0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x54, 0x68, 0x72, 0x69, 0x66, 0x74, 0x5d,
    0x0a, 0x20, 0x54, 0x68, 0x72, 0x69, 0x66, 0x74, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20,
    0x63, 0x68, 0x65, 0x63, 0x6b, 0x65, 0x72, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72,
    0x76, 0x69, 0x65, 0x77, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x65, 0x61,
    0x6c, 0x74, 0x68, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x65, 0x72, 0x73, 0x5f, 0x74, 0x68, 0x72,
    0x69, 0x66, 0x74, 0x3e, 0x60, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x68, 0x65, 0x61, 0x6c, 0x74,
    0x68, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x65, 0x72, 0x73, 0x2e, 0x74, 0x68, 0x72, 0x69, 0x66,
    0x74, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x13, 0x08, 0x0e, 0x0a,
    0xa1, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x16, 0x02, 0x42, 0x1a, 0x93, 0x01,
    0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d,
    0x65, 0x74, 0x68, 0x6f, 0x64, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x6e, 0x20, 0x65,
    0x61, 0x63, 0x68, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x64, 0x69, 0x73, 0x70, 0x61, 0x74, 0x63,
    0x68, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x2e, 0x0a, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20,
    0x69, 0x73, 0x20, 0x63, 0x61, 0x73, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x73, 0x69, 0x74, 0x69, 0x76,
    0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x09, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x17, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x16, 0x19, 0x41, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x16, 0x1a, 0x40, 0x0a, 0xae, 0x04, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0x1d, 0x02, 0x1e, 0x35, 0x1a, 0x9f, 0x04, 0x20, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72,
    0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x2e, 0x20,
    0x4e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a,
    0x60, 0x41, 0x55, 0x54, 0x4f, 0x5f, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x50, 0x4f, 0x52, 0x54, 0x3c,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x65, 0x6e, 0x75,
    0x6d, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f,
    0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x2e, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e,
    0x76, 0x33, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x54, 0x79, 0x70, 0x65,
    0x2e, 0x41, 0x55, 0x54, 0x4f, 0x5f, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x50, 0x4f, 0x52, 0x54, 0x3e,
    0x60, 0x0a, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72,
    0x74, 0x65, 0x64, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x77, 0x65, 0x20, 0x64, 0x6f, 0x6e, 0x27,
    0x74, 0x20, 0x68, 0x6f, 0x6e, 0x6f, 0x72, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x54, 0x68,
    0x72, 0x69, 0x66, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x4f, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69,
    0x5f, 0x6d, 0x73, 0x67, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e,
    0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x76, 0x33, 0x2e,
    0x54, 0x68, 0x72, 0x69, 0x66, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x4f, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x3e, 0x60, 0x0a, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x69,
    0x74, 0x27, 0x73, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x41, 0x55, 0x54,
    0x4f, 0x5f, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x50, 0x4f, 0x52, 0x54, 0x3c, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x74,
    0x68, 0x72, 0x69, 0x66, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x76, 0x33, 0x2e, 0x54,
    0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x54, 0x79, 0x70, 0x65, 0x2e, 0x41, 0x55, 0x54,
    0x4f, 0x5f, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x50, 0x4f, 0x52, 0x54, 0x3e, 0x60, 0x2e, 0x0a, 0x20,
    0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2d, 0x63, 0x61, 0x74, 0x65,
    0x67, 0x6f, 0x72, 0x79, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5d, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1d, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x30, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x1d, 0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08,
    0x12, 0x03, 0x1e, 0x06, 0x34, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08,
    0x10, 0x12, 0x03, 0x1e, 0x07, 0x33, 0x0a, 0xd3, 0x05, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x04, 0x26, 0x02, 0x27, 0x35, 0x1a, 0xc4, 0x05, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75,
    0x72, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
    0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x2e, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x41, 0x55, 0x54, 0x4f, 0x5f, 0x50,
    0x52, 0x4f, 0x54, 0x4f, 0x43, 0x4f, 0x4c, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33,
    0x5f, 0x61, 0x70, 0x69, 0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x74, 0x68, 0x72, 0x69, 0x66,
    0x74, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x76, 0x33, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x2e, 0x41, 0x55, 0x54, 0x4f, 0x5f, 0x50, 0x52, 0x4f,
    0x54, 0x4f, 0x43, 0x4f, 0x4c, 0x3e, 0x60, 0x0a, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x3a, 0x72, 0x65,
    0x66, 0x3a, 0x60, 0x54, 0x57, 0x49, 0x54, 0x54, 0x45, 0x52, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x74, 0x68,
    0x72, 0x69, 0x66, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x76, 0x33, 0x2e, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x2e, 0x54, 0x57, 0x49, 0x54, 0x54,
    0x45, 0x52, 0x3e, 0x60, 0x0a, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x75,
    0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x77, 0x65, 0x20,
    0x64, 0x6f, 0x6e, 0x27, 0x74, 0x20, 0x68, 0x6f, 0x6e, 0x6f, 0x72, 0x20, 0x3a, 0x72, 0x65, 0x66,
    0x3a, 0x60, 0x54, 0x68, 0x72, 0x69, 0x66, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33,
    0x5f, 0x61, 0x70, 0x69, 0x5f, 0x6d, 0x73, 0x67, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77,
    0x6f, 0x72, 0x6b, 0x2e, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2e, 0x76, 0x33, 0x2e, 0x54, 0x68, 0x72, 0x69, 0x66, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x3e, 0x60, 0x0a, 0x20, 0x73, 0x69, 0x6e,
    0x63, 0x65, 0x20, 0x69, 0x74, 0x27, 0x73, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65,
    0x20, 0x74, 0x6f, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a,
    0x60, 0x41, 0x55, 0x54, 0x4f, 0x5f, 0x50, 0x52, 0x4f, 0x54, 0x4f, 0x43, 0x4f, 0x4c, 0x3c, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x65, 0x6e, 0x75, 0x6d,
    0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72,
    0x6b, 0x2e, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x76,
    0x33, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x2e, 0x41,
    0x55, 0x54, 0x4f, 0x5f, 0x50, 0x52, 0x4f, 0x54, 0x4f, 0x43, 0x4f, 0x4c, 0x3e, 0x60, 0x0a, 0x20,
    0x6f, 0x72, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x54, 0x57, 0x49, 0x54, 0x54, 0x45, 0x52,
    0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x65, 0x6e,
    0x75, 0x6d, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77,
    0x6f, 0x72, 0x6b, 0x2e, 0x74, 0x68, 0x72, 0x69, 0x66, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2e, 0x76, 0x33, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x54, 0x79, 0x70, 0x65,
    0x2e, 0x54, 0x57, 0x49, 0x54, 0x54, 0x45, 0x52, 0x3e, 0x60, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x26, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x2f, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x26, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12,
    0x03, 0x27, 0x06, 0x34, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x02, 0x08, 0xaf, 0x08, 0x10,
    0x12, 0x03, 0x27, 0x07, 0x33, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.health_checkers.thrift.v3.serde.rs");
// @@protoc_insertion_point(module)