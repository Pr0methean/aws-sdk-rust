// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_conversion_configuration::_modify_conversion_configuration_output::ModifyConversionConfigurationOutputBuilder;

pub use crate::operation::modify_conversion_configuration::_modify_conversion_configuration_input::ModifyConversionConfigurationInputBuilder;

impl ModifyConversionConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_conversion_configuration::ModifyConversionConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_conversion_configuration::ModifyConversionConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_conversion_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyConversionConfiguration`.
///
/// <p>Modifies the specified schema conversion configuration using the provided parameters.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyConversionConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_conversion_configuration::builders::ModifyConversionConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_conversion_configuration::ModifyConversionConfigurationOutput,
        crate::operation::modify_conversion_configuration::ModifyConversionConfigurationError,
    > for ModifyConversionConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_conversion_configuration::ModifyConversionConfigurationOutput,
            crate::operation::modify_conversion_configuration::ModifyConversionConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyConversionConfigurationFluentBuilder {
    /// Creates a new `ModifyConversionConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyConversionConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_conversion_configuration::builders::ModifyConversionConfigurationInputBuilder {
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
        crate::operation::modify_conversion_configuration::ModifyConversionConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_conversion_configuration::ModifyConversionConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_conversion_configuration::ModifyConversionConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_conversion_configuration::ModifyConversionConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_conversion_configuration::ModifyConversionConfigurationOutput,
        crate::operation::modify_conversion_configuration::ModifyConversionConfigurationError,
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
    /// <p>The migration project name or Amazon Resource Name (ARN).</p>
    pub fn migration_project_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.migration_project_identifier(input.into());
        self
    }
    /// <p>The migration project name or Amazon Resource Name (ARN).</p>
    pub fn set_migration_project_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_migration_project_identifier(input);
        self
    }
    /// <p>The migration project name or Amazon Resource Name (ARN).</p>
    pub fn get_migration_project_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_migration_project_identifier()
    }
    /// <p>The new conversion configuration.</p>
    pub fn conversion_configuration(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.conversion_configuration(input.into());
        self
    }
    /// <p>The new conversion configuration.</p>
    pub fn set_conversion_configuration(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_conversion_configuration(input);
        self
    }
    /// <p>The new conversion configuration.</p>
    pub fn get_conversion_configuration(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_conversion_configuration()
    }
}
