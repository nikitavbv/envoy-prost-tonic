// @generated
// [#protodoc-title: Decompressor]
// [#extension: envoy.filters.http.decompressor]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decompressor {
    /// A decompressor library to use for both request and response decompression. Currently only
    /// :ref:`envoy.compression.gzip.compressor<envoy_v3_api_msg_extensions.compression.gzip.decompressor.v3.Gzip>`
    /// is included in Envoy.
    /// [#extension-category: envoy.compression.decompressor]
    #[prost(message, optional, tag="1")]
    pub decompressor_library: ::core::option::Option<super::super::super::super::super::config::core::v3::TypedExtensionConfig>,
    /// Configuration for request decompression. Decompression is enabled by default if left empty.
    #[prost(message, optional, tag="2")]
    pub request_direction_config: ::core::option::Option<decompressor::RequestDirectionConfig>,
    /// Configuration for response decompression. Decompression is enabled by default if left empty.
    #[prost(message, optional, tag="3")]
    pub response_direction_config: ::core::option::Option<decompressor::ResponseDirectionConfig>,
}
/// Nested message and enum types in `Decompressor`.
pub mod decompressor {
    /// Common configuration for filter behavior on both the request and response direction.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommonDirectionConfig {
        /// Runtime flag that controls whether the filter is enabled for decompression or not. If set to false, the
        /// filter will operate as a pass-through filter. If the message is unspecified, the filter will be enabled.
        #[prost(message, optional, tag="1")]
        pub enabled: ::core::option::Option<super::super::super::super::super::super::config::core::v3::RuntimeFeatureFlag>,
        /// If set to true, will decompress response even if a ``no-transform`` cache control header is set.
        #[prost(bool, tag="2")]
        pub ignore_no_transform_header: bool,
    }
    /// Configuration for filter behavior on the request direction.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestDirectionConfig {
        #[prost(message, optional, tag="1")]
        pub common_config: ::core::option::Option<CommonDirectionConfig>,
        /// If set to true, and response decompression is enabled, the filter modifies the Accept-Encoding
        /// request header by appending the decompressor_library's encoding. Defaults to true.
        #[prost(message, optional, tag="2")]
        pub advertise_accept_encoding: ::core::option::Option<::pbjson_types::BoolValue>,
    }
    /// Configuration for filter behavior on the response direction.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResponseDirectionConfig {
        #[prost(message, optional, tag="1")]
        pub common_config: ::core::option::Option<CommonDirectionConfig>,
    }
}
/// Encoded file descriptor set for the `envoy.extensions.filters.http.decompressor.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb3, 0x1b, 0x0a, 0x40, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68, 0x74,
    0x74, 0x70, 0x2f, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2f,
    0x76, 0x33, 0x2f, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e,
    0x68, 0x74, 0x74, 0x70, 0x2e, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f,
    0x72, 0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x24, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x77, 0x72, 0x61,
    0x70, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70,
    0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0xb2, 0x07, 0x0a, 0x0c, 0x44, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x6f, 0x72, 0x12, 0x67, 0x0a, 0x14, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x6f, 0x72, 0x5f, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x64, 0x45,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x42, 0x08,
    0xfa, 0x42, 0x05, 0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x13, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70,
    0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x4c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x12, 0x8c, 0x01,
    0x0a, 0x18, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x52, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70,
    0x2e, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x76, 0x33,
    0x2e, 0x44, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x52, 0x16, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x44, 0x69, 0x72,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x8f, 0x01, 0x0a,
    0x19, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x53, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70,
    0x2e, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x76, 0x33,
    0x2e, 0x44, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x17, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x44,
    0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x1a, 0x98,
    0x01, 0x0a, 0x15, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x42, 0x0a, 0x07, 0x65, 0x6e, 0x61, 0x62,
    0x6c, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33,
    0x2e, 0x52, 0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x46,
    0x6c, 0x61, 0x67, 0x52, 0x07, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x12, 0x3b, 0x0a, 0x1a,
    0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x5f, 0x6e, 0x6f, 0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66,
    0x6f, 0x72, 0x6d, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x17, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x4e, 0x6f, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66,
    0x6f, 0x72, 0x6d, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x1a, 0xe8, 0x01, 0x0a, 0x16, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x12, 0x76, 0x0a, 0x0d, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x51, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x64, 0x65, 0x63, 0x6f,
    0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x76, 0x33, 0x2e, 0x44, 0x65, 0x63, 0x6f,
    0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x44,
    0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x0c,
    0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x56, 0x0a, 0x19,
    0x61, 0x64, 0x76, 0x65, 0x72, 0x74, 0x69, 0x73, 0x65, 0x5f, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74,
    0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2e, 0x42, 0x6f, 0x6f, 0x6c, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x17, 0x61, 0x64, 0x76,
    0x65, 0x72, 0x74, 0x69, 0x73, 0x65, 0x41, 0x63, 0x63, 0x65, 0x70, 0x74, 0x45, 0x6e, 0x63, 0x6f,
    0x64, 0x69, 0x6e, 0x67, 0x1a, 0x91, 0x01, 0x0a, 0x17, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x12, 0x76, 0x0a, 0x0d, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x51, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x6f, 0x72, 0x2e, 0x76, 0x33, 0x2e, 0x44, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x6f, 0x72, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x44, 0x69, 0x72, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x0c, 0x63, 0x6f, 0x6d, 0x6d,
    0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x42, 0xbf, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06,
    0x02, 0x10, 0x02, 0x0a, 0x3b, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f,
    0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70,
    0x2e, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x76, 0x33,
    0x42, 0x11, 0x44, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x63, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
    0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x64, 0x65, 0x63, 0x6f,
    0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2f, 0x76, 0x33, 0x3b, 0x64, 0x65, 0x63, 0x6f,
    0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x76, 0x33, 0x4a, 0xa1, 0x10, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x3a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x36, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x04, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x2e, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x09, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0a, 0x00, 0x21,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x54, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x0c, 0x00, 0x54, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x32, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x32, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0e, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0e, 0x00, 0x22, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x7a, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12,
    0x03, 0x0f, 0x00, 0x7a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x10, 0x00, 0x46, 0x0a, 0x0d,
    0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x10, 0x00, 0x46, 0x0a, 0x5c, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x15, 0x00, 0x3a, 0x01, 0x32, 0x50, 0x20, 0x5b, 0x23, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x44, 0x65,
    0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x64, 0x65, 0x63,
    0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x15, 0x08, 0x14, 0x0a, 0x64, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12,
    0x04, 0x17, 0x02, 0x1e, 0x03, 0x1a, 0x56, 0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x20, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x62, 0x65, 0x68, 0x61, 0x76, 0x69, 0x6f, 0x72,
    0x20, 0x6f, 0x6e, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x17, 0x0a, 0x1f, 0x0a, 0xe3, 0x01, 0x0a, 0x06,
    0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x04, 0x32, 0x1a, 0xd3, 0x01, 0x20, 0x52,
    0x75, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x66, 0x6c, 0x61, 0x67, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20,
    0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x64, 0x65, 0x63, 0x6f,
    0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74,
    0x2e, 0x20, 0x49, 0x66, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x61, 0x6c, 0x73,
    0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x65, 0x20, 0x61, 0x73, 0x20, 0x61,
    0x20, 0x70, 0x61, 0x73, 0x73, 0x2d, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x20, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x75, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66,
    0x69, 0x65, 0x64, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x2e,
    0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1a, 0x04,
    0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x26,
    0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x30,
    0x31, 0x0a, 0x71, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x04, 0x28,
    0x1a, 0x62, 0x20, 0x49, 0x66, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x72, 0x75,
    0x65, 0x2c, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e,
    0x20, 0x69, 0x66, 0x20, 0x61, 0x20, 0x60, 0x60, 0x6e, 0x6f, 0x2d, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x66, 0x6f, 0x72, 0x6d, 0x60, 0x60, 0x20, 0x63, 0x61, 0x63, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x73,
    0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x1d, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x1d, 0x09, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x1d, 0x26, 0x27, 0x0a, 0x4b, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x01, 0x12, 0x04, 0x21, 0x02,
    0x27, 0x03, 0x1a, 0x3d, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x62, 0x65,
    0x68, 0x61, 0x76, 0x69, 0x6f, 0x72, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x01, 0x01, 0x12, 0x03, 0x21, 0x0a, 0x20, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x2c, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x22, 0x04, 0x19, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x1a, 0x27, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x2a, 0x2b, 0x0a, 0xc4,
    0x01, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x26, 0x04, 0x3c, 0x1a, 0xb4,
    0x01, 0x20, 0x49, 0x66, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x72, 0x75, 0x65,
    0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x64,
    0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20,
    0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x20, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x41, 0x63, 0x63, 0x65, 0x70, 0x74, 0x2d, 0x45, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67,
    0x0a, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x20, 0x62, 0x79, 0x20, 0x61, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x5f, 0x6c,
    0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x27, 0x73, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e,
    0x67, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x72, 0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x26, 0x04, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x26, 0x1e, 0x37, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x26, 0x3a, 0x3b, 0x0a, 0x4c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x02, 0x12, 0x04, 0x2a,
    0x02, 0x2c, 0x03, 0x1a, 0x3e, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x62,
    0x65, 0x68, 0x61, 0x76, 0x69, 0x6f, 0x72, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x0a,
    0x21, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x04, 0x2c,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2b, 0x04, 0x19,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x1a, 0x27,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x2a, 0x2b,
    0x0a, 0xa5, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x32, 0x02, 0x33, 0x34, 0x1a,
    0x96, 0x02, 0x20, 0x41, 0x20, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f,
    0x72, 0x20, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x64,
    0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x43, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x0a, 0x20, 0x3a, 0x72,
    0x65, 0x66, 0x3a, 0x60, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x67, 0x7a, 0x69, 0x70, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x72,
    0x65, 0x73, 0x73, 0x6f, 0x72, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61,
    0x70, 0x69, 0x5f, 0x6d, 0x73, 0x67, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x67, 0x7a,
    0x69, 0x70, 0x2e, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e,
    0x76, 0x33, 0x2e, 0x47, 0x7a, 0x69, 0x70, 0x3e, 0x60, 0x0a, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6e,
    0x63, 0x6c, 0x75, 0x64, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2d, 0x63, 0x61,
    0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f,
    0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70,
    0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x5d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x32, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x32, 0x26, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32,
    0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x33, 0x06, 0x33,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x11, 0x12, 0x03, 0x33, 0x07,
    0x32, 0x0a, 0x6a, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x36, 0x02, 0x36, 0x1a, 0x5d,
    0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x64, 0x65, 0x63, 0x6f, 0x6d,
    0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x44, 0x65, 0x63, 0x6f, 0x6d, 0x70,
    0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x65, 0x6e, 0x61, 0x62, 0x6c,
    0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x66,
    0x20, 0x6c, 0x65, 0x66, 0x74, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x36, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x36, 0x19, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x36, 0x34, 0x35, 0x0a, 0x6b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x39, 0x02, 0x38, 0x1a, 0x5e, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x20, 0x64, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e,
    0x20, 0x44, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69,
    0x73, 0x20, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x64, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x66, 0x20, 0x6c, 0x65, 0x66, 0x74, 0x20, 0x65, 0x6d, 0x70,
    0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x39,
    0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x39, 0x1a, 0x33,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x39, 0x36, 0x37, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.http.decompressor.v3.serde.rs");
// @@protoc_insertion_point(module)