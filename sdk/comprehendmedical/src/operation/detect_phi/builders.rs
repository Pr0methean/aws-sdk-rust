// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::detect_phi::_detect_phi_output::DetectPhiOutputBuilder;

pub use crate::operation::detect_phi::_detect_phi_input::DetectPhiInputBuilder;

impl DetectPhiInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::detect_phi::DetectPhiOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::detect_phi::DetectPHIError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.detect_phi();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DetectPHI`.
///
/// <p>Inspects the clinical text for protected health information (PHI) entities and returns the entity category, location, and confidence score for each entity. Amazon Comprehend Medical only detects entities in English language texts.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DetectPHIFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::detect_phi::builders::DetectPhiInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::detect_phi::DetectPhiOutput, crate::operation::detect_phi::DetectPHIError>
    for DetectPHIFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::detect_phi::DetectPhiOutput, crate::operation::detect_phi::DetectPHIError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DetectPHIFluentBuilder {
    /// Creates a new `DetectPHI`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DetectPHI as a reference.
    pub fn as_input(&self) -> &crate::operation::detect_phi::builders::DetectPhiInputBuilder {
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
        crate::operation::detect_phi::DetectPhiOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::detect_phi::DetectPHIError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::detect_phi::DetectPHI::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::detect_phi::DetectPHI::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::detect_phi::DetectPhiOutput,
        crate::operation::detect_phi::DetectPHIError,
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
    /// <p>A UTF-8 text string containing the clinical content being examined for PHI entities.</p>
    pub fn text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.text(input.into());
        self
    }
    /// <p>A UTF-8 text string containing the clinical content being examined for PHI entities.</p>
    pub fn set_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_text(input);
        self
    }
    /// <p>A UTF-8 text string containing the clinical content being examined for PHI entities.</p>
    pub fn get_text(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_text()
    }
}
