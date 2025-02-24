// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_configuration_set_suppression_options::_put_configuration_set_suppression_options_output::PutConfigurationSetSuppressionOptionsOutputBuilder;

pub use crate::operation::put_configuration_set_suppression_options::_put_configuration_set_suppression_options_input::PutConfigurationSetSuppressionOptionsInputBuilder;

impl PutConfigurationSetSuppressionOptionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_configuration_set_suppression_options();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutConfigurationSetSuppressionOptions`.
///
/// <p>Specify the account suppression list preferences for a configuration set.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutConfigurationSetSuppressionOptionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_configuration_set_suppression_options::builders::PutConfigurationSetSuppressionOptionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsOutput,
        crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsError,
    > for PutConfigurationSetSuppressionOptionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsOutput,
            crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutConfigurationSetSuppressionOptionsFluentBuilder {
    /// Creates a new `PutConfigurationSetSuppressionOptions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutConfigurationSetSuppressionOptions as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::put_configuration_set_suppression_options::builders::PutConfigurationSetSuppressionOptionsInputBuilder {
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
        crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptions::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsOutput,
        crate::operation::put_configuration_set_suppression_options::PutConfigurationSetSuppressionOptionsError,
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
    /// <p>The name of the configuration set to change the suppression list preferences for.</p>
    pub fn configuration_set_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.configuration_set_name(input.into());
        self
    }
    /// <p>The name of the configuration set to change the suppression list preferences for.</p>
    pub fn set_configuration_set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_configuration_set_name(input);
        self
    }
    /// <p>The name of the configuration set to change the suppression list preferences for.</p>
    pub fn get_configuration_set_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_configuration_set_name()
    }
    /// Appends an item to `SuppressedReasons`.
    ///
    /// To override the contents of this collection use [`set_suppressed_reasons`](Self::set_suppressed_reasons).
    ///
    /// <p>A list that contains the reasons that email addresses are automatically added to the suppression list for your account. This list can contain any or all of the following:</p>
    /// <ul>
    /// <li>
    /// <p><code>COMPLAINT</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a complaint.</p></li>
    /// <li>
    /// <p><code>BOUNCE</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a hard bounce.</p></li>
    /// </ul>
    pub fn suppressed_reasons(mut self, input: crate::types::SuppressionListReason) -> Self {
        self.inner = self.inner.suppressed_reasons(input);
        self
    }
    /// <p>A list that contains the reasons that email addresses are automatically added to the suppression list for your account. This list can contain any or all of the following:</p>
    /// <ul>
    /// <li>
    /// <p><code>COMPLAINT</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a complaint.</p></li>
    /// <li>
    /// <p><code>BOUNCE</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a hard bounce.</p></li>
    /// </ul>
    pub fn set_suppressed_reasons(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SuppressionListReason>>) -> Self {
        self.inner = self.inner.set_suppressed_reasons(input);
        self
    }
    /// <p>A list that contains the reasons that email addresses are automatically added to the suppression list for your account. This list can contain any or all of the following:</p>
    /// <ul>
    /// <li>
    /// <p><code>COMPLAINT</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a complaint.</p></li>
    /// <li>
    /// <p><code>BOUNCE</code> – Amazon SES adds an email address to the suppression list for your account when a message sent to that address results in a hard bounce.</p></li>
    /// </ul>
    pub fn get_suppressed_reasons(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SuppressionListReason>> {
        self.inner.get_suppressed_reasons()
    }
}
