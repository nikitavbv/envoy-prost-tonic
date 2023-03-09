// @generated
// [#protodoc-title: RBAC]
// Role-Based Access Control :ref:`configuration overview <config_network_filters_rbac>`.
// [#extension: envoy.filters.network.rbac]

/// RBAC network filter config.
///
/// Header should not be used in rules/shadow_rules in RBAC network filter as
/// this information is only available in :ref:`RBAC http filter <config_http_filters_rbac>`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rbac {
    /// Specify the RBAC rules to be applied globally.
    /// If absent, no enforcing RBAC policy will be applied.
    #[prost(message, optional, tag="1")]
    pub rules: ::core::option::Option<super::super::super::super::rbac::v2::Rbac>,
    /// Shadow rules are not enforced by the filter but will emit stats and logs
    /// and can be used for rule testing.
    /// If absent, no shadow RBAC policy will be applied.
    #[prost(message, optional, tag="2")]
    pub shadow_rules: ::core::option::Option<super::super::super::super::rbac::v2::Rbac>,
    /// The prefix to use when emitting statistics.
    #[prost(string, tag="3")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// RBAC enforcement strategy. By default RBAC will be enforced only once
    /// when the first byte of data arrives from the downstream. When used in
    /// conjunction with filters that emit dynamic metadata after decoding
    /// every payload (e.g., Mongo, MySQL, Kafka) set the enforcement type to
    /// CONTINUOUS to enforce RBAC policies on every message boundary.
    #[prost(enumeration="rbac::EnforcementType", tag="4")]
    pub enforcement_type: i32,
}
/// Nested message and enum types in `RBAC`.
pub mod rbac {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EnforcementType {
        /// Apply RBAC policies when the first byte of data arrives on the connection.
        OneTimeOnFirstByte = 0,
        /// Continuously apply RBAC policies as data arrives. Use this mode when
        /// using RBAC with message oriented protocols such as Mongo, MySQL, Kafka,
        /// etc. when the protocol decoders emit dynamic metadata such as the
        /// resources being accessed and the operations on the resources.
        Continuous = 1,
    }
    impl EnforcementType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EnforcementType::OneTimeOnFirstByte => "ONE_TIME_ON_FIRST_BYTE",
                EnforcementType::Continuous => "CONTINUOUS",
            }
        }
    }
}
/// Encoded file descriptor set for the `envoy.config.filter.network.rbac.v2` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xac, 0x15, 0x0a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x2f, 0x72, 0x62, 0x61, 0x63, 0x2f, 0x76, 0x32, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x23, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x32, 0x1a, 0x1f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2f, 0x76, 0x32, 0x2f, 0x72,
    0x62, 0x61, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x75, 0x64, 0x70, 0x61, 0x2f,
    0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x69, 0x67, 0x72,
    0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f,
    0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0xc6, 0x02, 0x0a, 0x04, 0x52, 0x42, 0x41, 0x43, 0x12, 0x30, 0x0a, 0x05, 0x72, 0x75,
    0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x32,
    0x2e, 0x52, 0x42, 0x41, 0x43, 0x52, 0x05, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x12, 0x3d, 0x0a, 0x0c,
    0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x32, 0x2e, 0x52, 0x42, 0x41, 0x43, 0x52, 0x0b,
    0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x52, 0x75, 0x6c, 0x65, 0x73, 0x12, 0x28, 0x0a, 0x0b, 0x73,
    0x74, 0x61, 0x74, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x20, 0x01, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x50,
    0x72, 0x65, 0x66, 0x69, 0x78, 0x12, 0x64, 0x0a, 0x10, 0x65, 0x6e, 0x66, 0x6f, 0x72, 0x63, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x39, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x72, 0x62,
    0x61, 0x63, 0x2e, 0x76, 0x32, 0x2e, 0x52, 0x42, 0x41, 0x43, 0x2e, 0x45, 0x6e, 0x66, 0x6f, 0x72,
    0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0f, 0x65, 0x6e, 0x66, 0x6f,
    0x72, 0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x22, 0x3d, 0x0a, 0x0f, 0x45,
    0x6e, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1a,
    0x0a, 0x16, 0x4f, 0x4e, 0x45, 0x5f, 0x54, 0x49, 0x4d, 0x45, 0x5f, 0x4f, 0x4e, 0x5f, 0x46, 0x49,
    0x52, 0x53, 0x54, 0x5f, 0x42, 0x59, 0x54, 0x45, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x43, 0x4f,
    0x4e, 0x54, 0x49, 0x4e, 0x55, 0x4f, 0x55, 0x53, 0x10, 0x01, 0x42, 0xcb, 0x01, 0xf2, 0x98, 0xfe,
    0x8f, 0x05, 0x2a, 0x12, 0x28, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x33, 0xba, 0x80, 0xc8,
    0xd1, 0x06, 0x02, 0x10, 0x01, 0x0a, 0x31, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70,
    0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x32, 0x42, 0x09, 0x52, 0x62, 0x61, 0x63, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x51, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
    0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2f, 0x76,
    0x32, 0x3b, 0x72, 0x62, 0x61, 0x63, 0x76, 0x32, 0x4a, 0xbc, 0x0f, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x37, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04,
    0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x28, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x08, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x4a, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x0a, 0x00, 0x4a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b,
    0x00, 0x2a, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x2a, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0c,
    0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x68, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x0b, 0x12, 0x03, 0x0d, 0x00, 0x68, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00,
    0x64, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12, 0x03, 0x0e, 0x00, 0x64,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87,
    0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0xef, 0x02, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x19, 0x00, 0x37, 0x01, 0x1a, 0xc4, 0x01, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x0a, 0x0a, 0x20, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x69, 0x6e, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x2f, 0x73, 0x68, 0x61, 0x64, 0x6f,
    0x77, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x61,
    0x73, 0x0a, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x61, 0x76, 0x61, 0x69,
    0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x52,
    0x42, 0x41, 0x43, 0x20, 0x68, 0x74, 0x74, 0x70, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20,
    0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x73, 0x5f, 0x72, 0x62, 0x61, 0x63, 0x3e, 0x60, 0x2e, 0x0a, 0x32, 0x9b, 0x01,
    0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c,
    0x65, 0x3a, 0x20, 0x52, 0x42, 0x41, 0x43, 0x5d, 0x0a, 0x20, 0x52, 0x6f, 0x6c, 0x65, 0x2d, 0x42,
    0x61, 0x73, 0x65, 0x64, 0x20, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x43, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77,
    0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x72, 0x62, 0x61, 0x63, 0x3e, 0x60, 0x2e,
    0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x19, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12,
    0x04, 0x1a, 0x02, 0x23, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x1a, 0x07, 0x16, 0x0a, 0x5b, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1c,
    0x04, 0x1f, 0x1a, 0x4c, 0x20, 0x41, 0x70, 0x70, 0x6c, 0x79, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20,
    0x70, 0x6f, 0x6c, 0x69, 0x63, 0x69, 0x65, 0x73, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x62, 0x79, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x64, 0x61, 0x74, 0x61, 0x20, 0x61, 0x72, 0x72, 0x69, 0x76, 0x65, 0x73, 0x20, 0x6f, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x04, 0x1a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1c, 0x1d, 0x1e,
    0x0a, 0xa1, 0x02, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x22, 0x04, 0x13,
    0x1a, 0x91, 0x02, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x6f, 0x75, 0x73, 0x6c, 0x79,
    0x20, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x70, 0x6f, 0x6c, 0x69,
    0x63, 0x69, 0x65, 0x73, 0x20, 0x61, 0x73, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x61, 0x72, 0x72,
    0x69, 0x76, 0x65, 0x73, 0x2e, 0x20, 0x55, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d,
    0x6f, 0x64, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x0a, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20,
    0x52, 0x42, 0x41, 0x43, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x20, 0x6f, 0x72, 0x69, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x73, 0x20, 0x73, 0x75, 0x63, 0x68, 0x20, 0x61, 0x73, 0x20, 0x4d, 0x6f, 0x6e,
    0x67, 0x6f, 0x2c, 0x20, 0x4d, 0x79, 0x53, 0x51, 0x4c, 0x2c, 0x20, 0x4b, 0x61, 0x66, 0x6b, 0x61,
    0x2c, 0x0a, 0x20, 0x65, 0x74, 0x63, 0x2e, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x64, 0x65, 0x63, 0x6f, 0x64, 0x65,
    0x72, 0x73, 0x20, 0x65, 0x6d, 0x69, 0x74, 0x20, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x20,
    0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x75, 0x63, 0x68, 0x20, 0x61, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20,
    0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x22, 0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x22, 0x11, 0x12, 0x0a, 0x73, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x27, 0x02,
    0x20, 0x1a, 0x66, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x52, 0x42, 0x41, 0x43, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65,
    0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x20, 0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x6c,
    0x79, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x61, 0x62, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x6e,
    0x6f, 0x20, 0x65, 0x6e, 0x66, 0x6f, 0x72, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x52, 0x42, 0x41, 0x43,
    0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20,
    0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x27, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x27, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x27, 0x1e, 0x1f, 0x0a, 0xae, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x02,
    0x27, 0x1a, 0xa0, 0x01, 0x20, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x20, 0x72, 0x75, 0x6c, 0x65,
    0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x65, 0x6e, 0x66, 0x6f, 0x72, 0x63,
    0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x20, 0x62, 0x75, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x65, 0x6d, 0x69, 0x74, 0x20, 0x73,
    0x74, 0x61, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6c, 0x6f, 0x67, 0x73, 0x0a, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x20, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x2e,
    0x0a, 0x20, 0x49, 0x66, 0x20, 0x61, 0x62, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x6e, 0x6f, 0x20,
    0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x70, 0x6f, 0x6c, 0x69,
    0x63, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69,
    0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2c,
    0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x16, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2c, 0x25, 0x26, 0x0a, 0x3a,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x02, 0x44, 0x1a, 0x2d, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20,
    0x77, 0x68, 0x65, 0x6e, 0x20, 0x65, 0x6d, 0x69, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x74,
    0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x2f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x2f, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x2f, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x2f,
    0x19, 0x43, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x02, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03,
    0x2f, 0x1a, 0x42, 0x0a, 0xe7, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x36, 0x02,
    0x27, 0x1a, 0xd9, 0x02, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x65, 0x6e, 0x66, 0x6f, 0x72, 0x63,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x74, 0x72, 0x61, 0x74, 0x65, 0x67, 0x79, 0x2e, 0x20,
    0x42, 0x79, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x65, 0x6e, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x64,
    0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x63, 0x65, 0x0a, 0x20, 0x77, 0x68, 0x65, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x62, 0x79, 0x74, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x61, 0x72, 0x72, 0x69, 0x76, 0x65, 0x73, 0x20,
    0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x2e, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69,
    0x6e, 0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x6a, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x65, 0x6d, 0x69, 0x74, 0x20, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x20, 0x6d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x64, 0x65, 0x63,
    0x6f, 0x64, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x70, 0x61, 0x79,
    0x6c, 0x6f, 0x61, 0x64, 0x20, 0x28, 0x65, 0x2e, 0x67, 0x2e, 0x2c, 0x20, 0x4d, 0x6f, 0x6e, 0x67,
    0x6f, 0x2c, 0x20, 0x4d, 0x79, 0x53, 0x51, 0x4c, 0x2c, 0x20, 0x4b, 0x61, 0x66, 0x6b, 0x61, 0x29,
    0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x66, 0x6f, 0x72, 0x63, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x43, 0x4f,
    0x4e, 0x54, 0x49, 0x4e, 0x55, 0x4f, 0x55, 0x53, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x6e, 0x66, 0x6f,
    0x72, 0x63, 0x65, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x69, 0x65,
    0x73, 0x20, 0x6f, 0x6e, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x20, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x61, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x36, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x36, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x36, 0x25, 0x26, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)