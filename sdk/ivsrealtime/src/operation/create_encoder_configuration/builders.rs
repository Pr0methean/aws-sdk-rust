// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_encoder_configuration::_create_encoder_configuration_output::CreateEncoderConfigurationOutputBuilder;

pub use crate::operation::create_encoder_configuration::_create_encoder_configuration_input::CreateEncoderConfigurationInputBuilder;

impl CreateEncoderConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_encoder_configuration::CreateEncoderConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_encoder_configuration::CreateEncoderConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_encoder_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateEncoderConfiguration`.
///
/// <p>Creates an EncoderConfiguration object.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateEncoderConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_encoder_configuration::builders::CreateEncoderConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_encoder_configuration::CreateEncoderConfigurationOutput,
        crate::operation::create_encoder_configuration::CreateEncoderConfigurationError,
    > for CreateEncoderConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_encoder_configuration::CreateEncoderConfigurationOutput,
            crate::operation::create_encoder_configuration::CreateEncoderConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateEncoderConfigurationFluentBuilder {
    /// Creates a new `CreateEncoderConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateEncoderConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::create_encoder_configuration::builders::CreateEncoderConfigurationInputBuilder {
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
        crate::operation::create_encoder_configuration::CreateEncoderConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_encoder_configuration::CreateEncoderConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_encoder_configuration::CreateEncoderConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_encoder_configuration::CreateEncoderConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_encoder_configuration::CreateEncoderConfigurationOutput,
        crate::operation::create_encoder_configuration::CreateEncoderConfigurationError,
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
    /// <p>Optional name to identify the resource.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>Optional name to identify the resource.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>Optional name to identify the resource.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>Video configuration. Default: video resolution 1280x720, bitrate 2500 kbps, 30 fps.</p>
    pub fn video(mut self, input: crate::types::Video) -> Self {
        self.inner = self.inner.video(input);
        self
    }
    /// <p>Video configuration. Default: video resolution 1280x720, bitrate 2500 kbps, 30 fps.</p>
    pub fn set_video(mut self, input: ::std::option::Option<crate::types::Video>) -> Self {
        self.inner = self.inner.set_video(input);
        self
    }
    /// <p>Video configuration. Default: video resolution 1280x720, bitrate 2500 kbps, 30 fps.</p>
    pub fn get_video(&self) -> &::std::option::Option<crate::types::Video> {
        self.inner.get_video()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags attached to the resource. Array of maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> for details, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no constraints on tags beyond what is documented there.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Tags attached to the resource. Array of maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> for details, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no constraints on tags beyond what is documented there.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Tags attached to the resource. Array of maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> for details, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no constraints on tags beyond what is documented there.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
