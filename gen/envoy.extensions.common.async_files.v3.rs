// @generated
// [#protodoc-title: AsyncFileManager configuration]

/// Configuration to instantiate or select a singleton ``AsyncFileManager``.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncFileManagerConfig {
    /// An optional identifier for the manager. An empty string is a valid identifier
    /// for a common, default ``AsyncFileManager``.
    ///
    /// Reusing the same id with different configurations in the same envoy instance
    /// is an error.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(oneof="async_file_manager_config::ManagerType", tags="2")]
    pub manager_type: ::core::option::Option<async_file_manager_config::ManagerType>,
}
/// Nested message and enum types in `AsyncFileManagerConfig`.
pub mod async_file_manager_config {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThreadPool {
        /// The number of threads to use. If unset or zero, will default to the number
        /// of concurrent threads the hardware supports. This default is subject to
        /// change if performance analysis suggests it.
        #[prost(uint32, tag="1")]
        pub thread_count: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ManagerType {
        /// Configuration for a thread-pool based async file manager.
        #[prost(message, tag="2")]
        ThreadPool(ThreadPool),
    }
}
/// Encoded file descriptor set for the `envoy.extensions.common.async_files.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd8, 0x0d, 0x0a, 0x3f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x61, 0x73, 0x79,
    0x6e, 0x63, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x76, 0x33, 0x2f, 0x61, 0x73, 0x79, 0x6e,
    0x63, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x26, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x61, 0x73,
    0x79, 0x6e, 0x63, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x78, 0x64,
    0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x33,
    0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75,
    0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xe6, 0x01, 0x0a, 0x16, 0x41, 0x73, 0x79, 0x6e, 0x63, 0x46,
    0x69, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64,
    0x12, 0x6c, 0x0a, 0x0b, 0x74, 0x68, 0x72, 0x65, 0x61, 0x64, 0x5f, 0x70, 0x6f, 0x6f, 0x6c, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x49, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
    0x61, 0x73, 0x79, 0x6e, 0x63, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2e, 0x76, 0x33, 0x2e, 0x41,
    0x73, 0x79, 0x6e, 0x63, 0x46, 0x69, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x54, 0x68, 0x72, 0x65, 0x61, 0x64, 0x50, 0x6f, 0x6f, 0x6c,
    0x48, 0x00, 0x52, 0x0a, 0x74, 0x68, 0x72, 0x65, 0x61, 0x64, 0x50, 0x6f, 0x6f, 0x6c, 0x1a, 0x39,
    0x0a, 0x0a, 0x54, 0x68, 0x72, 0x65, 0x61, 0x64, 0x50, 0x6f, 0x6f, 0x6c, 0x12, 0x2b, 0x0a, 0x0c,
    0x74, 0x68, 0x72, 0x65, 0x61, 0x64, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0d, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x2a, 0x03, 0x18, 0x80, 0x08, 0x52, 0x0b, 0x74, 0x68,
    0x72, 0x65, 0x61, 0x64, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x42, 0x13, 0x0a, 0x0c, 0x6d, 0x61, 0x6e,
    0x61, 0x67, 0x65, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x12, 0x03, 0xf8, 0x42, 0x01, 0x42, 0xbc,
    0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0xd2, 0xc6, 0xa4, 0xe1, 0x06, 0x02, 0x08,
    0x01, 0x0a, 0x34, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x61, 0x73, 0x79, 0x6e, 0x63, 0x5f, 0x66,
    0x69, 0x6c, 0x65, 0x73, 0x2e, 0x76, 0x33, 0x42, 0x15, 0x41, 0x73, 0x79, 0x6e, 0x63, 0x46, 0x69,
    0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
    0x5a, 0x5b, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e,
    0x2f, 0x61, 0x73, 0x79, 0x6e, 0x63, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x76, 0x33, 0x3b,
    0x61, 0x73, 0x79, 0x6e, 0x63, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x76, 0x33, 0x4a, 0xe3, 0x08,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x28, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x2f, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06,
    0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x21, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x4d, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09,
    0x00, 0x4d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x36, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x36, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00,
    0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0c, 0x00, 0x72, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0c, 0x00,
    0x72, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0e, 0x00, 0x40, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xea, 0xc8, 0x94, 0x6c, 0x01, 0x12, 0x03,
    0x0e, 0x00, 0x40, 0x0a, 0x8b, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x13, 0x00, 0x28, 0x01,
    0x1a, 0x4a, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x69, 0x61, 0x74, 0x65, 0x20,
    0x6f, 0x72, 0x20, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67,
    0x6c, 0x65, 0x74, 0x6f, 0x6e, 0x20, 0x60, 0x60, 0x41, 0x73, 0x79, 0x6e, 0x63, 0x46, 0x69, 0x6c,
    0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x60, 0x60, 0x2e, 0x0a, 0x32, 0x33, 0x20, 0x5b,
    0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a,
    0x20, 0x41, 0x73, 0x79, 0x6e, 0x63, 0x46, 0x69, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65,
    0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5d,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x13, 0x08, 0x1e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x14, 0x02, 0x19, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x14, 0x0a, 0x14, 0x0a, 0xd2, 0x01, 0x0a, 0x06, 0x04, 0x00,
    0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x18, 0x04, 0x44, 0x1a, 0xc2, 0x01, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x72, 0x65, 0x61,
    0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x75, 0x6e,
    0x73, 0x65, 0x74, 0x20, 0x6f, 0x72, 0x20, 0x7a, 0x65, 0x72, 0x6f, 0x2c, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x0a, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6e, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x72, 0x65, 0x61, 0x64, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x68, 0x61, 0x72, 0x64, 0x77, 0x61, 0x72, 0x65, 0x20, 0x73, 0x75, 0x70, 0x70,
    0x6f, 0x72, 0x74, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75,
    0x6c, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x74, 0x6f,
    0x0a, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x69, 0x66, 0x20, 0x70, 0x65, 0x72, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x61, 0x6e, 0x61, 0x6c, 0x79, 0x73, 0x69, 0x73,
    0x20, 0x73, 0x75, 0x67, 0x67, 0x65, 0x73, 0x74, 0x73, 0x20, 0x69, 0x74, 0x2e, 0x0a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x04, 0x0a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x0b, 0x17, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x1a, 0x1b, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x18, 0x1c, 0x43, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x05, 0x12, 0x03, 0x18, 0x1d,
    0x42, 0x0a, 0xe7, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x20, 0x02, 0x10, 0x1a,
    0xd9, 0x01, 0x20, 0x41, 0x6e, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x69,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x20, 0x41, 0x6e, 0x20, 0x65, 0x6d,
    0x70, 0x74, 0x79, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72,
    0x0a, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2c, 0x20,
    0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x60, 0x60, 0x41, 0x73, 0x79, 0x6e, 0x63, 0x46,
    0x69, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x60, 0x60, 0x2e, 0x0a, 0x0a, 0x20,
    0x52, 0x65, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65,
    0x20, 0x69, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65,
    0x6e, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x0a, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x20, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x20, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x22,
    0x02, 0x27, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x22, 0x08,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x02, 0x12, 0x03, 0x23, 0x04, 0x26, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x08, 0x00, 0x02, 0xaf, 0x08, 0x12, 0x03, 0x23, 0x04, 0x26, 0x0a,
    0x48, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x26, 0x04, 0x1f, 0x1a, 0x3b, 0x20, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x61, 0x20, 0x74, 0x68, 0x72, 0x65, 0x61, 0x64, 0x2d, 0x70, 0x6f, 0x6f, 0x6c, 0x20, 0x62,
    0x61, 0x73, 0x65, 0x64, 0x20, 0x61, 0x73, 0x79, 0x6e, 0x63, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20,
    0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x26, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x26, 0x0f, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x26, 0x1d, 0x1e, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.common.async_files.v3.serde.rs");
// @@protoc_insertion_point(module)