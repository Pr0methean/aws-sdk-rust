// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::detect_text::_detect_text_output::DetectTextOutputBuilder;

pub use crate::operation::detect_text::_detect_text_input::DetectTextInputBuilder;

impl DetectTextInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::detect_text::DetectTextOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::detect_text::DetectTextError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.detect_text();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DetectText`.
///
/// <p>Detects text in the input image and converts it into machine-readable text.</p>
/// <p>Pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, you must pass it as a reference to an image in an Amazon S3 bucket. For the AWS CLI, passing image bytes is not supported. The image must be either a .png or .jpeg formatted file.</p>
/// <p>The <code>DetectText</code> operation returns text in an array of <code>TextDetection</code> elements, <code>TextDetections</code>. Each <code>TextDetection</code> element provides information about a single word or line of text that was detected in the image.</p>
/// <p>A word is one or more script characters that are not separated by spaces. <code>DetectText</code> can detect up to 100 words in an image.</p>
/// <p>A line is a string of equally spaced words. A line isn't necessarily a complete sentence. For example, a driver's license number is detected as a line. A line ends when there is no aligned text after it. Also, a line ends when there is a large gap between words, relative to the length of the words. This means, depending on the gap between words, Amazon Rekognition may detect multiple lines in text aligned in the same direction. Periods don't represent the end of a line. If a sentence spans multiple lines, the <code>DetectText</code> operation returns multiple lines.</p>
/// <p>To determine whether a <code>TextDetection</code> element is a line of text or a word, use the <code>TextDetection</code> object <code>Type</code> field.</p>
/// <p>To be detected, text must be within +/- 90 degrees orientation of the horizontal axis.</p>
/// <p>For more information, see Detecting text in the Amazon Rekognition Developer Guide.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DetectTextFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::detect_text::builders::DetectTextInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::detect_text::DetectTextOutput,
        crate::operation::detect_text::DetectTextError,
    > for DetectTextFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::detect_text::DetectTextOutput,
            crate::operation::detect_text::DetectTextError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DetectTextFluentBuilder {
    /// Creates a new `DetectText`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DetectText as a reference.
    pub fn as_input(&self) -> &crate::operation::detect_text::builders::DetectTextInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::detect_text::DetectTextOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::detect_text::DetectTextError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::detect_text::DetectText::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::detect_text::DetectText::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::detect_text::DetectTextOutput,
        crate::operation::detect_text::DetectTextError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The input image as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI to call Amazon Rekognition operations, you can't pass image bytes.</p>
    /// <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    pub fn image(mut self, input: crate::types::Image) -> Self {
        self.inner = self.inner.image(input);
        self
    }
    /// <p>The input image as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI to call Amazon Rekognition operations, you can't pass image bytes.</p>
    /// <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    pub fn set_image(mut self, input: ::std::option::Option<crate::types::Image>) -> Self {
        self.inner = self.inner.set_image(input);
        self
    }
    /// <p>The input image as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI to call Amazon Rekognition operations, you can't pass image bytes.</p>
    /// <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    pub fn get_image(&self) -> &::std::option::Option<crate::types::Image> {
        self.inner.get_image()
    }
    /// <p>Optional parameters that let you set the criteria that the text must meet to be included in your response.</p>
    pub fn filters(mut self, input: crate::types::DetectTextFilters) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Optional parameters that let you set the criteria that the text must meet to be included in your response.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<crate::types::DetectTextFilters>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>Optional parameters that let you set the criteria that the text must meet to be included in your response.</p>
    pub fn get_filters(&self) -> &::std::option::Option<crate::types::DetectTextFilters> {
        self.inner.get_filters()
    }
}
