// @generated
// [#protodoc-title: RBAC]
// Role-Based Access Control :ref:`configuration overview <config_http_filters_rbac>`.
// [#extension: envoy.filters.http.rbac]

/// RBAC filter config.
/// [#next-free-field: 6]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rbac {
    /// Specify the RBAC rules to be applied globally.
    /// If absent, no enforcing RBAC policy will be applied.
    /// If present and empty, DENY.
    /// If both rules and matcher are configured, rules will be ignored.
    #[prost(message, optional, tag="1")]
    pub rules: ::core::option::Option<super::super::super::super::super::config::rbac::v3::Rbac>,
    /// The match tree to use when resolving RBAC action for incoming requests. Requests do not
    /// match any matcher will be denied.
    /// If absent, no enforcing RBAC matcher will be applied.
    /// If present and empty, deny all requests.
    #[prost(message, optional, tag="4")]
    pub matcher: ::core::option::Option<super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher>,
    /// Shadow rules are not enforced by the filter (i.e., returning a 403)
    /// but will emit stats and logs and can be used for rule testing.
    /// If absent, no shadow RBAC policy will be applied.
    /// If both shadow rules and shadow matcher are configured, shadow rules will be ignored.
    #[prost(message, optional, tag="2")]
    pub shadow_rules: ::core::option::Option<super::super::super::super::super::config::rbac::v3::Rbac>,
    /// The match tree to use for emitting stats and logs which can be used for rule testing for
    /// incoming requests.
    /// If absent, no shadow matcher will be applied.
    #[prost(message, optional, tag="5")]
    pub shadow_matcher: ::core::option::Option<super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher>,
    /// If specified, shadow rules will emit stats with the given prefix.
    /// This is useful to distinguish the stat when there are more than 1 RBAC filter configured with
    /// shadow rules.
    #[prost(string, tag="3")]
    pub shadow_rules_stat_prefix: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RbacPerRoute {
    /// Override the global configuration of the filter with this new config.
    /// If absent, the global RBAC policy will be disabled for this route.
    #[prost(message, optional, tag="2")]
    pub rbac: ::core::option::Option<Rbac>,
}
/// Encoded file descriptor set for the `envoy.extensions.filters.http.rbac.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x8c, 0x1a, 0x0a, 0x30, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68, 0x74,
    0x74, 0x70, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2f, 0x76, 0x33, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x25, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e,
    0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2f,
    0x76, 0x33, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x78,
    0x64, 0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76,
    0x33, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x21,
    0x78, 0x64, 0x73, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72,
    0x2f, 0x76, 0x33, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1e, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x21, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0xdd, 0x03, 0x0a, 0x04, 0x52, 0x42, 0x41, 0x43, 0x12, 0x49, 0x0a, 0x05,
    0x72, 0x75, 0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e,
    0x76, 0x33, 0x2e, 0x52, 0x42, 0x41, 0x43, 0x42, 0x17, 0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x11, 0x12,
    0x0f, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x5f, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x72,
    0x52, 0x05, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x12, 0x57, 0x0a, 0x07, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x78, 0x64, 0x73, 0x2e, 0x74,
    0x79, 0x70, 0x65, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x76, 0x33, 0x2e, 0x4d,
    0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x42, 0x1f, 0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x11, 0x12, 0x0f,
    0x72, 0x75, 0x6c, 0x65, 0x73, 0x5f, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x72, 0xd2,
    0xc6, 0xa4, 0xe1, 0x06, 0x02, 0x08, 0x01, 0x52, 0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72,
    0x12, 0x5d, 0x0a, 0x0c, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x73,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x33, 0x2e, 0x52, 0x42,
    0x41, 0x43, 0x42, 0x1e, 0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x18, 0x12, 0x16, 0x73, 0x68, 0x61, 0x64,
    0x6f, 0x77, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x5f, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69,
    0x65, 0x72, 0x52, 0x0b, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x52, 0x75, 0x6c, 0x65, 0x73, 0x12,
    0x6b, 0x0a, 0x0e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65,
    0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x78, 0x64, 0x73, 0x2e, 0x74, 0x79,
    0x70, 0x65, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x76, 0x33, 0x2e, 0x4d, 0x61,
    0x74, 0x63, 0x68, 0x65, 0x72, 0x42, 0x26, 0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x18, 0x12, 0x16, 0x73,
    0x68, 0x61, 0x64, 0x6f, 0x77, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x5f, 0x73, 0x70, 0x65, 0x63,
    0x69, 0x66, 0x69, 0x65, 0x72, 0xd2, 0xc6, 0xa4, 0xe1, 0x06, 0x02, 0x08, 0x01, 0x52, 0x0d, 0x73,
    0x68, 0x61, 0x64, 0x6f, 0x77, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x12, 0x37, 0x0a, 0x18,
    0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x5f, 0x73, 0x74, 0x61,
    0x74, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x15,
    0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x52, 0x75, 0x6c, 0x65, 0x73, 0x53, 0x74, 0x61, 0x74, 0x50,
    0x72, 0x65, 0x66, 0x69, 0x78, 0x3a, 0x2c, 0x9a, 0xc5, 0x88, 0x1e, 0x27, 0x0a, 0x25, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x32, 0x2e, 0x52,
    0x42, 0x41, 0x43, 0x22, 0x8b, 0x01, 0x0a, 0x0c, 0x52, 0x42, 0x41, 0x43, 0x50, 0x65, 0x72, 0x52,
    0x6f, 0x75, 0x74, 0x65, 0x12, 0x3f, 0x0a, 0x04, 0x72, 0x62, 0x61, 0x63, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74,
    0x74, 0x70, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x33, 0x2e, 0x52, 0x42, 0x41, 0x43, 0x52,
    0x04, 0x72, 0x62, 0x61, 0x63, 0x3a, 0x34, 0x9a, 0xc5, 0x88, 0x1e, 0x2f, 0x0a, 0x2d, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x32, 0x2e, 0x52,
    0x42, 0x41, 0x43, 0x50, 0x65, 0x72, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x4a, 0x04, 0x08, 0x01, 0x10,
    0x02, 0x42, 0x9f, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x33, 0x69, 0x6f,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76,
    0x33, 0x42, 0x09, 0x52, 0x62, 0x61, 0x63, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x53,
    0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f,
    0x68, 0x74, 0x74, 0x70, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2f, 0x76, 0x33, 0x3b, 0x72, 0x62, 0x61,
    0x63, 0x76, 0x33, 0x4a, 0xd1, 0x11, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x4b, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02,
    0x00, 0x2e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x07, 0x00, 0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x00, 0x28, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0a, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x0b, 0x00, 0x2b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x4c, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0d, 0x00, 0x4c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0e, 0x00, 0x2a, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x2a, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03,
    0x0f, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x10, 0x00, 0x6a, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0b, 0x12, 0x03, 0x10, 0x00, 0x6a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x11,
    0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x11, 0x00,
    0x46, 0x0a, 0xd0, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x19, 0x00, 0x40, 0x01, 0x1a, 0x2c,
    0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x6e, 0x65, 0x78, 0x74, 0x2d, 0x66, 0x72, 0x65,
    0x65, 0x2d, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x3a, 0x20, 0x36, 0x5d, 0x0a, 0x32, 0x95, 0x01, 0x20,
    0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65,
    0x3a, 0x20, 0x52, 0x42, 0x41, 0x43, 0x5d, 0x0a, 0x20, 0x52, 0x6f, 0x6c, 0x65, 0x2d, 0x42, 0x61,
    0x73, 0x65, 0x64, 0x20, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x20,
    0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x73, 0x5f, 0x72, 0x62, 0x61, 0x63, 0x3e, 0x60, 0x2e, 0x0a, 0x20, 0x5b, 0x23,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x62,
    0x61, 0x63, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x19, 0x08, 0x0c,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x1a, 0x02, 0x1b, 0x2e, 0x0a, 0x10, 0x0a,
    0x08, 0x04, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x1a, 0x02, 0x1b, 0x2e, 0x0a,
    0xd4, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x21, 0x02, 0x22, 0x4d, 0x1a, 0xc5,
    0x01, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x52, 0x42,
    0x41, 0x43, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x61,
    0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x20, 0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x6c, 0x79, 0x2e,
    0x0a, 0x20, 0x49, 0x66, 0x20, 0x61, 0x62, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x6e, 0x6f, 0x20,
    0x65, 0x6e, 0x66, 0x6f, 0x72, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x70,
    0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x70,
    0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65,
    0x6e, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2c, 0x20, 0x44, 0x45,
    0x4e, 0x59, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x72, 0x75, 0x6c,
    0x65, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x20, 0x61,
    0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x64, 0x2c, 0x20, 0x72,
    0x75, 0x6c, 0x65, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x69, 0x67, 0x6e,
    0x6f, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x21, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21,
    0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x1e, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x22, 0x06, 0x4c, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x00, 0x02, 0x00, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12, 0x03, 0x22, 0x07,
    0x4b, 0x0a, 0xec, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0x28, 0x02, 0x2b, 0x04,
    0x1a, 0xdd, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72,
    0x65, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x72,
    0x65, 0x73, 0x6f, 0x6c, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x63, 0x6f, 0x6d, 0x69, 0x6e,
    0x67, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2e, 0x20, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x73, 0x20, 0x64, 0x6f, 0x20, 0x6e, 0x6f, 0x74, 0x0a, 0x20, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x6e, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x20,
    0x49, 0x66, 0x20, 0x61, 0x62, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x6e, 0x6f, 0x20, 0x65, 0x6e,
    0x66, 0x6f, 0x72, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x70, 0x70,
    0x6c, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e,
    0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2c, 0x20, 0x64, 0x65, 0x6e,
    0x79, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x28, 0x02, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x28, 0x1e, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x28, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x08, 0x12, 0x04, 0x28, 0x2a, 0x2b, 0x03, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02,
    0x01, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12, 0x03, 0x29, 0x04, 0x48, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x00, 0x02, 0x01, 0x08, 0xea, 0xc8, 0x94, 0x6c, 0x01, 0x12, 0x03, 0x2a, 0x04, 0x3d, 0x0a,
    0x9e, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x31, 0x02, 0x32, 0x54, 0x1a, 0x8f,
    0x02, 0x20, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x20, 0x61,
    0x72, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x65, 0x6e, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x64, 0x20,
    0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x28, 0x69,
    0x2e, 0x65, 0x2e, 0x2c, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x61,
    0x20, 0x34, 0x30, 0x33, 0x29, 0x0a, 0x20, 0x62, 0x75, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x65, 0x6d, 0x69, 0x74, 0x20, 0x73, 0x74, 0x61, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6c,
    0x6f, 0x67, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x20, 0x74, 0x65, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x61, 0x62, 0x73, 0x65, 0x6e, 0x74,
    0x2c, 0x20, 0x6e, 0x6f, 0x20, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x20, 0x52, 0x42, 0x41, 0x43,
    0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20,
    0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x62, 0x6f, 0x74,
    0x68, 0x20, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65,
    0x72, 0x20, 0x61, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x64,
    0x2c, 0x20, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x64, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x31, 0x02, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x31, 0x16, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x31, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x08, 0x12, 0x03, 0x32, 0x06, 0x53, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x02,
    0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12, 0x03, 0x32, 0x07, 0x52, 0x0a, 0xac, 0x01, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x37, 0x02, 0x3a, 0x04, 0x1a, 0x9d, 0x01, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x65, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x75, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x6d, 0x69, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x20, 0x73, 0x74, 0x61, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6c, 0x6f, 0x67, 0x73, 0x20,
    0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x20, 0x74, 0x65, 0x73, 0x74, 0x69,
    0x6e, 0x67, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x69, 0x6e, 0x63, 0x6f, 0x6d, 0x69, 0x6e, 0x67,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x61,
    0x62, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x6e, 0x6f, 0x20, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77,
    0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65,
    0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x06, 0x12, 0x03, 0x37, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x37, 0x1e, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x37, 0x2f, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x08, 0x12, 0x04, 0x37,
    0x31, 0x3a, 0x03, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x03, 0x08, 0x8e, 0xe3, 0xff, 0x51,
    0x02, 0x12, 0x03, 0x38, 0x04, 0x4f, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x03, 0x08, 0xea,
    0xc8, 0x94, 0x6c, 0x01, 0x12, 0x03, 0x39, 0x04, 0x3d, 0x0a, 0xbf, 0x01, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x3f, 0x02, 0x26, 0x1a, 0xb1, 0x01, 0x20, 0x49, 0x66, 0x20, 0x73, 0x70,
    0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2c, 0x20, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x20,
    0x72, 0x75, 0x6c, 0x65, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x65, 0x6d, 0x69, 0x74, 0x20,
    0x73, 0x74, 0x61, 0x74, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67,
    0x69, 0x76, 0x65, 0x6e, 0x20, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x2e, 0x0a, 0x20, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x66, 0x75, 0x6c, 0x20, 0x74, 0x6f, 0x20,
    0x64, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x75, 0x69, 0x73, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x74, 0x61, 0x74, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20,
    0x61, 0x72, 0x65, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x31, 0x20,
    0x52, 0x42, 0x41, 0x43, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x75, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x73, 0x68, 0x61,
    0x64, 0x6f, 0x77, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x3f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x3f, 0x09, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x3f, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x42, 0x00, 0x4b,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x42, 0x08, 0x14, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x01, 0x07, 0x12, 0x04, 0x43, 0x02, 0x44, 0x36, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x01,
    0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x43, 0x02, 0x44, 0x36, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x09, 0x12, 0x03, 0x46, 0x02, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x09, 0x00,
    0x12, 0x03, 0x46, 0x0b, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x09, 0x00, 0x01, 0x12, 0x03,
    0x46, 0x0b, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x09, 0x00, 0x02, 0x12, 0x03, 0x46, 0x0b,
    0x0c, 0x0a, 0x99, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x02, 0x10, 0x1a,
    0x8b, 0x01, 0x20, 0x4f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6e, 0x65, 0x77,
    0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x61, 0x62, 0x73,
    0x65, 0x6e, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x20,
    0x52, 0x42, 0x41, 0x43, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x62, 0x65, 0x20, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4a, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4a, 0x07, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x4a, 0x0e, 0x0f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.http.rbac.v3.serde.rs");
// @@protoc_insertion_point(module)