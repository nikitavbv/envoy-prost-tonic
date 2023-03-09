// @generated
// [#protodoc-title: SafeCrossScheme internal redirect predicate]

/// An internal redirect predicate that checks the scheme between the
/// downstream url and the redirect target url and allows a) same scheme
/// redirect and b) safe cross scheme redirect, which means if the downstream
/// scheme is HTTPS, both HTTPS and HTTP redirect targets are allowed, but if the
/// downstream scheme is HTTP, only HTTP redirect targets are allowed.
/// [#extension: envoy.internal_redirect_predicates.safe_cross_scheme]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeCrossSchemeConfig {
}
/// Encoded file descriptor set for the `envoy.extensions.internal_redirect.safe_cross_scheme.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd7, 0x08, 0x0a, 0x56, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72,
    0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x2f, 0x73, 0x61, 0x66, 0x65, 0x5f, 0x63, 0x72, 0x6f,
    0x73, 0x73, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x73, 0x61, 0x66,
    0x65, 0x5f, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x5f, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x37, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x69, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x2e,
    0x73, 0x61, 0x66, 0x65, 0x5f, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d,
    0x65, 0x2e, 0x76, 0x33, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x17, 0x0a, 0x15, 0x53, 0x61, 0x66, 0x65, 0x43, 0x72, 0x6f, 0x73, 0x73,
    0x53, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x42, 0xe1, 0x01, 0xba,
    0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x45, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x5f, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x2e, 0x73, 0x61, 0x66, 0x65, 0x5f, 0x63,
    0x72, 0x6f, 0x73, 0x73, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x2e, 0x76, 0x33, 0x42, 0x1a,
    0x53, 0x61, 0x66, 0x65, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x72, 0x67, 0x69,
    0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72,
    0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70,
    0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72,
    0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x2f, 0x73, 0x61, 0x66, 0x65, 0x5f, 0x63, 0x72, 0x6f,
    0x73, 0x73, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x2f, 0x76, 0x33, 0x3b, 0x73, 0x61, 0x66,
    0x65, 0x5f, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x76, 0x33,
    0x4a, 0x9f, 0x05, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x15, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x40, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x06, 0x00, 0x5e, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x06, 0x00, 0x5e, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12,
    0x03, 0x07, 0x00, 0x3b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x22, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x08, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04,
    0x09, 0x00, 0x89, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x04, 0x09, 0x00, 0x89, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87,
    0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0a, 0x00, 0x46, 0x0a, 0xfa, 0x03, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x14, 0x00, 0x15, 0x01, 0x1a, 0xab, 0x03, 0x20, 0x41, 0x6e, 0x20, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x20, 0x70,
    0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x68,
    0x65, 0x63, 0x6b, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x20,
    0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x64, 0x6f, 0x77,
    0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x75, 0x72, 0x6c, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x20, 0x74, 0x61, 0x72,
    0x67, 0x65, 0x74, 0x20, 0x75, 0x72, 0x6c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x6c, 0x6c, 0x6f,
    0x77, 0x73, 0x20, 0x61, 0x29, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d,
    0x65, 0x0a, 0x20, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x62, 0x29, 0x20, 0x73, 0x61, 0x66, 0x65, 0x20, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x20, 0x73, 0x63,
    0x68, 0x65, 0x6d, 0x65, 0x20, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x2c, 0x20, 0x77,
    0x68, 0x69, 0x63, 0x68, 0x20, 0x6d, 0x65, 0x61, 0x6e, 0x73, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x0a, 0x20, 0x73, 0x63,
    0x68, 0x65, 0x6d, 0x65, 0x20, 0x69, 0x73, 0x20, 0x48, 0x54, 0x54, 0x50, 0x53, 0x2c, 0x20, 0x62,
    0x6f, 0x74, 0x68, 0x20, 0x48, 0x54, 0x54, 0x50, 0x53, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x48, 0x54,
    0x54, 0x50, 0x20, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x20, 0x74, 0x61, 0x72, 0x67,
    0x65, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x64, 0x2c,
    0x20, 0x62, 0x75, 0x74, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x64, 0x6f, 0x77,
    0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x20, 0x69,
    0x73, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2c, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x48, 0x54, 0x54,
    0x50, 0x20, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x20, 0x74, 0x61, 0x72, 0x67, 0x65,
    0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x64, 0x2e, 0x0a,
    0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x64,
    0x69, 0x72, 0x65, 0x63, 0x74, 0x5f, 0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73,
    0x2e, 0x73, 0x61, 0x66, 0x65, 0x5f, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x73, 0x63, 0x68, 0x65,
    0x6d, 0x65, 0x5d, 0x0a, 0x32, 0x40, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f,
    0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x53, 0x61, 0x66, 0x65, 0x43, 0x72, 0x6f,
    0x73, 0x73, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61,
    0x6c, 0x20, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x20, 0x70, 0x72, 0x65, 0x64, 0x69,
    0x63, 0x61, 0x74, 0x65, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x14,
    0x08, 0x1d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)