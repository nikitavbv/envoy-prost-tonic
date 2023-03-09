// @generated
// [#protodoc-title: Brotli Compressor]
// [#extension: envoy.compression.brotli.compressor]

/// [#next-free-field: 7]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brotli {
    /// Value from 0 to 11 that controls the main compression speed-density lever.
    /// The higher quality, the slower compression. The default value is 3.
    #[prost(message, optional, tag="1")]
    pub quality: ::core::option::Option<::pbjson_types::UInt32Value>,
    /// A value used to tune encoder for specific input. For more information about modes,
    /// please refer to brotli manual: <https://brotli.org/encode.html#aa6f>
    /// This field will be set to "DEFAULT" if not specified.
    #[prost(enumeration="brotli::EncoderMode", tag="2")]
    pub encoder_mode: i32,
    /// Value from 10 to 24 that represents the base two logarithmic of the compressor's window size.
    /// Larger window results in better compression at the expense of memory usage. The default is 18.
    /// For more details about this parameter, please refer to brotli manual:
    /// <https://brotli.org/encode.html#a9a8>
    #[prost(message, optional, tag="3")]
    pub window_bits: ::core::option::Option<::pbjson_types::UInt32Value>,
    /// Value from 16 to 24 that represents the base two logarithmic of the compressor's input block
    /// size. Larger input block results in better compression at the expense of memory usage. The
    /// default is 24. For more details about this parameter, please refer to brotli manual:
    /// <https://brotli.org/encode.html#a9a8>
    #[prost(message, optional, tag="4")]
    pub input_block_bits: ::core::option::Option<::pbjson_types::UInt32Value>,
    /// Value for compressor's next output buffer. If not set, defaults to 4096.
    #[prost(message, optional, tag="5")]
    pub chunk_size: ::core::option::Option<::pbjson_types::UInt32Value>,
    /// If true, disables "literal context modeling" format feature.
    /// This flag is a "decoding-speed vs compression ratio" trade-off.
    #[prost(bool, tag="6")]
    pub disable_literal_context_modeling: bool,
}
/// Nested message and enum types in `Brotli`.
pub mod brotli {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EncoderMode {
        Default = 0,
        Generic = 1,
        Text = 2,
        Font = 3,
    }
    impl EncoderMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EncoderMode::Default => "DEFAULT",
                EncoderMode::Generic => "GENERIC",
                EncoderMode::Text => "TEXT",
                EncoderMode::Font => "FONT",
            }
        }
    }
}
/// Encoded file descriptor set for the `envoy.extensions.compression.brotli.compressor.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd7, 0x18, 0x0a, 0x3e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x2f, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x6f, 0x72, 0x2f, 0x76, 0x33, 0x2f, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x31, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x2e, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x6f, 0x72, 0x2e, 0x76, 0x33, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e,
    0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xab,
    0x04, 0x0a, 0x06, 0x42, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x12, 0x3f, 0x0a, 0x07, 0x71, 0x75, 0x61,
    0x6c, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x55, 0x49, 0x6e,
    0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x2a, 0x02, 0x18,
    0x0b, 0x52, 0x07, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x12, 0x72, 0x0a, 0x0c, 0x65, 0x6e,
    0x63, 0x6f, 0x64, 0x65, 0x72, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x45, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e,
    0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f,
    0x72, 0x2e, 0x76, 0x33, 0x2e, 0x42, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e, 0x45, 0x6e, 0x63, 0x6f,
    0x64, 0x65, 0x72, 0x4d, 0x6f, 0x64, 0x65, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x82, 0x01, 0x02, 0x10,
    0x01, 0x52, 0x0b, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x72, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x48,
    0x0a, 0x0b, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x5f, 0x62, 0x69, 0x74, 0x73, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x55, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x42, 0x09, 0xfa, 0x42, 0x06, 0x2a, 0x04, 0x18, 0x18, 0x28, 0x0a, 0x52, 0x0a, 0x77, 0x69,
    0x6e, 0x64, 0x6f, 0x77, 0x42, 0x69, 0x74, 0x73, 0x12, 0x51, 0x0a, 0x10, 0x69, 0x6e, 0x70, 0x75,
    0x74, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x62, 0x69, 0x74, 0x73, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2e, 0x55, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65,
    0x42, 0x09, 0xfa, 0x42, 0x06, 0x2a, 0x04, 0x18, 0x18, 0x28, 0x10, 0x52, 0x0e, 0x69, 0x6e, 0x70,
    0x75, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x42, 0x69, 0x74, 0x73, 0x12, 0x49, 0x0a, 0x0a, 0x63,
    0x68, 0x75, 0x6e, 0x6b, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2e, 0x55, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x0c, 0xfa,
    0x42, 0x09, 0x2a, 0x07, 0x18, 0x80, 0x80, 0x04, 0x28, 0x80, 0x20, 0x52, 0x09, 0x63, 0x68, 0x75,
    0x6e, 0x6b, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x47, 0x0a, 0x20, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c,
    0x65, 0x5f, 0x6c, 0x69, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78,
    0x74, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x69, 0x6e, 0x67, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x1d, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x4c, 0x69, 0x74, 0x65, 0x72, 0x61, 0x6c,
    0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x69, 0x6e, 0x67, 0x22,
    0x3b, 0x0a, 0x0b, 0x45, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x72, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x0b,
    0x0a, 0x07, 0x44, 0x45, 0x46, 0x41, 0x55, 0x4c, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x47,
    0x45, 0x4e, 0x45, 0x52, 0x49, 0x43, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x54, 0x45, 0x58, 0x54,
    0x10, 0x02, 0x12, 0x08, 0x0a, 0x04, 0x46, 0x4f, 0x4e, 0x54, 0x10, 0x03, 0x42, 0xbf, 0x01, 0xba,
    0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x3f, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x2e, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x72,
    0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x76, 0x33, 0x42, 0x0b, 0x42, 0x72, 0x6f, 0x74, 0x6c, 0x69,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x65, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e,
    0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67,
    0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2f, 0x62, 0x72, 0x6f,
    0x74, 0x6c, 0x69, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2f, 0x76,
    0x33, 0x3b, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x76, 0x33, 0x4a, 0x91,
    0x11, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x36, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x3a, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x06, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x21, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x58, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x09, 0x00, 0x58, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x2c, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x2c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b,
    0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x7c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0c,
    0x00, 0x7c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06,
    0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x7e, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x13, 0x00, 0x36, 0x01, 0x1a, 0x17, 0x20, 0x5b, 0x23, 0x6e, 0x65, 0x78, 0x74,
    0x2d, 0x66, 0x72, 0x65, 0x65, 0x2d, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x3a, 0x20, 0x37, 0x5d, 0x0a,
    0x32, 0x59, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69,
    0x74, 0x6c, 0x65, 0x3a, 0x20, 0x42, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x20, 0x43, 0x6f, 0x6d, 0x70,
    0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6d, 0x70,
    0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e, 0x63,
    0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x13, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12,
    0x04, 0x14, 0x02, 0x19, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x14, 0x07, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15,
    0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15,
    0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x15,
    0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x16, 0x04,
    0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x04,
    0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x16, 0x0e,
    0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x17, 0x04, 0x0d,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x04, 0x08,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x17, 0x0b, 0x0c,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x18, 0x04, 0x0d, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x18, 0x04, 0x08, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x18, 0x0b, 0x0c, 0x0a,
    0x9f, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02, 0x50, 0x1a, 0x91, 0x01,
    0x20, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x30, 0x20, 0x74, 0x6f,
    0x20, 0x31, 0x31, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72,
    0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x70, 0x65, 0x65, 0x64, 0x2d, 0x64, 0x65, 0x6e,
    0x73, 0x69, 0x74, 0x79, 0x20, 0x6c, 0x65, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x68, 0x69, 0x67, 0x68, 0x65, 0x72, 0x20, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x2c,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6c, 0x6f, 0x77, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x70,
    0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x69, 0x73, 0x20, 0x33, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1d, 0x02, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x1e, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x1d, 0x2a, 0x4f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02,
    0x00, 0x08, 0xaf, 0x08, 0x05, 0x12, 0x03, 0x1d, 0x2b, 0x4e, 0x0a, 0xdd, 0x01, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x22, 0x02, 0x4e, 0x1a, 0xcf, 0x01, 0x20, 0x41, 0x20, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x75, 0x6e, 0x65,
    0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x20, 0x46, 0x6f, 0x72,
    0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x73, 0x2c, 0x0a, 0x20,
    0x70, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20,
    0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x20, 0x6d, 0x61, 0x6e, 0x75, 0x61, 0x6c, 0x3a, 0x20, 0x68,
    0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e, 0x6f, 0x72,
    0x67, 0x2f, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x2e, 0x68, 0x74, 0x6d, 0x6c, 0x23, 0x61, 0x61,
    0x36, 0x66, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x22, 0x44,
    0x45, 0x46, 0x41, 0x55, 0x4c, 0x54, 0x22, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73,
    0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x22, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x22, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x22, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x22,
    0x1f, 0x4d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x10, 0x12, 0x03,
    0x22, 0x20, 0x4c, 0x0a, 0xb9, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x28, 0x02,
    0x5c, 0x1a, 0xab, 0x02, 0x20, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x31, 0x30, 0x20, 0x74, 0x6f, 0x20, 0x32, 0x34, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x72, 0x65,
    0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x61, 0x73,
    0x65, 0x20, 0x74, 0x77, 0x6f, 0x20, 0x6c, 0x6f, 0x67, 0x61, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x69,
    0x63, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x6f, 0x72, 0x27, 0x73, 0x20, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x20, 0x73, 0x69, 0x7a,
    0x65, 0x2e, 0x0a, 0x20, 0x4c, 0x61, 0x72, 0x67, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6e, 0x64, 0x6f,
    0x77, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x62, 0x65, 0x74,
    0x74, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20,
    0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x70, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x20, 0x75, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x73, 0x20, 0x31,
    0x38, 0x2e, 0x0a, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x64, 0x65, 0x74,
    0x61, 0x69, 0x6c, 0x73, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2c, 0x20, 0x70, 0x6c, 0x65, 0x61, 0x73,
    0x65, 0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x72, 0x6f, 0x74, 0x6c,
    0x69, 0x20, 0x6d, 0x61, 0x6e, 0x75, 0x61, 0x6c, 0x3a, 0x0a, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73,
    0x3a, 0x2f, 0x2f, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e, 0x6f, 0x72, 0x67, 0x2f, 0x65, 0x6e,
    0x63, 0x6f, 0x64, 0x65, 0x2e, 0x68, 0x74, 0x6d, 0x6c, 0x23, 0x61, 0x39, 0x61, 0x38, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x28, 0x02, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x28, 0x1e, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x28, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x08, 0x12, 0x03, 0x28, 0x2e, 0x5b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x02, 0x08,
    0xaf, 0x08, 0x05, 0x12, 0x03, 0x28, 0x2f, 0x5a, 0x0a, 0xc3, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x2e, 0x02, 0x61, 0x1a, 0xb5, 0x02, 0x20, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x20,
    0x66, 0x72, 0x6f, 0x6d, 0x20, 0x31, 0x36, 0x20, 0x74, 0x6f, 0x20, 0x32, 0x34, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x62, 0x61, 0x73, 0x65, 0x20, 0x74, 0x77, 0x6f, 0x20, 0x6c, 0x6f, 0x67, 0x61, 0x72,
    0x69, 0x74, 0x68, 0x6d, 0x69, 0x63, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f,
    0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x27, 0x73, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74,
    0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x0a, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x2e, 0x20, 0x4c, 0x61,
    0x72, 0x67, 0x65, 0x72, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x62, 0x65, 0x74, 0x74,
    0x65, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x61,
    0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x70, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x20, 0x75, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x20, 0x54,
    0x68, 0x65, 0x0a, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x73, 0x20, 0x32,
    0x34, 0x2e, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x64, 0x65, 0x74, 0x61,
    0x69, 0x6c, 0x73, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70,
    0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2c, 0x20, 0x70, 0x6c, 0x65, 0x61, 0x73, 0x65,
    0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69,
    0x20, 0x6d, 0x61, 0x6e, 0x75, 0x61, 0x6c, 0x3a, 0x0a, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a,
    0x2f, 0x2f, 0x62, 0x72, 0x6f, 0x74, 0x6c, 0x69, 0x2e, 0x6f, 0x72, 0x67, 0x2f, 0x65, 0x6e, 0x63,
    0x6f, 0x64, 0x65, 0x2e, 0x68, 0x74, 0x6d, 0x6c, 0x23, 0x61, 0x39, 0x61, 0x38, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2e, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2e, 0x1e, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x2e, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x08, 0x12, 0x03, 0x2e, 0x33, 0x60, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x03, 0x08, 0xaf,
    0x08, 0x05, 0x12, 0x03, 0x2e, 0x34, 0x5f, 0x0a, 0x57, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12,
    0x03, 0x31, 0x02, 0x60, 0x1a, 0x4a, 0x20, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x27, 0x73, 0x20, 0x6e, 0x65,
    0x78, 0x74, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72,
    0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x64, 0x65,
    0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x34, 0x30, 0x39, 0x36, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x31, 0x02, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x31, 0x1e, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x31, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x08, 0x12, 0x03, 0x31, 0x2d, 0x5f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x04,
    0x08, 0xaf, 0x08, 0x05, 0x12, 0x03, 0x31, 0x2e, 0x5e, 0x0a, 0x8c, 0x01, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x05, 0x12, 0x03, 0x35, 0x02, 0x2c, 0x1a, 0x7f, 0x20, 0x49, 0x66, 0x20, 0x74, 0x72, 0x75,
    0x65, 0x2c, 0x20, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x20, 0x22, 0x6c, 0x69, 0x74,
    0x65, 0x72, 0x61, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x20, 0x6d, 0x6f, 0x64,
    0x65, 0x6c, 0x69, 0x6e, 0x67, 0x22, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x66, 0x65,
    0x61, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x66, 0x6c, 0x61,
    0x67, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x22, 0x64, 0x65, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67,
    0x2d, 0x73, 0x70, 0x65, 0x65, 0x64, 0x20, 0x76, 0x73, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x22, 0x20, 0x74, 0x72, 0x61,
    0x64, 0x65, 0x2d, 0x6f, 0x66, 0x66, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x35, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x35, 0x07, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x35,
    0x2a, 0x2b, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)