// @generated
// [#protodoc-title: Squash]
// Squash :ref:`configuration overview <config_http_filters_squash>`.
// [#extension: envoy.filters.http.squash]

/// [#next-free-field: 6]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Squash {
    /// The name of the cluster that hosts the Squash server.
    #[prost(string, tag="1")]
    pub cluster: ::prost::alloc::string::String,
    /// When the filter requests the Squash server to create a DebugAttachment, it will use this
    /// structure as template for the body of the request. It can contain reference to environment
    /// variables in the form of '{{ ENV_VAR_NAME }}'. These can be used to provide the Squash server
    /// with more information to find the process to attach the debugger to. For example, in a
    /// Istio/k8s environment, this will contain information on the pod:
    ///
    /// .. code-block:: json
    ///
    ///   {
    ///     "spec": {
    ///       "attachment": {
    ///         "pod": "{{ POD_NAME }}",
    ///         "namespace": "{{ POD_NAMESPACE }}"
    ///       },
    ///       "match_request": true
    ///     }
    ///   }
    ///
    /// (where POD_NAME, POD_NAMESPACE are configured in the pod via the Downward API)
    #[prost(message, optional, tag="2")]
    pub attachment_template: ::core::option::Option<::pbjson_types::Struct>,
    /// The timeout for individual requests sent to the Squash cluster. Defaults to 1 second.
    #[prost(message, optional, tag="3")]
    pub request_timeout: ::core::option::Option<::pbjson_types::Duration>,
    /// The total timeout Squash will delay a request and wait for it to be attached. Defaults to 60
    /// seconds.
    #[prost(message, optional, tag="4")]
    pub attachment_timeout: ::core::option::Option<::pbjson_types::Duration>,
    /// Amount of time to poll for the status of the attachment object in the Squash server
    /// (to check if has been attached). Defaults to 1 second.
    #[prost(message, optional, tag="5")]
    pub attachment_poll_period: ::core::option::Option<::pbjson_types::Duration>,
}
/// Encoded file descriptor set for the `envoy.extensions.filters.http.squash.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd8, 0x14, 0x0a, 0x3c, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x73, 0x71, 0x75, 0x61, 0x73,
    0x68, 0x2f, 0x76, 0x33, 0x2f, 0x73, 0x71, 0x75, 0x61, 0x73, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x27, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70,
    0x2e, 0x73, 0x71, 0x75, 0x61, 0x73, 0x68, 0x2e, 0x76, 0x33, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x73, 0x74, 0x72, 0x75,
    0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x21, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e,
    0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x86, 0x03, 0x0a, 0x06, 0x53, 0x71, 0x75, 0x61, 0x73, 0x68, 0x12, 0x21,
    0x0a, 0x07, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42,
    0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x10, 0x01, 0x52, 0x07, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x12, 0x48, 0x0a, 0x13, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x5f,
    0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17,
    0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
    0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x12, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d,
    0x65, 0x6e, 0x74, 0x54, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x12, 0x42, 0x0a, 0x0f, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x0e, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x12,
    0x48, 0x0a, 0x12, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x69,
    0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x11, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65,
    0x6e, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x12, 0x4f, 0x0a, 0x16, 0x61, 0x74, 0x74,
    0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x70, 0x6f, 0x6c, 0x6c, 0x5f, 0x70, 0x65, 0x72,
    0x69, 0x6f, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x52, 0x14, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74,
    0x50, 0x6f, 0x6c, 0x6c, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x3a, 0x30, 0x9a, 0xc5, 0x88, 0x1e,
    0x2b, 0x0a, 0x29, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x73, 0x71, 0x75, 0x61,
    0x73, 0x68, 0x2e, 0x76, 0x32, 0x2e, 0x53, 0x71, 0x75, 0x61, 0x73, 0x68, 0x42, 0xa7, 0x01, 0xba,
    0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x35, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e,
    0x68, 0x74, 0x74, 0x70, 0x2e, 0x73, 0x71, 0x75, 0x61, 0x73, 0x68, 0x2e, 0x76, 0x33, 0x42, 0x0b,
    0x53, 0x71, 0x75, 0x61, 0x73, 0x68, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x57, 0x67,
    0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70,
    0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d,
    0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68,
    0x74, 0x74, 0x70, 0x2f, 0x73, 0x71, 0x75, 0x61, 0x73, 0x68, 0x2f, 0x76, 0x33, 0x3b, 0x73, 0x71,
    0x75, 0x61, 0x73, 0x68, 0x76, 0x33, 0x4a, 0x9a, 0x0e, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x3c,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x02, 0x00, 0x30, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x28,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x07, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08, 0x00,
    0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x09, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0b, 0x00, 0x4e, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0b, 0x00,
    0x4e, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x08, 0x12, 0x03, 0x0c, 0x00, 0x2c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x22,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0d, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x0e, 0x00, 0x6e, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0e, 0x00, 0x6e,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87,
    0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0xae, 0x01, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x16, 0x00, 0x3c, 0x01, 0x1a, 0x17, 0x20, 0x5b, 0x23, 0x6e, 0x65, 0x78, 0x74, 0x2d,
    0x66, 0x72, 0x65, 0x65, 0x2d, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x3a, 0x20, 0x36, 0x5d, 0x0a, 0x32,
    0x88, 0x01, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69,
    0x74, 0x6c, 0x65, 0x3a, 0x20, 0x53, 0x71, 0x75, 0x61, 0x73, 0x68, 0x5d, 0x0a, 0x20, 0x53, 0x71,
    0x75, 0x61, 0x73, 0x68, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65,
    0x77, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x73, 0x71, 0x75, 0x61, 0x73, 0x68, 0x3e, 0x60, 0x2e,
    0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74,
    0x70, 0x2e, 0x73, 0x71, 0x75, 0x61, 0x73, 0x68, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x16, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x17,
    0x02, 0x18, 0x32, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12,
    0x04, 0x17, 0x02, 0x18, 0x32, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1b,
    0x02, 0x3e, 0x1a, 0x37, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x71, 0x75, 0x61,
    0x73, 0x68, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1b, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x1b, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03,
    0x1b, 0x15, 0x3d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12,
    0x03, 0x1b, 0x16, 0x3c, 0x0a, 0xc9, 0x05, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x30,
    0x02, 0x31, 0x1a, 0xbb, 0x05, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x53, 0x71, 0x75, 0x61, 0x73, 0x68, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x20, 0x74, 0x6f, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x20, 0x61, 0x20, 0x44, 0x65, 0x62,
    0x75, 0x67, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x69, 0x74,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x0a, 0x20,
    0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x20, 0x61, 0x73, 0x20, 0x74, 0x65, 0x6d,
    0x70, 0x6c, 0x61, 0x74, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6f,
    0x64, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69,
    0x6e, 0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x65,
    0x6e, 0x76, 0x69, 0x72, 0x6f, 0x6e, 0x6d, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x76, 0x61, 0x72, 0x69,
    0x61, 0x62, 0x6c, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x72,
    0x6d, 0x20, 0x6f, 0x66, 0x20, 0x27, 0x7b, 0x7b, 0x20, 0x45, 0x4e, 0x56, 0x5f, 0x56, 0x41, 0x52,
    0x5f, 0x4e, 0x41, 0x4d, 0x45, 0x20, 0x7d, 0x7d, 0x27, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x73, 0x65,
    0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x71, 0x75, 0x61,
    0x73, 0x68, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x0a, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x6d, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x74, 0x6f, 0x20, 0x66, 0x69, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f,
    0x63, 0x65, 0x73, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x64, 0x65, 0x62, 0x75, 0x67, 0x67, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x2e, 0x20,
    0x46, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2c, 0x20, 0x69, 0x6e, 0x20,
    0x61, 0x0a, 0x20, 0x49, 0x73, 0x74, 0x69, 0x6f, 0x2f, 0x6b, 0x38, 0x73, 0x20, 0x65, 0x6e, 0x76,
    0x69, 0x72, 0x6f, 0x6e, 0x6d, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70,
    0x6f, 0x64, 0x3a, 0x0a, 0x0a, 0x20, 0x2e, 0x2e, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x2d, 0x62, 0x6c,
    0x6f, 0x63, 0x6b, 0x3a, 0x3a, 0x20, 0x6a, 0x73, 0x6f, 0x6e, 0x0a, 0x0a, 0x20, 0x20, 0x7b, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x22, 0x73, 0x70, 0x65, 0x63, 0x22, 0x3a, 0x20, 0x7b, 0x0a, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x22, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x22,
    0x3a, 0x20, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22, 0x70, 0x6f, 0x64,
    0x22, 0x3a, 0x20, 0x22, 0x7b, 0x7b, 0x20, 0x50, 0x4f, 0x44, 0x5f, 0x4e, 0x41, 0x4d, 0x45, 0x20,
    0x7d, 0x7d, 0x22, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22, 0x6e, 0x61,
    0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x22, 0x3a, 0x20, 0x22, 0x7b, 0x7b, 0x20, 0x50, 0x4f,
    0x44, 0x5f, 0x4e, 0x41, 0x4d, 0x45, 0x53, 0x50, 0x41, 0x43, 0x45, 0x20, 0x7d, 0x7d, 0x22, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x7d, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x3a, 0x20,
    0x74, 0x72, 0x75, 0x65, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x7d, 0x0a, 0x20, 0x20, 0x7d, 0x0a, 0x0a,
    0x20, 0x28, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x50, 0x4f, 0x44, 0x5f, 0x4e, 0x41, 0x4d, 0x45,
    0x2c, 0x20, 0x50, 0x4f, 0x44, 0x5f, 0x4e, 0x41, 0x4d, 0x45, 0x53, 0x50, 0x41, 0x43, 0x45, 0x20,
    0x61, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x64, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x6f, 0x64, 0x20, 0x76, 0x69, 0x61, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x44, 0x6f, 0x77, 0x6e, 0x77, 0x61, 0x72, 0x64, 0x20, 0x41, 0x50, 0x49, 0x29, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x30, 0x02, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x30, 0x19, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30, 0x2f, 0x30, 0x0a, 0x64, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x33, 0x02, 0x2f, 0x1a, 0x57, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x69,
    0x6d, 0x65, 0x6f, 0x75, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69,
    0x64, 0x75, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x73, 0x65,
    0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x71, 0x75, 0x61, 0x73, 0x68,
    0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x31, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x33, 0x02, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x33, 0x1b, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x33, 0x2d, 0x2e, 0x0a, 0x75, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x37, 0x02, 0x32, 0x1a, 0x68, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x6f,
    0x74, 0x61, 0x6c, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x20, 0x53, 0x71, 0x75, 0x61,
    0x73, 0x68, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x20, 0x61, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x77, 0x61, 0x69, 0x74,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x69, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x61, 0x74,
    0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73,
    0x20, 0x74, 0x6f, 0x20, 0x36, 0x30, 0x0a, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x37, 0x02, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x37, 0x1b, 0x2d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x37, 0x30, 0x31, 0x0a, 0x9b, 0x01, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x3b, 0x02, 0x36, 0x1a, 0x8d, 0x01, 0x20, 0x41, 0x6d, 0x6f,
    0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x70,
    0x6f, 0x6c, 0x6c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68,
    0x6d, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x53, 0x71, 0x75, 0x61, 0x73, 0x68, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x0a, 0x20, 0x28, 0x74, 0x6f, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x69, 0x66, 0x20, 0x68,
    0x61, 0x73, 0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64,
    0x29, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x31,
    0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x06, 0x12, 0x03, 0x3b, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x3b, 0x1b, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x3b, 0x34, 0x35, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.http.squash.v3.serde.rs");
// @@protoc_insertion_point(module)