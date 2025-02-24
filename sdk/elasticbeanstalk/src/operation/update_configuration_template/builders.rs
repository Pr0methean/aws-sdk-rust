// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_configuration_template::_update_configuration_template_output::UpdateConfigurationTemplateOutputBuilder;

pub use crate::operation::update_configuration_template::_update_configuration_template_input::UpdateConfigurationTemplateInputBuilder;

impl UpdateConfigurationTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_configuration_template::UpdateConfigurationTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_configuration_template::UpdateConfigurationTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_configuration_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateConfigurationTemplate`.
///
/// <p>Updates the specified configuration template to have the specified properties or configuration option values.</p><note>
/// <p>If a property (for example, <code>ApplicationName</code>) is not provided, its value remains unchanged. To clear such properties, specify an empty string.</p>
/// </note>
/// <p>Related Topics</p>
/// <ul>
/// <li>
/// <p><code>DescribeConfigurationOptions</code></p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateConfigurationTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_configuration_template::builders::UpdateConfigurationTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_configuration_template::UpdateConfigurationTemplateOutput,
        crate::operation::update_configuration_template::UpdateConfigurationTemplateError,
    > for UpdateConfigurationTemplateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_configuration_template::UpdateConfigurationTemplateOutput,
            crate::operation::update_configuration_template::UpdateConfigurationTemplateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateConfigurationTemplateFluentBuilder {
    /// Creates a new `UpdateConfigurationTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateConfigurationTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::update_configuration_template::builders::UpdateConfigurationTemplateInputBuilder {
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
        crate::operation::update_configuration_template::UpdateConfigurationTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_configuration_template::UpdateConfigurationTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_configuration_template::UpdateConfigurationTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_configuration_template::UpdateConfigurationTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_configuration_template::UpdateConfigurationTemplateOutput,
        crate::operation::update_configuration_template::UpdateConfigurationTemplateError,
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
    /// <p>The name of the application associated with the configuration template to update.</p>
    /// <p>If no application is found with this name, <code>UpdateConfigurationTemplate</code> returns an <code>InvalidParameterValue</code> error.</p>
    pub fn application_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of the application associated with the configuration template to update.</p>
    /// <p>If no application is found with this name, <code>UpdateConfigurationTemplate</code> returns an <code>InvalidParameterValue</code> error.</p>
    pub fn set_application_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>The name of the application associated with the configuration template to update.</p>
    /// <p>If no application is found with this name, <code>UpdateConfigurationTemplate</code> returns an <code>InvalidParameterValue</code> error.</p>
    pub fn get_application_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_name()
    }
    /// <p>The name of the configuration template to update.</p>
    /// <p>If no configuration template is found with this name, <code>UpdateConfigurationTemplate</code> returns an <code>InvalidParameterValue</code> error.</p>
    pub fn template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_name(input.into());
        self
    }
    /// <p>The name of the configuration template to update.</p>
    /// <p>If no configuration template is found with this name, <code>UpdateConfigurationTemplate</code> returns an <code>InvalidParameterValue</code> error.</p>
    pub fn set_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_name(input);
        self
    }
    /// <p>The name of the configuration template to update.</p>
    /// <p>If no configuration template is found with this name, <code>UpdateConfigurationTemplate</code> returns an <code>InvalidParameterValue</code> error.</p>
    pub fn get_template_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_name()
    }
    /// <p>A new description for the configuration.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A new description for the configuration.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A new description for the configuration.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Appends an item to `OptionSettings`.
    ///
    /// To override the contents of this collection use [`set_option_settings`](Self::set_option_settings).
    ///
    /// <p>A list of configuration option settings to update with the new specified option value.</p>
    pub fn option_settings(mut self, input: crate::types::ConfigurationOptionSetting) -> Self {
        self.inner = self.inner.option_settings(input);
        self
    }
    /// <p>A list of configuration option settings to update with the new specified option value.</p>
    pub fn set_option_settings(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ConfigurationOptionSetting>>) -> Self {
        self.inner = self.inner.set_option_settings(input);
        self
    }
    /// <p>A list of configuration option settings to update with the new specified option value.</p>
    pub fn get_option_settings(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ConfigurationOptionSetting>> {
        self.inner.get_option_settings()
    }
    /// Appends an item to `OptionsToRemove`.
    ///
    /// To override the contents of this collection use [`set_options_to_remove`](Self::set_options_to_remove).
    ///
    /// <p>A list of configuration options to remove from the configuration set.</p>
    /// <p>Constraint: You can remove only <code>UserDefined</code> configuration options.</p>
    pub fn options_to_remove(mut self, input: crate::types::OptionSpecification) -> Self {
        self.inner = self.inner.options_to_remove(input);
        self
    }
    /// <p>A list of configuration options to remove from the configuration set.</p>
    /// <p>Constraint: You can remove only <code>UserDefined</code> configuration options.</p>
    pub fn set_options_to_remove(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::OptionSpecification>>) -> Self {
        self.inner = self.inner.set_options_to_remove(input);
        self
    }
    /// <p>A list of configuration options to remove from the configuration set.</p>
    /// <p>Constraint: You can remove only <code>UserDefined</code> configuration options.</p>
    pub fn get_options_to_remove(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::OptionSpecification>> {
        self.inner.get_options_to_remove()
    }
}
