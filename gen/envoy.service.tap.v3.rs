// @generated
/// \[#not-implemented-hide:\] Stream message for the Tap API. Envoy will open a stream to the server
/// and stream taps without ever expecting a response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamTapsRequest {
    /// Identifier data effectively is a structured metadata. As a performance optimization this will
    /// only be sent in the first message on the stream.
    #[prost(message, optional, tag="1")]
    pub identifier: ::core::option::Option<stream_taps_request::Identifier>,
    /// The trace id. this can be used to merge together a streaming trace. Note that the trace_id
    /// is not guaranteed to be spatially or temporally unique.
    #[prost(uint64, tag="2")]
    pub trace_id: u64,
    /// The trace data.
    #[prost(message, optional, tag="3")]
    pub trace: ::core::option::Option<super::super::super::data::tap::v3::TraceWrapper>,
}
/// Nested message and enum types in `StreamTapsRequest`.
pub mod stream_taps_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identifier {
        /// The node sending taps over the stream.
        #[prost(message, optional, tag="1")]
        pub node: ::core::option::Option<super::super::super::super::config::core::v3::Node>,
        /// The opaque identifier that was set in the :ref:`output config
        /// <envoy_v3_api_field_config.tap.v3.StreamingGrpcSink.tap_id>`.
        #[prost(string, tag="2")]
        pub tap_id: ::prost::alloc::string::String,
    }
}
/// \[#not-implemented-hide:\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamTapsResponse {
}
/// Encoded file descriptor set for the `envoy.service.tap.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf0, 0x15, 0x0a, 0x1e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2f, 0x74, 0x61, 0x70, 0x2f, 0x76, 0x33, 0x2f, 0x74, 0x61, 0x70, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x14, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2e, 0x74, 0x61, 0x70, 0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x33, 0x2f,
    0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x2f, 0x74, 0x61, 0x70, 0x2f, 0x76, 0x33, 0x2f, 0x77, 0x72,
    0x61, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70,
    0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x21, 0x75, 0x64, 0x70, 0x61,
    0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x8c, 0x03, 0x0a, 0x11, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x54, 0x61, 0x70, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x52, 0x0a, 0x0a,
    0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x32, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x2e, 0x74, 0x61, 0x70, 0x2e, 0x76, 0x33, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x61,
    0x70, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69,
    0x66, 0x69, 0x65, 0x72, 0x52, 0x0a, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72,
    0x12, 0x19, 0x0a, 0x08, 0x74, 0x72, 0x61, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x07, 0x74, 0x72, 0x61, 0x63, 0x65, 0x49, 0x64, 0x12, 0x35, 0x0a, 0x05, 0x74,
    0x72, 0x61, 0x63, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x74, 0x61, 0x70, 0x2e, 0x76, 0x33, 0x2e, 0x54,
    0x72, 0x61, 0x63, 0x65, 0x57, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x52, 0x05, 0x74, 0x72, 0x61,
    0x63, 0x65, 0x1a, 0x9c, 0x01, 0x0a, 0x0a, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x12, 0x38, 0x0a, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63,
    0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x42, 0x08, 0xfa, 0x42, 0x05,
    0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x12, 0x15, 0x0a, 0x06, 0x74,
    0x61, 0x70, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x61, 0x70,
    0x49, 0x64, 0x3a, 0x3d, 0x9a, 0xc5, 0x88, 0x1e, 0x38, 0x0a, 0x36, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x74, 0x61, 0x70, 0x2e, 0x76, 0x32, 0x61,
    0x6c, 0x70, 0x68, 0x61, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x61, 0x70, 0x73, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x3a, 0x32, 0x9a, 0xc5, 0x88, 0x1e, 0x2d, 0x0a, 0x2b, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x74, 0x61, 0x70, 0x2e, 0x76, 0x32, 0x61, 0x6c,
    0x70, 0x68, 0x61, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x61, 0x70, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x49, 0x0a, 0x12, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54,
    0x61, 0x70, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x3a, 0x33, 0x9a, 0xc5, 0x88,
    0x1e, 0x2e, 0x0a, 0x2c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x2e, 0x74, 0x61, 0x70, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x54, 0x61, 0x70, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x32, 0x75, 0x0a, 0x0e, 0x54, 0x61, 0x70, 0x53, 0x69, 0x6e, 0x6b, 0x53, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x12, 0x63, 0x0a, 0x0a, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x61, 0x70, 0x73,
    0x12, 0x27, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x2e, 0x74, 0x61, 0x70, 0x2e, 0x76, 0x33, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x61,
    0x70, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x28, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x74, 0x61, 0x70, 0x2e, 0x76, 0x33,
    0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x61, 0x70, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x22, 0x00, 0x28, 0x01, 0x42, 0x7b, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10,
    0x02, 0x0a, 0x22, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x74,
    0x61, 0x70, 0x2e, 0x76, 0x33, 0x42, 0x08, 0x54, 0x61, 0x70, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50,
    0x01, 0x5a, 0x41, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2f, 0x74, 0x61, 0x70, 0x2f, 0x76, 0x33, 0x3b, 0x74,
    0x61, 0x70, 0x76, 0x33, 0x4a, 0xc4, 0x0e, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x3f, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x02, 0x00, 0x1d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x07, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08, 0x00, 0x2b, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x09, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0b, 0x00, 0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0b, 0x00, 0x3b, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12,
    0x03, 0x0c, 0x00, 0x29, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x22, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0d, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0e, 0x00, 0x58, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0e, 0x00, 0x58, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99,
    0x6a, 0x02, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0xb9, 0x01, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04,
    0x15, 0x00, 0x1b, 0x01, 0x1a, 0x85, 0x01, 0x20, 0x5b, 0x23, 0x6e, 0x6f, 0x74, 0x2d, 0x69, 0x6d,
    0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x2d, 0x68, 0x69, 0x64, 0x65, 0x3a, 0x5d,
    0x20, 0x41, 0x20, 0x74, 0x61, 0x70, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x20, 0x69, 0x6e, 0x63, 0x6f, 0x6d, 0x69,
    0x6e, 0x67, 0x20, 0x74, 0x61, 0x70, 0x73, 0x2e, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x0a, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x54, 0x61, 0x70, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x20,
    0x63, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x64, 0x20, 0x74, 0x61, 0x70, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x0a, 0x32, 0x25, 0x20, 0x5b,
    0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a,
    0x20, 0x54, 0x61, 0x70, 0x20, 0x73, 0x69, 0x6e, 0x6b, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x15, 0x08, 0x16, 0x0a,
    0xed, 0x01, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x19, 0x02, 0x1a, 0x03, 0x1a, 0xde,
    0x01, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x54, 0x61, 0x70, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x65, 0x76, 0x65, 0x72,
    0x2e, 0x20, 0x49, 0x74, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x65, 0x78,
    0x70, 0x65, 0x63, 0x74, 0x20, 0x61, 0x6e, 0x79, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x73,
    0x20, 0x6e, 0x6f, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62,
    0x65, 0x20, 0x64, 0x6f, 0x6e, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61,
    0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x2e, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64,
    0x0a, 0x20, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x20, 0x69, 0x66, 0x20,
    0x69, 0x74, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x73, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79,
    0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x06, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x19, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x19, 0x18, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x19, 0x34, 0x46, 0x0a, 0xa2, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x1f, 0x00, 0x39, 0x01, 0x1a, 0x95, 0x01, 0x20, 0x5b, 0x23, 0x6e, 0x6f, 0x74, 0x2d, 0x69, 0x6d,
    0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x2d, 0x68, 0x69, 0x64, 0x65, 0x3a, 0x5d,
    0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x54, 0x61, 0x70, 0x20, 0x41, 0x50, 0x49, 0x2e,
    0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6f, 0x70, 0x65, 0x6e,
    0x20, 0x61, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x0a, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x20, 0x74, 0x61, 0x70, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74,
    0x20, 0x65, 0x76, 0x65, 0x72, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x69, 0x6e, 0x67, 0x20,
    0x61, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12,
    0x04, 0x20, 0x02, 0x21, 0x34, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03,
    0x01, 0x12, 0x04, 0x20, 0x02, 0x21, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12,
    0x04, 0x23, 0x02, 0x2d, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03,
    0x23, 0x0a, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x07, 0x12, 0x04, 0x24, 0x04,
    0x25, 0x41, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x00, 0x03, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01,
    0x12, 0x04, 0x24, 0x04, 0x25, 0x41, 0x0a, 0x37, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x28, 0x04, 0x4f, 0x1a, 0x28, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x64, 0x65,
    0x20, 0x73, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x61, 0x70, 0x73, 0x20, 0x6f, 0x76,
    0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x28, 0x04, 0x17, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x18, 0x1c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x1f, 0x20, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x28, 0x21, 0x4e, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x11, 0x12, 0x03, 0x28,
    0x22, 0x4d, 0x0a, 0x8d, 0x01, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2c,
    0x04, 0x16, 0x1a, 0x7e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x61, 0x71, 0x75, 0x65, 0x20,
    0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x77, 0x61, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x3a,
    0x72, 0x65, 0x66, 0x3a, 0x60, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x0a, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70,
    0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x74,
    0x61, 0x70, 0x2e, 0x76, 0x33, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x47,
    0x72, 0x70, 0x63, 0x53, 0x69, 0x6e, 0x6b, 0x2e, 0x74, 0x61, 0x70, 0x5f, 0x69, 0x64, 0x3e, 0x60,
    0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2c,
    0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c,
    0x0b, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2c,
    0x14, 0x15, 0x0a, 0x9f, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x31, 0x02, 0x1c,
    0x1a, 0x91, 0x01, 0x20, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x20, 0x64,
    0x61, 0x74, 0x61, 0x20, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x6c, 0x79, 0x20,
    0x69, 0x73, 0x20, 0x61, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x64, 0x20,
    0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x20, 0x41, 0x73, 0x20, 0x61, 0x20, 0x70,
    0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6d,
    0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x0a, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x31,
    0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x0d, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x1a, 0x1b, 0x0a, 0xa3,
    0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x35, 0x02, 0x16, 0x1a, 0x95, 0x01, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x63, 0x65, 0x20, 0x69, 0x64, 0x2e, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x6d, 0x65, 0x72, 0x67, 0x65, 0x20, 0x74, 0x6f, 0x67, 0x65, 0x74, 0x68, 0x65, 0x72,
    0x20, 0x61, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x72, 0x61,
    0x63, 0x65, 0x2e, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x74, 0x72, 0x61, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x0a, 0x20, 0x69, 0x73, 0x20, 0x6e,
    0x6f, 0x74, 0x20, 0x67, 0x75, 0x61, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x62, 0x65, 0x20, 0x73, 0x70, 0x61, 0x74, 0x69, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x6f, 0x72,
    0x20, 0x74, 0x65, 0x6d, 0x70, 0x6f, 0x72, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x75, 0x6e, 0x69, 0x71,
    0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x35,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x35, 0x09, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x35, 0x14, 0x15, 0x0a, 0x1e,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x38, 0x02, 0x25, 0x1a, 0x11, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x74, 0x72, 0x61, 0x63, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x38, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x38, 0x1b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x38, 0x23, 0x24, 0x0a, 0x26, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x3c, 0x00, 0x3f, 0x01, 0x1a, 0x1a, 0x20, 0x5b, 0x23, 0x6e, 0x6f, 0x74, 0x2d, 0x69, 0x6d, 0x70,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x2d, 0x68, 0x69, 0x64, 0x65, 0x3a, 0x5d, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x01, 0x07, 0x12, 0x04, 0x3d, 0x02, 0x3e, 0x35, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x01, 0x07,
    0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x3d, 0x02, 0x3e, 0x35, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
include!("envoy.service.tap.v3.tonic.rs");
// @@protoc_insertion_point(module)