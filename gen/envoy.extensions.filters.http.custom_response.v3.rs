// @generated
// [#protodoc-title: Custom Response Filter]
// [#extension: envoy.filters.http.custom_response]

// The Custom Response Filter allows for replacing upstream responses.

/// The filter configuration is a collection of custom response
/// policies in a matcher tree. The configuration can be defined at the filter,
/// virtual host or route level. The most specific configuration will apply.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResponse {
    /// Matcher to match against the original response to select a
    /// :ref:`Custom Response Policy <extension_category_envoy.http.custom_response>`
    /// that will override the original response. The matching is done by matching
    /// against :ref:`response header values<extension_category_envoy.matching.http.input>`
    /// Example:
    ///
    /// .. validated-code-block:: yaml
    ///    :type-name: xds.type.matcher.v3.Matcher
    ///
    ///    matcher_list:
    ///      matchers:
    ///        # Apply a locally stored custom response to any 4xx response.
    ///      - predicate:
    ///          single_predicate:
    ///            input:
    ///              name: 4xx_response
    ///              typed_config:
    ///                "@type": type.googleapis.com/envoy.type.matcher.v3.HttpResponseStatusCodeClassMatchInput
    ///            value_match:
    ///              exact: "4xx"
    ///        on_match:
    ///          action:
    ///            name: action
    ///            typed_config:
    ///              "@type": type.googleapis.com/envoy.extensions.filters.http.custom_response.v3.LocalResponsePolicy
    ///              status_code: 499
    ///              body:
    ///                inline_string: "not allowed"
    ///              body_format:
    ///                json_format:
    ///                  status: "%RESPONSE_CODE%"
    ///                  message: "%LOCAL_REPLY_BODY%"
    ///              response_headers_to_add:
    ///              - header:
    ///                  key: "foo"
    ///                  value: "x-bar"
    ///        # Redirect to different upstream if the status code is one of 502, 503 or 504.
    ///      - predicate:
    ///          or_matcher:
    ///            predicate:
    ///            - single_predicate:
    ///                input:
    ///                  name: "502_response"
    ///                  typed_config:
    ///                    "@type": type.googleapis.com/envoy.type.matcher.v3.HttpResponseStatusCodeMatchInput
    ///                value_match:
    ///                  exact: "502"
    ///            - single_predicate:
    ///                input:
    ///                  name: "503_response"
    ///                  typed_config:
    ///                    "@type": type.googleapis.com/envoy.type.matcher.v3.HttpResponseStatusCodeMatchInput
    ///                value_match:
    ///                  exact: "503"
    ///            - single_predicate:
    ///                input:
    ///                  name: "504_response"
    ///                  typed_config:
    ///                    "@type": type.googleapis.com/envoy.type.matcher.v3.HttpResponseStatusCodeMatchInput
    ///                value_match:
    ///                  exact: "504"
    ///        on_match:
    ///          action:
    ///            name: action
    ///            typed_config:
    ///              "@type": type.googleapis.com/envoy.extensions.filters.http.custom_response.v3.RedirectPolicy
    ///              status_code: 299
    ///              uri: "<https://foo.example/gateway_error">
    ///              response_headers_to_add:
    ///              - header:
    ///                  key: "foo2"
    ///                  value: "x-bar2"
    ///
    /// -- attention::
    ///   The first matched policy wins. Once the response is matched, matcher
    ///   evaluations end.
    ///
    /// Refer to :ref:`Unified Matcher API <envoy_v3_api_msg_.xds.type.matcher.v3.Matcher>`
    /// documentation for more information on the matcher trees.
    /// [#extension-category: envoy.http.custom_response]
    #[prost(message, optional, tag="1")]
    pub custom_response_matcher: ::core::option::Option<super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher>,
}
/// Encoded file descriptor set for the `envoy.extensions.filters.http.custom_response.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xfd, 0x20, 0x0a, 0x46, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68, 0x74,
    0x74, 0x70, 0x2f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x30, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x63, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x78,
    0x64, 0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76,
    0x33, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x21,
    0x78, 0x64, 0x73, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72,
    0x2f, 0x76, 0x33, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x66, 0x0a, 0x0e, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x54, 0x0a, 0x17, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x78, 0x64, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x76, 0x33, 0x2e, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65,
    0x72, 0x52, 0x15, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x42, 0xd2, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06,
    0x02, 0x10, 0x02, 0xd2, 0xc6, 0xa4, 0xe1, 0x06, 0x02, 0x08, 0x01, 0x0a, 0x3e, 0x69, 0x6f, 0x2e,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x76, 0x33, 0x42, 0x13, 0x43, 0x75, 0x73,
    0x74, 0x6f, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x50, 0x01, 0x5a, 0x69, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2f, 0x76, 0x33, 0x3b, 0x63, 0x75, 0x73, 0x74,
    0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x76, 0x33, 0x4a, 0xd8, 0x1c,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x6a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x39, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05,
    0x00, 0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x27, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x57, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09,
    0x00, 0x57, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x34, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x34, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00,
    0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x01,
    0x08, 0x12, 0x04, 0x0c, 0x00, 0x80, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x04, 0x0c,
    0x00, 0x80, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0e, 0x00, 0x40, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xea, 0xc8, 0x94, 0x6c, 0x01,
    0x12, 0x03, 0x0e, 0x00, 0x40, 0x0a, 0x87, 0x03, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x18, 0x00,
    0x6a, 0x01, 0x1a, 0xd4, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69,
    0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f,
    0x66, 0x20, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x0a, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x69, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x61,
    0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x20, 0x74, 0x72, 0x65, 0x65, 0x2e, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20,
    0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2c, 0x0a, 0x20,
    0x76, 0x69, 0x72, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x6f, 0x72, 0x20,
    0x72, 0x6f, 0x75, 0x74, 0x65, 0x20, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x2e, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x6d, 0x6f, 0x73, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x2e, 0x0a, 0x32, 0x5d, 0x20, 0x5b, 0x23, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x43, 0x75,
    0x73, 0x74, 0x6f, 0x6d, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x46, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x0a, 0x32, 0x45, 0x20, 0x54, 0x68, 0x65, 0x20, 0x43,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x46,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x18, 0x08, 0x16, 0x0a, 0xd0, 0x17, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x69, 0x02, 0x3a, 0x1a, 0xc2, 0x17, 0x20, 0x4d, 0x61, 0x74,
    0x63, 0x68, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x61, 0x67,
    0x61, 0x69, 0x6e, 0x73, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x61, 0x6c, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x73,
    0x65, 0x6c, 0x65, 0x63, 0x74, 0x20, 0x61, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x43,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x50,
    0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x3c, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x5f, 0x63, 0x61, 0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0x5f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x68, 0x74, 0x74, 0x70, 0x2e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x3e, 0x60, 0x0a, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x69,
    0x73, 0x20, 0x64, 0x6f, 0x6e, 0x65, 0x20, 0x62, 0x79, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69,
    0x6e, 0x67, 0x0a, 0x20, 0x61, 0x67, 0x61, 0x69, 0x6e, 0x73, 0x74, 0x20, 0x3a, 0x72, 0x65, 0x66,
    0x3a, 0x60, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x3c, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x5f, 0x63, 0x61, 0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0x5f, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e,
    0x69, 0x6e, 0x70, 0x75, 0x74, 0x3e, 0x60, 0x0a, 0x20, 0x45, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65,
    0x3a, 0x0a, 0x0a, 0x20, 0x2e, 0x2e, 0x20, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x64,
    0x2d, 0x63, 0x6f, 0x64, 0x65, 0x2d, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x3a, 0x3a, 0x20, 0x79, 0x61,
    0x6d, 0x6c, 0x0a, 0x20, 0x20, 0x20, 0x3a, 0x74, 0x79, 0x70, 0x65, 0x2d, 0x6e, 0x61, 0x6d, 0x65,
    0x3a, 0x20, 0x78, 0x64, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x65, 0x72, 0x2e, 0x76, 0x33, 0x2e, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x0a, 0x0a, 0x20,
    0x20, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x3a, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x73, 0x3a, 0x0a, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x23, 0x20, 0x41, 0x70, 0x70, 0x6c, 0x79, 0x20, 0x61, 0x20,
    0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x63,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x34, 0x78, 0x78, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x2e, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2d, 0x20, 0x70, 0x72, 0x65, 0x64, 0x69,
    0x63, 0x61, 0x74, 0x65, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x73,
    0x69, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x3a,
    0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x69, 0x6e, 0x70, 0x75,
    0x74, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x6e, 0x61, 0x6d, 0x65, 0x3a, 0x20, 0x34, 0x78, 0x78, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x3a, 0x0a, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22, 0x40, 0x74,
    0x79, 0x70, 0x65, 0x22, 0x3a, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x61, 0x70, 0x69, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x76, 0x33, 0x2e,
    0x48, 0x74, 0x74, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x43, 0x6f, 0x64, 0x65, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x4d, 0x61, 0x74, 0x63, 0x68,
    0x49, 0x6e, 0x70, 0x75, 0x74, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x3a, 0x0a, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x65, 0x78, 0x61, 0x63, 0x74,
    0x3a, 0x20, 0x22, 0x34, 0x78, 0x78, 0x22, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x6f,
    0x6e, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x3a, 0x20, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x74, 0x79, 0x70,
    0x65, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22, 0x40, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3a,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x61, 0x70, 0x69, 0x73,
    0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74,
    0x74, 0x70, 0x2e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f,
    0x64, 0x65, 0x3a, 0x20, 0x34, 0x39, 0x39, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x62, 0x6f, 0x64, 0x79, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x69, 0x6e, 0x6c, 0x69, 0x6e, 0x65,
    0x5f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3a, 0x20, 0x22, 0x6e, 0x6f, 0x74, 0x20, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x65, 0x64, 0x22, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x62, 0x6f, 0x64, 0x79, 0x5f, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x3a,
    0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x6a, 0x73, 0x6f, 0x6e, 0x5f, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x3a, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x73, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x3a, 0x20, 0x22, 0x25, 0x52, 0x45, 0x53, 0x50, 0x4f, 0x4e, 0x53, 0x45,
    0x5f, 0x43, 0x4f, 0x44, 0x45, 0x25, 0x22, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x3a, 0x20, 0x22, 0x25, 0x4c, 0x4f, 0x43, 0x41, 0x4c, 0x5f, 0x52, 0x45, 0x50, 0x4c, 0x59, 0x5f,
    0x42, 0x4f, 0x44, 0x59, 0x25, 0x22, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x68, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x73, 0x5f, 0x74, 0x6f, 0x5f, 0x61, 0x64, 0x64, 0x3a, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2d, 0x20, 0x68, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x6b, 0x65, 0x79, 0x3a, 0x20, 0x22, 0x66, 0x6f, 0x6f, 0x22, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x20, 0x22, 0x78, 0x2d, 0x62, 0x61, 0x72, 0x22, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x23, 0x20, 0x52, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x75,
    0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x6e,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x35, 0x30, 0x32, 0x2c, 0x20, 0x35, 0x30, 0x33, 0x20, 0x6f, 0x72,
    0x20, 0x35, 0x30, 0x34, 0x2e, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2d, 0x20, 0x70, 0x72, 0x65,
    0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x6f, 0x72, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x3a, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74,
    0x65, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2d, 0x20,
    0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65,
    0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x3a, 0x20, 0x22,
    0x35, 0x30, 0x32, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x0a, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x3a, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x22, 0x40, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3a, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x61, 0x70, 0x69, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x2e,
    0x76, 0x33, 0x2e, 0x48, 0x74, 0x74, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f, 0x64, 0x65, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x6e,
    0x70, 0x75, 0x74, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x3a, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x65, 0x78, 0x61, 0x63, 0x74, 0x3a, 0x20, 0x22, 0x35, 0x30, 0x32, 0x22, 0x0a, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2d, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c,
    0x65, 0x5f, 0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x3a, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x69, 0x6e, 0x70, 0x75,
    0x74, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x3a, 0x20, 0x22, 0x35, 0x30, 0x33, 0x5f, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x74, 0x79, 0x70, 0x65, 0x64, 0x5f,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22, 0x40, 0x74, 0x79, 0x70,
    0x65, 0x22, 0x3a, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x61,
    0x70, 0x69, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x74, 0x79,
    0x70, 0x65, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x76, 0x33, 0x2e, 0x48, 0x74,
    0x74, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x43, 0x6f, 0x64, 0x65, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x0a, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x65, 0x78, 0x61, 0x63,
    0x74, 0x3a, 0x20, 0x22, 0x35, 0x30, 0x33, 0x22, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x2d, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x70, 0x72, 0x65,
    0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x3a, 0x0a, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x6e,
    0x61, 0x6d, 0x65, 0x3a, 0x20, 0x22, 0x35, 0x30, 0x34, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x22, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x74, 0x79, 0x70, 0x65, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22, 0x40, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3a, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x61, 0x70, 0x69, 0x73, 0x2e, 0x63,
    0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x76, 0x33, 0x2e, 0x48, 0x74, 0x74, 0x70, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f, 0x64, 0x65, 0x4d,
    0x61, 0x74, 0x63, 0x68, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x65, 0x78, 0x61, 0x63, 0x74, 0x3a, 0x20, 0x22, 0x35,
    0x30, 0x34, 0x22, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x6f, 0x6e, 0x5f, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x6e, 0x61, 0x6d, 0x65, 0x3a, 0x20, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x74, 0x79, 0x70, 0x65, 0x64, 0x5f, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x22, 0x40, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3a, 0x20, 0x74, 0x79, 0x70,
    0x65, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x61, 0x70, 0x69, 0x73, 0x2e, 0x63, 0x6f, 0x6d,
    0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x63,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x76,
    0x33, 0x2e, 0x52, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79,
    0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x73, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x3a, 0x20, 0x32, 0x39, 0x39, 0x0a, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x75, 0x72, 0x69, 0x3a,
    0x20, 0x22, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x66, 0x6f, 0x6f, 0x2e, 0x65, 0x78,
    0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2f, 0x67, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x5f, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x22, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x73, 0x5f, 0x74, 0x6f, 0x5f, 0x61, 0x64, 0x64, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2d, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x6b, 0x65, 0x79, 0x3a, 0x20, 0x22, 0x66, 0x6f, 0x6f, 0x32, 0x22, 0x0a, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x20, 0x22, 0x78, 0x2d, 0x62, 0x61, 0x72, 0x32, 0x22, 0x0a,
    0x0a, 0x20, 0x2d, 0x2d, 0x20, 0x61, 0x74, 0x74, 0x65, 0x6e, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x3a,
    0x0a, 0x20, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x65, 0x64, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x77, 0x69, 0x6e, 0x73,
    0x2e, 0x20, 0x4f, 0x6e, 0x63, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x64, 0x2c, 0x20,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x0a, 0x20, 0x20, 0x65, 0x76, 0x61, 0x6c, 0x75, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x65, 0x6e, 0x64, 0x2e, 0x0a, 0x0a, 0x20, 0x52, 0x65, 0x66,
    0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x55, 0x6e, 0x69, 0x66,
    0x69, 0x65, 0x64, 0x20, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x20, 0x41, 0x50, 0x49, 0x20,
    0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x6d, 0x73,
    0x67, 0x5f, 0x2e, 0x78, 0x64, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x6d, 0x61, 0x74, 0x63,
    0x68, 0x65, 0x72, 0x2e, 0x76, 0x33, 0x2e, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x3e, 0x60,
    0x0a, 0x20, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x74, 0x63,
    0x68, 0x65, 0x72, 0x20, 0x74, 0x72, 0x65, 0x65, 0x73, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x2d, 0x63, 0x61, 0x74, 0x65, 0x67, 0x6f, 0x72, 0x79,
    0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x63, 0x75, 0x73,
    0x74, 0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x69, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x1e, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x69, 0x38, 0x39, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.http.custom_response.v3.serde.rs");
// @@protoc_insertion_point(module)