// @generated
/// Empty response for the StreamAccessLogs API. Will never be sent. See below.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAccessLogsResponse {
}
/// Stream message for the StreamAccessLogs API. Envoy will open a stream to the server and stream
/// access logs without ever expecting a response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAccessLogsMessage {
    /// Identifier data that will only be sent in the first message on the stream. This is effectively
    /// structured metadata and is a performance optimization.
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<stream_access_logs_message::Identifier>,
    /// Batches of log entries of a single type. Generally speaking, a given stream should only
    /// ever include one type of log entry.
    #[prost(oneof="stream_access_logs_message::LogEntries", tags="2, 3")]
    pub log_entries: ::core::option::Option<stream_access_logs_message::LogEntries>,
}
/// Nested message and enum types in `StreamAccessLogsMessage`.
pub mod stream_access_logs_message {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identifier {
        /// The node sending the access log messages over the stream.
        #[prost(message, optional, tag="1")]
        pub node: ::core::option::Option<super::super::super::super::api::v2::core::Node>,
        /// The friendly name of the log configured in :ref:`CommonGrpcAccessLogConfig
        /// <envoy_api_msg_config.accesslog.v2.CommonGrpcAccessLogConfig>`.
        #[prost(string, tag="2")]
        pub log_name: ::prost::alloc::string::String,
    }
    /// Wrapper for batches of HTTP access log entries.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HttpAccessLogEntries {
        #[prost(message, repeated, tag="1")]
        pub log_entry: ::prost::alloc::vec::Vec<super::super::super::super::data::accesslog::v2::HttpAccessLogEntry>,
    }
    /// Wrapper for batches of TCP access log entries.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TcpAccessLogEntries {
        #[prost(message, repeated, tag="1")]
        pub log_entry: ::prost::alloc::vec::Vec<super::super::super::super::data::accesslog::v2::TcpAccessLogEntry>,
    }
    /// Batches of log entries of a single type. Generally speaking, a given stream should only
    /// ever include one type of log entry.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LogEntries {
        #[prost(message, tag="2")]
        HttpLogs(HttpAccessLogEntries),
        #[prost(message, tag="3")]
        TcpLogs(TcpAccessLogEntries),
    }
}
/// Encoded file descriptor set for the `envoy.service.accesslog.v2` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xfb, 0x1c, 0x0a, 0x24, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2f, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x6c, 0x6f, 0x67, 0x2f, 0x76, 0x32, 0x2f,
    0x61, 0x6c, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1a, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x6c,
    0x6f, 0x67, 0x2e, 0x76, 0x32, 0x1a, 0x1c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x61, 0x70, 0x69,
    0x2f, 0x76, 0x32, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x27, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x2f,
    0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x6c, 0x6f, 0x67, 0x2f, 0x76, 0x32, 0x2f, 0x61, 0x63, 0x63,
    0x65, 0x73, 0x73, 0x6c, 0x6f, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64,
    0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1a, 0x0a, 0x18, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x22, 0x9b, 0x05, 0x0a, 0x17, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x4c, 0x6f, 0x67, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x5e, 0x0a, 0x0a,
    0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x3e, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x2e, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x6c, 0x6f, 0x67, 0x2e, 0x76, 0x32, 0x2e, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67, 0x73, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72,
    0x52, 0x0a, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x67, 0x0a, 0x09,
    0x68, 0x74, 0x74, 0x70, 0x5f, 0x6c, 0x6f, 0x67, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x48, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x6c, 0x6f, 0x67, 0x2e, 0x76, 0x32, 0x2e, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67, 0x73, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x2e, 0x48, 0x54, 0x54, 0x50, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x4c,
    0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x48, 0x00, 0x52, 0x08, 0x68, 0x74, 0x74,
    0x70, 0x4c, 0x6f, 0x67, 0x73, 0x12, 0x64, 0x0a, 0x08, 0x74, 0x63, 0x70, 0x5f, 0x6c, 0x6f, 0x67,
    0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x47, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x6c, 0x6f,
    0x67, 0x2e, 0x76, 0x32, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x4c, 0x6f, 0x67, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x54, 0x43, 0x50,
    0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73,
    0x48, 0x00, 0x52, 0x07, 0x74, 0x63, 0x70, 0x4c, 0x6f, 0x67, 0x73, 0x1a, 0x67, 0x0a, 0x0a, 0x49,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x35, 0x0a, 0x04, 0x6e, 0x6f, 0x64,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x4e, 0x6f, 0x64, 0x65,
    0x42, 0x08, 0xfa, 0x42, 0x05, 0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x04, 0x6e, 0x6f, 0x64, 0x65,
    0x12, 0x22, 0x0a, 0x08, 0x6c, 0x6f, 0x67, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x20, 0x01, 0x52, 0x07, 0x6c, 0x6f, 0x67,
    0x4e, 0x61, 0x6d, 0x65, 0x1a, 0x6a, 0x0a, 0x14, 0x48, 0x54, 0x54, 0x50, 0x41, 0x63, 0x63, 0x65,
    0x73, 0x73, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x12, 0x52, 0x0a, 0x09,
    0x6c, 0x6f, 0x67, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x2b, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x61, 0x63, 0x63,
    0x65, 0x73, 0x73, 0x6c, 0x6f, 0x67, 0x2e, 0x76, 0x32, 0x2e, 0x48, 0x54, 0x54, 0x50, 0x41, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x42, 0x08, 0xfa, 0x42,
    0x05, 0x92, 0x01, 0x02, 0x08, 0x01, 0x52, 0x08, 0x6c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79,
    0x1a, 0x68, 0x0a, 0x13, 0x54, 0x43, 0x50, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67,
    0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x12, 0x51, 0x0a, 0x09, 0x6c, 0x6f, 0x67, 0x5f, 0x65,
    0x6e, 0x74, 0x72, 0x79, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x6c, 0x6f,
    0x67, 0x2e, 0x76, 0x32, 0x2e, 0x54, 0x43, 0x50, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f,
    0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x92, 0x01, 0x02, 0x08, 0x01,
    0x52, 0x08, 0x6c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x42, 0x12, 0x0a, 0x0b, 0x6c, 0x6f,
    0x67, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x12, 0x03, 0xf8, 0x42, 0x01, 0x32, 0x96,
    0x01, 0x0a, 0x10, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67, 0x53, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x12, 0x81, 0x01, 0x0a, 0x10, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67, 0x73, 0x12, 0x33, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x6c,
    0x6f, 0x67, 0x2e, 0x76, 0x32, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x63, 0x63, 0x65,
    0x73, 0x73, 0x4c, 0x6f, 0x67, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x1a, 0x34, 0x2e,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x61, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x6c, 0x6f, 0x67, 0x2e, 0x76, 0x32, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x22, 0x00, 0x28, 0x01, 0x42, 0x8d, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02,
    0x10, 0x01, 0x0a, 0x28, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78,
    0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x6c, 0x6f, 0x67, 0x2e, 0x76, 0x32, 0x42, 0x08, 0x41, 0x6c,
    0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x4d, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
    0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f,
    0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65,
    0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2f, 0x61,
    0x63, 0x63, 0x65, 0x73, 0x73, 0x6c, 0x6f, 0x67, 0x2f, 0x76, 0x32, 0x3b, 0x61, 0x63, 0x63, 0x65,
    0x73, 0x73, 0x6c, 0x6f, 0x67, 0x76, 0x32, 0x4a, 0xcc, 0x13, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x47, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00,
    0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x31, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08,
    0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x41, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x0a, 0x00, 0x41, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00,
    0x29, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x29, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0c, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0c, 0x00,
    0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x64, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x0b, 0x12, 0x03, 0x0d, 0x00, 0x64, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x46,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0e, 0x00, 0x46, 0x0a,
    0x87, 0x01, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x13, 0x00, 0x1c, 0x01, 0x1a, 0x47, 0x20, 0x53,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x6c, 0x6f, 0x67, 0x73,
    0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x61,
    0x6e, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x2e, 0x0a, 0x32, 0x32, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64,
    0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x41,
    0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x20, 0x28, 0x41, 0x4c, 0x53, 0x29, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01,
    0x12, 0x03, 0x13, 0x08, 0x18, 0x0a, 0x86, 0x04, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04,
    0x1a, 0x02, 0x1b, 0x03, 0x1a, 0xf7, 0x03, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77, 0x69,
    0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73,
    0x65, 0x6e, 0x64, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73,
    0x4c, 0x6f, 0x67, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x65, 0x76, 0x65, 0x72, 0x2e, 0x20, 0x49, 0x74,
    0x20, 0x64, 0x6f, 0x65, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74,
    0x20, 0x61, 0x6e, 0x79, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x73, 0x20, 0x6e, 0x6f, 0x74,
    0x68, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x64, 0x6f,
    0x6e, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x73, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20, 0x64, 0x69,
    0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x20, 0x69, 0x66, 0x20, 0x69, 0x74, 0x20, 0x65,
    0x78, 0x70, 0x65, 0x63, 0x74, 0x73, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x74, 0x6f, 0x20,
    0x72, 0x65, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x2e, 0x20, 0x49, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x20, 0x77, 0x65, 0x20, 0x6d, 0x61, 0x79, 0x20,
    0x64, 0x65, 0x63, 0x69, 0x64, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x64, 0x64, 0x20, 0x61, 0x20,
    0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x41, 0x50, 0x49, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x22, 0x63, 0x72, 0x69, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x22, 0x20, 0x61, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x20, 0x6c, 0x6f, 0x67, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x77, 0x68, 0x69,
    0x63, 0x68, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x75,
    0x66, 0x66, 0x65, 0x72, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x6c, 0x6f, 0x67, 0x73,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x0a, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x20,
    0x69, 0x74, 0x20, 0x67, 0x65, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x41, 0x43, 0x4b, 0x20, 0x73,
    0x6f, 0x20, 0x69, 0x74, 0x20, 0x63, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20,
    0x72, 0x65, 0x74, 0x72, 0x79, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x41, 0x50, 0x49, 0x20,
    0x69, 0x73, 0x20, 0x64, 0x65, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x68, 0x69, 0x67, 0x68, 0x20, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x70, 0x75, 0x74, 0x20,
    0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x74, 0x20, 0x6d, 0x69,
    0x67, 0x68, 0x74, 0x20, 0x62, 0x65, 0x20, 0x6c, 0x6f, 0x73, 0x73, 0x79, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x06, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x1a, 0x1e, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1a, 0x40, 0x58, 0x0a, 0x59, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1f, 0x00,
    0x20, 0x01, 0x1a, 0x4d, 0x20, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67, 0x73, 0x20, 0x41, 0x50, 0x49,
    0x2e, 0x20, 0x57, 0x69, 0x6c, 0x6c, 0x20, 0x6e, 0x65, 0x76, 0x65, 0x72, 0x20, 0x62, 0x65, 0x20,
    0x73, 0x65, 0x6e, 0x74, 0x2e, 0x20, 0x53, 0x65, 0x65, 0x20, 0x62, 0x65, 0x6c, 0x6f, 0x77, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x20, 0x0a, 0x9d, 0x01,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x24, 0x00, 0x47, 0x01, 0x1a, 0x90, 0x01, 0x20, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x4c, 0x6f, 0x67, 0x73, 0x20, 0x41, 0x50, 0x49, 0x2e, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6f, 0x70, 0x65, 0x6e, 0x20, 0x61, 0x20, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x0a, 0x20, 0x61, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x20, 0x6c, 0x6f, 0x67, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75,
    0x74, 0x20, 0x65, 0x76, 0x65, 0x72, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x69, 0x6e, 0x67,
    0x20, 0x61, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x24, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03,
    0x00, 0x12, 0x04, 0x25, 0x02, 0x2c, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01,
    0x12, 0x03, 0x25, 0x0a, 0x14, 0x0a, 0x4a, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x27, 0x04, 0x4c, 0x1a, 0x3b, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20,
    0x73, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x63, 0x63, 0x65,
    0x73, 0x73, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20,
    0x6f, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e,
    0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x27, 0x04,
    0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x15,
    0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x27, 0x1c,
    0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x27, 0x1e,
    0x4b, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x11, 0x12,
    0x03, 0x27, 0x1f, 0x4a, 0x0a, 0x9d, 0x01, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x2b, 0x04, 0x43, 0x1a, 0x8d, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x72, 0x69, 0x65,
    0x6e, 0x64, 0x6c, 0x79, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6c, 0x6f, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x64, 0x20,
    0x69, 0x6e, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x47,
    0x72, 0x70, 0x63, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x0a, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x6d,
    0x73, 0x67, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73,
    0x6c, 0x6f, 0x67, 0x2e, 0x76, 0x32, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x47, 0x72, 0x70,
    0x63, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x4c, 0x6f, 0x67, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x3e, 0x60, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x2b, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x2b, 0x0b, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x2b, 0x16, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x08, 0x12,
    0x03, 0x2b, 0x18, 0x42, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x08, 0xaf,
    0x08, 0x0e, 0x12, 0x03, 0x2b, 0x19, 0x41, 0x0a, 0x3f, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x01, 0x12,
    0x04, 0x2f, 0x02, 0x32, 0x03, 0x1a, 0x31, 0x20, 0x57, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x48,
    0x54, 0x54, 0x50, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x65,
    0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x01,
    0x01, 0x12, 0x03, 0x2f, 0x0a, 0x1e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00,
    0x12, 0x04, 0x30, 0x04, 0x31, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x30, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x30, 0x0d, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x30, 0x32, 0x3b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x30, 0x3e, 0x3f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00,
    0x08, 0x12, 0x03, 0x31, 0x08, 0x34, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00,
    0x08, 0xaf, 0x08, 0x12, 0x12, 0x03, 0x31, 0x09, 0x33, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x01, 0x03,
    0x02, 0x12, 0x04, 0x35, 0x02, 0x38, 0x03, 0x1a, 0x30, 0x20, 0x57, 0x72, 0x61, 0x70, 0x70, 0x65,
    0x72, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x54, 0x43, 0x50, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x6c, 0x6f, 0x67, 0x20,
    0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03,
    0x02, 0x01, 0x12, 0x03, 0x35, 0x0a, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x02, 0x02,
    0x00, 0x12, 0x04, 0x36, 0x04, 0x37, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x02, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x36, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x02, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x36, 0x0d, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x36, 0x31, 0x3a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x02, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x36, 0x3d, 0x3e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x02, 0x02,
    0x00, 0x08, 0x12, 0x03, 0x37, 0x08, 0x34, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x02, 0x02,
    0x00, 0x08, 0xaf, 0x08, 0x12, 0x12, 0x03, 0x37, 0x09, 0x33, 0x0a, 0xa6, 0x01, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x3c, 0x02, 0x1c, 0x1a, 0x98, 0x01, 0x20, 0x49, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65,
    0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x65,
    0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x6c, 0x79, 0x0a, 0x20, 0x73, 0x74, 0x72, 0x75,
    0x63, 0x74, 0x75, 0x72, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d,
    0x61, 0x6e, 0x63, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6d, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3c, 0x02,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x0d, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3c, 0x1a, 0x1b, 0x0a, 0x8c, 0x01,
    0x0a, 0x04, 0x04, 0x01, 0x08, 0x00, 0x12, 0x04, 0x40, 0x02, 0x46, 0x03, 0x1a, 0x7e, 0x20, 0x42,
    0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x65, 0x6e,
    0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c,
    0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x20, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x6c, 0x6c,
    0x79, 0x20, 0x73, 0x70, 0x65, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x2c, 0x20, 0x61, 0x20, 0x67, 0x69,
    0x76, 0x65, 0x6e, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x0a, 0x20, 0x65, 0x76, 0x65, 0x72, 0x20, 0x69, 0x6e, 0x63,
    0x6c, 0x75, 0x64, 0x65, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x6c, 0x6f, 0x67, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x08, 0x00, 0x01, 0x12, 0x03, 0x40, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x08, 0x00, 0x02, 0x12, 0x03, 0x41, 0x04, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x08, 0x00,
    0x02, 0xaf, 0x08, 0x12, 0x03, 0x41, 0x04, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x43, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x43, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x43, 0x19,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x43, 0x25, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x45, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x45, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x45, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x45, 0x23, 0x24, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.service.accesslog.v2.serde.rs");
include!("envoy.service.accesslog.v2.tonic.rs");
// @@protoc_insertion_point(module)