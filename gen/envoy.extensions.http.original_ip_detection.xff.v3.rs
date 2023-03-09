// @generated
// [#protodoc-title: XFF original IP detection extension]

/// This extension allows for the original downstream remote IP to be detected
/// by reading the :ref:`config_http_conn_man_headers_x-forwarded-for` header.
///
/// [#extension: envoy.http.original_ip_detection.xff]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XffConfig {
    /// The number of additional ingress proxy hops from the right side of the
    /// :ref:`config_http_conn_man_headers_x-forwarded-for` HTTP header to trust when
    /// determining the origin client's IP address. The default is zero if this option
    /// is not specified. See the documentation for
    /// :ref:`config_http_conn_man_headers_x-forwarded-for` for more information.
    #[prost(uint32, tag="1")]
    pub xff_num_trusted_hops: u32,
}
/// Encoded file descriptor set for the `envoy.extensions.http.original_ip_detection.xff.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xe5, 0x09, 0x0a, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x70, 0x5f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x2f, 0x78, 0x66, 0x66, 0x2f, 0x76, 0x33, 0x2f, 0x78, 0x66, 0x66, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x32, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61,
    0x6c, 0x5f, 0x69, 0x70, 0x5f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x78,
    0x66, 0x66, 0x2e, 0x76, 0x33, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3c, 0x0a, 0x09, 0x58, 0x66, 0x66, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x12, 0x2f, 0x0a, 0x14, 0x78, 0x66, 0x66, 0x5f, 0x6e, 0x75, 0x6d, 0x5f, 0x74, 0x72, 0x75,
    0x73, 0x74, 0x65, 0x64, 0x5f, 0x68, 0x6f, 0x70, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x11, 0x78, 0x66, 0x66, 0x4e, 0x75, 0x6d, 0x54, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64, 0x48, 0x6f,
    0x70, 0x73, 0x42, 0xb7, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x40, 0x69,
    0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x68, 0x74,
    0x74, 0x70, 0x2e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x70, 0x5f, 0x64,
    0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x78, 0x66, 0x66, 0x2e, 0x76, 0x33, 0x42,
    0x08, 0x58, 0x66, 0x66, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x5f, 0x67, 0x69, 0x74,
    0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f,
    0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c,
    0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x61, 0x6c, 0x5f, 0x69, 0x70, 0x5f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2f,
    0x78, 0x66, 0x66, 0x2f, 0x76, 0x33, 0x3b, 0x78, 0x66, 0x66, 0x76, 0x33, 0x4a, 0xd1, 0x06, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x19, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x59,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x06, 0x00, 0x59, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x07, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x07, 0x00, 0x29,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a,
    0x12, 0x03, 0x08, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x76, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x09, 0x00, 0x76, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0a, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03,
    0x0a, 0x00, 0x46, 0x0a, 0x94, 0x02, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x12, 0x00, 0x19, 0x01,
    0x1a, 0xcd, 0x01, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x49, 0x50, 0x20,
    0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x65, 0x64, 0x0a, 0x20,
    0x62, 0x79, 0x20, 0x72, 0x65, 0x61, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x3a,
    0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70,
    0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x5f, 0x6d, 0x61, 0x6e, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x73, 0x5f, 0x78, 0x2d, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x2d, 0x66, 0x6f,
    0x72, 0x60, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x20, 0x5b, 0x23, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x68, 0x74, 0x74, 0x70, 0x2e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x70,
    0x5f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x78, 0x66, 0x66, 0x5d, 0x0a,
    0x32, 0x38, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69,
    0x74, 0x6c, 0x65, 0x3a, 0x20, 0x58, 0x46, 0x46, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61,
    0x6c, 0x20, 0x49, 0x50, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x12, 0x08, 0x11, 0x0a, 0xed, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x18, 0x02, 0x22, 0x1a, 0xdf, 0x02, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62,
    0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x20, 0x69, 0x6e, 0x67, 0x72, 0x65, 0x73, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x20, 0x68,
    0x6f, 0x70, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x69, 0x67,
    0x68, 0x74, 0x20, 0x73, 0x69, 0x64, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20,
    0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74,
    0x70, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x5f, 0x6d, 0x61, 0x6e, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x73, 0x5f, 0x78, 0x2d, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x2d, 0x66,
    0x6f, 0x72, 0x60, 0x20, 0x48, 0x54, 0x54, 0x50, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x72, 0x75, 0x73, 0x74, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x0a, 0x20, 0x64,
    0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x27, 0x73, 0x20, 0x49,
    0x50, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x64,
    0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x73, 0x20, 0x7a, 0x65, 0x72, 0x6f, 0x20, 0x69,
    0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x69,
    0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2e,
    0x20, 0x53, 0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66,
    0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x63, 0x6f,
    0x6e, 0x6e, 0x5f, 0x6d, 0x61, 0x6e, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x5f, 0x78,
    0x2d, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x2d, 0x66, 0x6f, 0x72, 0x60, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x18, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18,
    0x09, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x20, 0x21,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)