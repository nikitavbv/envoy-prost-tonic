// @generated
// [#protodoc-title: c-ares DNS resolver]
// [#extension: envoy.network.dns_resolver.cares]

/// Configuration for c-ares DNS resolver.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaresDnsResolverConfig {
    /// A list of dns resolver addresses.
    /// :ref:`use_resolvers_as_fallback<envoy_v3_api_field_extensions.network.dns_resolver.cares.v3.CaresDnsResolverConfig.use_resolvers_as_fallback>`
    /// below dictates if the DNS client should override system defaults or only use the provided
    /// resolvers if the system defaults are not available, i.e., as a fallback.
    #[prost(message, repeated, tag="1")]
    pub resolvers: ::prost::alloc::vec::Vec<super::super::super::super::super::config::core::v3::Address>,
    /// If true use the resolvers listed in the
    /// :ref:`resolvers<envoy_v3_api_field_extensions.network.dns_resolver.cares.v3.CaresDnsResolverConfig.resolvers>`
    /// field only if c-ares is unable to obtain a
    /// nameserver from the system (e.g., /etc/resolv.conf).
    /// Otherwise, the resolvers listed in the resolvers list will override the default system
    /// resolvers. Defaults to false.
    #[prost(bool, tag="3")]
    pub use_resolvers_as_fallback: bool,
    /// The resolver will query available network interfaces and determine if there are no available
    /// interfaces for a given IP family. It will then filter these addresses from the results it
    /// presents. e.g., if there are no available IPv4 network interfaces, the resolver will not
    /// provide IPv4 addresses.
    #[prost(bool, tag="4")]
    pub filter_unroutable_families: bool,
    /// Configuration of DNS resolver option flags which control the behavior of the DNS resolver.
    #[prost(message, optional, tag="2")]
    pub dns_resolver_options: ::core::option::Option<super::super::super::super::super::config::core::v3::DnsResolverOptions>,
}
/// Encoded file descriptor set for the `envoy.extensions.network.dns_resolver.cares.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb8, 0x13, 0x0a, 0x47, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x64, 0x6e,
    0x73, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2f, 0x63, 0x61, 0x72, 0x65, 0x73,
    0x2f, 0x76, 0x33, 0x2f, 0x63, 0x61, 0x72, 0x65, 0x73, 0x5f, 0x64, 0x6e, 0x73, 0x5f, 0x72, 0x65,
    0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x64, 0x6e, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c,
    0x76, 0x65, 0x72, 0x2e, 0x63, 0x61, 0x72, 0x65, 0x73, 0x2e, 0x76, 0x33, 0x1a, 0x22, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f,
    0x76, 0x33, 0x2f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x23, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63,
    0x6f, 0x72, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xb4, 0x02,
    0x0a, 0x16, 0x43, 0x61, 0x72, 0x65, 0x73, 0x44, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76,
    0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x45, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x6f,
    0x6c, 0x76, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e,
    0x76, 0x33, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x92,
    0x01, 0x02, 0x08, 0x01, 0x52, 0x09, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x73, 0x12,
    0x39, 0x0a, 0x19, 0x75, 0x73, 0x65, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x73,
    0x5f, 0x61, 0x73, 0x5f, 0x66, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x16, 0x75, 0x73, 0x65, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x73,
    0x41, 0x73, 0x46, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x12, 0x3c, 0x0a, 0x1a, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x5f, 0x75, 0x6e, 0x72, 0x6f, 0x75, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f,
    0x66, 0x61, 0x6d, 0x69, 0x6c, 0x69, 0x65, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x18,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x55, 0x6e, 0x72, 0x6f, 0x75, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x46, 0x61, 0x6d, 0x69, 0x6c, 0x69, 0x65, 0x73, 0x12, 0x5a, 0x0a, 0x14, 0x64, 0x6e, 0x73, 0x5f,
    0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x44, 0x6e,
    0x73, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x52, 0x12, 0x64, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x4f, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x42, 0xbe, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a,
    0x3c, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x64, 0x6e, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x6f,
    0x6c, 0x76, 0x65, 0x72, 0x2e, 0x63, 0x61, 0x72, 0x65, 0x73, 0x2e, 0x76, 0x33, 0x42, 0x15, 0x43,
    0x61, 0x72, 0x65, 0x73, 0x44, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x5d, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63,
    0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f,
    0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x64, 0x6e, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x6f,
    0x6c, 0x76, 0x65, 0x72, 0x2f, 0x63, 0x61, 0x72, 0x65, 0x73, 0x2f, 0x76, 0x33, 0x3b, 0x63, 0x61,
    0x72, 0x65, 0x73, 0x76, 0x33, 0x4a, 0xbb, 0x0d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2b, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x02, 0x00, 0x37, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x2c, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x2d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x07, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08, 0x00, 0x21,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x55, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x0a, 0x00, 0x55, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x36, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x36, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0c, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0c, 0x00, 0x22, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x74, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12,
    0x03, 0x0d, 0x00, 0x74, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x46, 0x0a, 0x0d,
    0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0e, 0x00, 0x46, 0x0a, 0x8e, 0x01,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x14, 0x00, 0x2b, 0x01, 0x1a, 0x28, 0x20, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x63,
    0x2d, 0x61, 0x72, 0x65, 0x73, 0x20, 0x44, 0x4e, 0x53, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76,
    0x65, 0x72, 0x2e, 0x0a, 0x32, 0x58, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f,
    0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x63, 0x2d, 0x61, 0x72, 0x65, 0x73, 0x20,
    0x44, 0x4e, 0x53, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x5d, 0x0a, 0x20, 0x5b,
    0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x64, 0x6e, 0x73, 0x5f, 0x72, 0x65,
    0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2e, 0x63, 0x61, 0x72, 0x65, 0x73, 0x5d, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x1e, 0x0a, 0xe6, 0x02, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x5d, 0x1a, 0xd8, 0x02, 0x20, 0x41, 0x20, 0x6c, 0x69,
    0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x64, 0x6e, 0x73, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76,
    0x65, 0x72, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x2e, 0x0a, 0x20, 0x3a,
    0x72, 0x65, 0x66, 0x3a, 0x60, 0x75, 0x73, 0x65, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65,
    0x72, 0x73, 0x5f, 0x61, 0x73, 0x5f, 0x66, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x3c, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x64, 0x6e, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65,
    0x72, 0x2e, 0x63, 0x61, 0x72, 0x65, 0x73, 0x2e, 0x76, 0x33, 0x2e, 0x43, 0x61, 0x72, 0x65, 0x73,
    0x44, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x75, 0x73, 0x65, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x73, 0x5f,
    0x61, 0x73, 0x5f, 0x66, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x3e, 0x60, 0x0a, 0x20, 0x62,
    0x65, 0x6c, 0x6f, 0x77, 0x20, 0x64, 0x69, 0x63, 0x74, 0x61, 0x74, 0x65, 0x73, 0x20, 0x69, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x44, 0x4e, 0x53, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20,
    0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x20,
    0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20,
    0x6f, 0x72, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76,
    0x65, 0x72, 0x73, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65,
    0x6d, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e,
    0x6f, 0x74, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x2c, 0x20, 0x69, 0x2e,
    0x65, 0x2e, 0x2c, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x66, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63,
    0x6b, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x19, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x19, 0x0b, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x22, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x19, 0x30, 0x5c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02,
    0x00, 0x08, 0xaf, 0x08, 0x12, 0x12, 0x03, 0x19, 0x31, 0x5b, 0x0a, 0x80, 0x03, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x21, 0x02, 0x25, 0x1a, 0xf2, 0x02, 0x20, 0x49, 0x66, 0x20, 0x74,
    0x72, 0x75, 0x65, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f,
    0x6c, 0x76, 0x65, 0x72, 0x73, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x72, 0x65, 0x73, 0x6f, 0x6c,
    0x76, 0x65, 0x72, 0x73, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70,
    0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f,
    0x6e, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x64, 0x6e, 0x73, 0x5f, 0x72,
    0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2e, 0x63, 0x61, 0x72, 0x65, 0x73, 0x2e, 0x76, 0x33,
    0x2e, 0x43, 0x61, 0x72, 0x65, 0x73, 0x44, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65,
    0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72,
    0x73, 0x3e, 0x60, 0x0a, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20,
    0x69, 0x66, 0x20, 0x63, 0x2d, 0x61, 0x72, 0x65, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x6e, 0x61,
    0x62, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x6f, 0x62, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x0a,
    0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x20, 0x28, 0x65, 0x2e, 0x67,
    0x2e, 0x2c, 0x20, 0x2f, 0x65, 0x74, 0x63, 0x2f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x29, 0x2e, 0x0a, 0x20, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65,
    0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x73, 0x20,
    0x6c, 0x69, 0x73, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65,
    0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x73, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64,
    0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x0a, 0x20, 0x72,
    0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x73, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x21, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x21, 0x07, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x21, 0x23, 0x24, 0x0a, 0xba, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x27, 0x02, 0x26, 0x1a, 0xac, 0x02, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73,
    0x6f, 0x6c, 0x76, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79,
    0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x73, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x72, 0x65, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x20, 0x61, 0x76, 0x61, 0x69,
    0x6c, 0x61, 0x62, 0x6c, 0x65, 0x0a, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65,
    0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x49, 0x50,
    0x20, 0x66, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65,
    0x73, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x20, 0x66, 0x72, 0x6f,
    0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x69, 0x74,
    0x0a, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x20, 0x65, 0x2e, 0x67, 0x2e,
    0x2c, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e,
    0x6f, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x49, 0x50, 0x76, 0x34,
    0x20, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61,
    0x63, 0x65, 0x73, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65,
    0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6e, 0x6f, 0x74, 0x0a, 0x20, 0x70, 0x72, 0x6f, 0x76,
    0x69, 0x64, 0x65, 0x20, 0x49, 0x50, 0x76, 0x34, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x27,
    0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x27, 0x07, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x27, 0x24, 0x25, 0x0a, 0x69,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x2a, 0x02, 0x3d, 0x1a, 0x5c, 0x20, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x44,
    0x4e, 0x53, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x20, 0x6f, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x65, 0x68, 0x61, 0x76,
    0x69, 0x6f, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x44, 0x4e, 0x53, 0x20, 0x72,
    0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x2a, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x2a, 0x24, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x2a, 0x3b, 0x3c, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)