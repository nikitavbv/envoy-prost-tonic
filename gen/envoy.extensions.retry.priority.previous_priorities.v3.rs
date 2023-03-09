// @generated
// [#protodoc-title: Previous priorities retry selector]

/// A retry host selector that attempts to spread retries between priorities, even if certain
/// priorities would not normally be attempted due to higher priorities being available.
///
/// As priorities get excluded, load will be distributed amongst the remaining healthy priorities
/// based on the relative health of the priorities, matching how load is distributed during regular
/// host selection. For example, given priority healths of {100, 50, 50}, the original load will be
/// {100, 0, 0} (since P0 has capacity to handle 100% of the traffic). If P0 is excluded, the load
/// changes to {0, 50, 50}, because P1 is only able to handle 50% of the traffic, causing the
/// remaining to spill over to P2.
///
/// Each priority attempted will be excluded until there are no healthy priorities left, at which
/// point the list of attempted priorities will be reset, essentially starting from the beginning.
/// For example, given three priorities P0, P1, P2 with healthy % of 100, 0 and 50 respectively, the
/// following sequence of priorities would be selected (assuming update_frequency = 1):
/// Attempt 1: P0 (P0 is 100% healthy)
/// Attempt 2: P2 (P0 already attempted, P2 only healthy priority)
/// Attempt 3: P0 (no healthy priorities, reset)
/// Attempt 4: P2
///
/// In the case of all upstream hosts being unhealthy, no adjustments will be made to the original
/// priority load, so behavior should be identical to not using this plugin.
///
/// Using this PriorityFilter requires rebuilding the priority load, which runs in O(# of
/// priorities), which might incur significant overhead for clusters with many priorities.
/// [#extension: envoy.retry_priorities.previous_priorities]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviousPrioritiesConfig {
    /// How often the priority load should be updated based on previously attempted priorities. Useful
    /// to allow each priorities to receive more than one request before being excluded or to reduce
    /// the number of times that the priority load has to be recomputed.
    ///
    /// For example, by setting this to 2, then the first two attempts (initial attempt and first
    /// retry) will use the unmodified priority load. The third and fourth attempt will use priority
    /// load which excludes the priorities routed to with the first two attempts, and the fifth and
    /// sixth attempt will use the priority load excluding the priorities used for the first four
    /// attempts.
    ///
    /// Must be greater than 0.
    #[prost(int32, tag="1")]
    pub update_frequency: i32,
}
/// Encoded file descriptor set for the `envoy.extensions.retry.priority.previous_priorities.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xe3, 0x19, 0x0a, 0x57, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x72, 0x65, 0x74, 0x72, 0x79, 0x2f, 0x70, 0x72, 0x69, 0x6f,
    0x72, 0x69, 0x74, 0x79, 0x2f, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x5f, 0x70, 0x72,
    0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x2f, 0x76, 0x33, 0x2f, 0x70, 0x72, 0x65, 0x76,
    0x69, 0x6f, 0x75, 0x73, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x5f,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x36, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x72,
    0x65, 0x74, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x2e, 0x70, 0x72,
    0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65,
    0x73, 0x2e, 0x76, 0x33, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x21, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x96, 0x01, 0x0a, 0x18, 0x50, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x50, 0x72, 0x69, 0x6f,
    0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x32, 0x0a, 0x10,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x66, 0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x1a, 0x02, 0x20, 0x00, 0x52,
    0x0f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x46, 0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x79,
    0x3a, 0x46, 0x9a, 0xc5, 0x88, 0x1e, 0x41, 0x0a, 0x3f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x65, 0x74, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x65, 0x76,
    0x69, 0x6f, 0x75, 0x73, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x2e,
    0x50, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69,
    0x65, 0x73, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x42, 0xe4, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06,
    0x02, 0x10, 0x02, 0x0a, 0x44, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f,
    0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x72, 0x65, 0x74, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69,
    0x74, 0x79, 0x2e, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x5f, 0x70, 0x72, 0x69, 0x6f,
    0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x2e, 0x76, 0x33, 0x42, 0x1d, 0x50, 0x72, 0x65, 0x76, 0x69,
    0x6f, 0x75, 0x73, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x73, 0x67, 0x69, 0x74, 0x68,
    0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78,
    0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61,
    0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2f, 0x72, 0x65, 0x74, 0x72, 0x79, 0x2f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69,
    0x74, 0x79, 0x2f, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x5f, 0x70, 0x72, 0x69, 0x6f,
    0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x2f, 0x76, 0x33, 0x3b, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f,
    0x75, 0x73, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x76, 0x33, 0x4a,
    0xec, 0x14, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x39, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x3f, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x05, 0x00, 0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x00, 0x21, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x5d, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x08, 0x00, 0x5d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x3e, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x09, 0x00, 0x3e, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0a, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0a, 0x00, 0x22, 0x0a, 0x09,
    0x0a, 0x01, 0x08, 0x12, 0x04, 0x0b, 0x00, 0x8a, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x08, 0x0b, 0x12,
    0x04, 0x0b, 0x00, 0x8a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x46, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0c, 0x00, 0x46, 0x0a, 0xa1,
    0x0d, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x29, 0x00, 0x39, 0x01, 0x1a, 0xdb, 0x0c, 0x20, 0x41,
    0x20, 0x72, 0x65, 0x74, 0x72, 0x79, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x73, 0x65, 0x6c, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70,
    0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x70, 0x72, 0x65, 0x61, 0x64, 0x20, 0x72, 0x65, 0x74,
    0x72, 0x69, 0x65, 0x73, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x70, 0x72, 0x69,
    0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x2c, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x20, 0x69, 0x66,
    0x20, 0x63, 0x65, 0x72, 0x74, 0x61, 0x69, 0x6e, 0x0a, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69,
    0x74, 0x69, 0x65, 0x73, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6e,
    0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x61, 0x74, 0x74, 0x65, 0x6d,
    0x70, 0x74, 0x65, 0x64, 0x20, 0x64, 0x75, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x69, 0x67, 0x68,
    0x65, 0x72, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x20, 0x62, 0x65,
    0x69, 0x6e, 0x67, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x0a,
    0x20, 0x41, 0x73, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x20, 0x67,
    0x65, 0x74, 0x20, 0x65, 0x78, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x64, 0x2c, 0x20, 0x6c, 0x6f, 0x61,
    0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x61, 0x6d, 0x6f, 0x6e, 0x67, 0x73, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x68, 0x65, 0x61, 0x6c,
    0x74, 0x68, 0x79, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x0a, 0x20,
    0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x6c,
    0x61, 0x74, 0x69, 0x76, 0x65, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x2c, 0x20,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x6c, 0x6f, 0x61,
    0x64, 0x20, 0x69, 0x73, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64,
    0x20, 0x64, 0x75, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x0a,
    0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x20, 0x46, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2c, 0x20, 0x67, 0x69,
    0x76, 0x65, 0x6e, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x68, 0x65, 0x61,
    0x6c, 0x74, 0x68, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x7b, 0x31, 0x30, 0x30, 0x2c, 0x20, 0x35, 0x30,
    0x2c, 0x20, 0x35, 0x30, 0x7d, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x61, 0x6c, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65,
    0x0a, 0x20, 0x7b, 0x31, 0x30, 0x30, 0x2c, 0x20, 0x30, 0x2c, 0x20, 0x30, 0x7d, 0x20, 0x28, 0x73,
    0x69, 0x6e, 0x63, 0x65, 0x20, 0x50, 0x30, 0x20, 0x68, 0x61, 0x73, 0x20, 0x63, 0x61, 0x70, 0x61,
    0x63, 0x69, 0x74, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x20, 0x31,
    0x30, 0x30, 0x25, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x66, 0x66,
    0x69, 0x63, 0x29, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x50, 0x30, 0x20, 0x69, 0x73, 0x20, 0x65, 0x78,
    0x63, 0x6c, 0x75, 0x64, 0x65, 0x64, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x61, 0x64,
    0x0a, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x7b, 0x30, 0x2c,
    0x20, 0x35, 0x30, 0x2c, 0x20, 0x35, 0x30, 0x7d, 0x2c, 0x20, 0x62, 0x65, 0x63, 0x61, 0x75, 0x73,
    0x65, 0x20, 0x50, 0x31, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x61, 0x62, 0x6c,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x20, 0x35, 0x30, 0x25, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x66, 0x66, 0x69, 0x63, 0x2c, 0x20,
    0x63, 0x61, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x72, 0x65, 0x6d,
    0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x70, 0x69, 0x6c, 0x6c, 0x20,
    0x6f, 0x76, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x50, 0x32, 0x2e, 0x0a, 0x0a, 0x20, 0x45, 0x61,
    0x63, 0x68, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x61, 0x74, 0x74, 0x65,
    0x6d, 0x70, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x65, 0x78,
    0x63, 0x6c, 0x75, 0x64, 0x65, 0x64, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x20, 0x74, 0x68, 0x65,
    0x72, 0x65, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
    0x79, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x20, 0x6c, 0x65, 0x66,
    0x74, 0x2c, 0x20, 0x61, 0x74, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x0a, 0x20, 0x70, 0x6f, 0x69,
    0x6e, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x61,
    0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x65, 0x64, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74,
    0x69, 0x65, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x73, 0x65,
    0x74, 0x2c, 0x20, 0x65, 0x73, 0x73, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x20, 0x46, 0x6f, 0x72,
    0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2c, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20,
    0x74, 0x68, 0x72, 0x65, 0x65, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73,
    0x20, 0x50, 0x30, 0x2c, 0x20, 0x50, 0x31, 0x2c, 0x20, 0x50, 0x32, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x20, 0x25, 0x20, 0x6f, 0x66, 0x20, 0x31, 0x30,
    0x30, 0x2c, 0x20, 0x30, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x35, 0x30, 0x20, 0x72, 0x65, 0x73, 0x70,
    0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x6c, 0x79, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x66,
    0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x20,
    0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65,
    0x64, 0x20, 0x28, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x5f, 0x66, 0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x3d, 0x20, 0x31,
    0x29, 0x3a, 0x0a, 0x20, 0x41, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x20, 0x31, 0x3a, 0x20, 0x50,
    0x30, 0x20, 0x28, 0x50, 0x30, 0x20, 0x69, 0x73, 0x20, 0x31, 0x30, 0x30, 0x25, 0x20, 0x68, 0x65,
    0x61, 0x6c, 0x74, 0x68, 0x79, 0x29, 0x0a, 0x20, 0x41, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x20,
    0x32, 0x3a, 0x20, 0x50, 0x32, 0x20, 0x28, 0x50, 0x30, 0x20, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64,
    0x79, 0x20, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x50, 0x32, 0x20,
    0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x20, 0x70, 0x72, 0x69,
    0x6f, 0x72, 0x69, 0x74, 0x79, 0x29, 0x0a, 0x20, 0x41, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x20,
    0x33, 0x3a, 0x20, 0x50, 0x30, 0x20, 0x28, 0x6e, 0x6f, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
    0x79, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x2c, 0x20, 0x72, 0x65,
    0x73, 0x65, 0x74, 0x29, 0x0a, 0x20, 0x41, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x20, 0x34, 0x3a,
    0x20, 0x50, 0x32, 0x0a, 0x0a, 0x20, 0x49, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x73,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x73, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x6e,
    0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x2c, 0x20, 0x6e, 0x6f, 0x20, 0x61, 0x64, 0x6a, 0x75,
    0x73, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20,
    0x6d, 0x61, 0x64, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x61, 0x6c, 0x0a, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x6c,
    0x6f, 0x61, 0x64, 0x2c, 0x20, 0x73, 0x6f, 0x20, 0x62, 0x65, 0x68, 0x61, 0x76, 0x69, 0x6f, 0x72,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74,
    0x69, 0x63, 0x61, 0x6c, 0x20, 0x74, 0x6f, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x75, 0x73, 0x69, 0x6e,
    0x67, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x2e, 0x0a, 0x0a,
    0x20, 0x55, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x50, 0x72, 0x69, 0x6f,
    0x72, 0x69, 0x74, 0x79, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69,
    0x72, 0x65, 0x73, 0x20, 0x72, 0x65, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x6c, 0x6f, 0x61, 0x64,
    0x2c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x72, 0x75, 0x6e, 0x73, 0x20, 0x69, 0x6e, 0x20,
    0x4f, 0x28, 0x23, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69,
    0x65, 0x73, 0x29, 0x2c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x6d, 0x69, 0x67, 0x68, 0x74,
    0x20, 0x69, 0x6e, 0x63, 0x75, 0x72, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x66, 0x69, 0x63, 0x61,
    0x6e, 0x74, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x68, 0x65, 0x61, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6d, 0x61,
    0x6e, 0x79, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x2e, 0x0a, 0x20,
    0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x72, 0x65, 0x74, 0x72, 0x79, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74,
    0x69, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x5f, 0x70, 0x72, 0x69,
    0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x5d, 0x0a, 0x32, 0x37, 0x20, 0x5b, 0x23, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x50, 0x72,
    0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65,
    0x73, 0x20, 0x72, 0x65, 0x74, 0x72, 0x79, 0x20, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x29, 0x08, 0x20, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x2a, 0x02, 0x2b, 0x48, 0x0a, 0x10, 0x0a, 0x08, 0x04,
    0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x2a, 0x02, 0x2b, 0x48, 0x0a, 0xa5, 0x05,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x38, 0x02, 0x40, 0x1a, 0x97, 0x05, 0x20, 0x48,
    0x6f, 0x77, 0x20, 0x6f, 0x66, 0x74, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x69,
    0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x62, 0x65, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x20, 0x62, 0x61, 0x73,
    0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x6c, 0x79,
    0x20, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x65, 0x64, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72,
    0x69, 0x74, 0x69, 0x65, 0x73, 0x2e, 0x20, 0x55, 0x73, 0x65, 0x66, 0x75, 0x6c, 0x0a, 0x20, 0x74,
    0x6f, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x70, 0x72, 0x69,
    0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69,
    0x76, 0x65, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x6f, 0x6e, 0x65,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20,
    0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x78, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x64, 0x20, 0x6f,
    0x72, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x64, 0x75, 0x63, 0x65, 0x0a, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69,
    0x74, 0x79, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x68, 0x61, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62,
    0x65, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x75, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x20,
    0x46, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2c, 0x20, 0x62, 0x79, 0x20,
    0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x20,
    0x32, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73,
    0x74, 0x20, 0x74, 0x77, 0x6f, 0x20, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x73, 0x20, 0x28,
    0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x20, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x0a, 0x20, 0x72, 0x65, 0x74, 0x72, 0x79,
    0x29, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75,
    0x6e, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69,
    0x74, 0x79, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x68, 0x69,
    0x72, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x66, 0x6f, 0x75, 0x72, 0x74, 0x68, 0x20, 0x61, 0x74,
    0x74, 0x65, 0x6d, 0x70, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x75, 0x73, 0x65, 0x20, 0x70,
    0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x0a, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x77, 0x68,
    0x69, 0x63, 0x68, 0x20, 0x65, 0x78, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x20, 0x72, 0x6f, 0x75, 0x74,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66,
    0x69, 0x72, 0x73, 0x74, 0x20, 0x74, 0x77, 0x6f, 0x20, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74,
    0x73, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x66, 0x74, 0x68,
    0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x73, 0x69, 0x78, 0x74, 0x68, 0x20, 0x61, 0x74, 0x74, 0x65,
    0x6d, 0x70, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x65,
    0x78, 0x63, 0x6c, 0x75, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x69,
    0x6f, 0x72, 0x69, 0x74, 0x69, 0x65, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x66, 0x6f, 0x75, 0x72, 0x0a,
    0x20, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x20, 0x4d, 0x75, 0x73,
    0x74, 0x20, 0x62, 0x65, 0x20, 0x67, 0x72, 0x65, 0x61, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61,
    0x6e, 0x20, 0x30, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x38, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x38, 0x08,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x38, 0x1b, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x38, 0x1d, 0x3f, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x03, 0x12, 0x03, 0x38, 0x1e, 0x3e, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.retry.priority.previous_priorities.v3.serde.rs");
// @@protoc_insertion_point(module)