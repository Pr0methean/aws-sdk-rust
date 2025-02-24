// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_resource_event_configuration::_get_resource_event_configuration_output::GetResourceEventConfigurationOutputBuilder;

pub use crate::operation::get_resource_event_configuration::_get_resource_event_configuration_input::GetResourceEventConfigurationInputBuilder;

impl GetResourceEventConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_resource_event_configuration::GetResourceEventConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_resource_event_configuration::GetResourceEventConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_resource_event_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetResourceEventConfiguration`.
///
/// <p>Get the event configuration for a particular resource identifier.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetResourceEventConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_resource_event_configuration::builders::GetResourceEventConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_resource_event_configuration::GetResourceEventConfigurationOutput,
        crate::operation::get_resource_event_configuration::GetResourceEventConfigurationError,
    > for GetResourceEventConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_resource_event_configuration::GetResourceEventConfigurationOutput,
            crate::operation::get_resource_event_configuration::GetResourceEventConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetResourceEventConfigurationFluentBuilder {
    /// Creates a new `GetResourceEventConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetResourceEventConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::get_resource_event_configuration::builders::GetResourceEventConfigurationInputBuilder {
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
        crate::operation::get_resource_event_configuration::GetResourceEventConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_resource_event_configuration::GetResourceEventConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_resource_event_configuration::GetResourceEventConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_resource_event_configuration::GetResourceEventConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_resource_event_configuration::GetResourceEventConfigurationOutput,
        crate::operation::get_resource_event_configuration::GetResourceEventConfigurationError,
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
    /// <p>Resource identifier to opt in for event messaging.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// <p>Resource identifier to opt in for event messaging.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
    /// <p>Resource identifier to opt in for event messaging.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identifier()
    }
    /// <p>Identifier type of the particular resource identifier for event configuration.</p>
    pub fn identifier_type(mut self, input: crate::types::IdentifierType) -> Self {
        self.inner = self.inner.identifier_type(input);
        self
    }
    /// <p>Identifier type of the particular resource identifier for event configuration.</p>
    pub fn set_identifier_type(mut self, input: ::std::option::Option<crate::types::IdentifierType>) -> Self {
        self.inner = self.inner.set_identifier_type(input);
        self
    }
    /// <p>Identifier type of the particular resource identifier for event configuration.</p>
    pub fn get_identifier_type(&self) -> &::std::option::Option<crate::types::IdentifierType> {
        self.inner.get_identifier_type()
    }
    /// <p>Partner type of the resource if the identifier type is <code>PartnerAccountId</code>.</p>
    pub fn partner_type(mut self, input: crate::types::EventNotificationPartnerType) -> Self {
        self.inner = self.inner.partner_type(input);
        self
    }
    /// <p>Partner type of the resource if the identifier type is <code>PartnerAccountId</code>.</p>
    pub fn set_partner_type(mut self, input: ::std::option::Option<crate::types::EventNotificationPartnerType>) -> Self {
        self.inner = self.inner.set_partner_type(input);
        self
    }
    /// <p>Partner type of the resource if the identifier type is <code>PartnerAccountId</code>.</p>
    pub fn get_partner_type(&self) -> &::std::option::Option<crate::types::EventNotificationPartnerType> {
        self.inner.get_partner_type()
    }
}
