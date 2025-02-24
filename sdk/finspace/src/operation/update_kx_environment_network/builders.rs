// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_kx_environment_network::_update_kx_environment_network_output::UpdateKxEnvironmentNetworkOutputBuilder;

pub use crate::operation::update_kx_environment_network::_update_kx_environment_network_input::UpdateKxEnvironmentNetworkInputBuilder;

impl UpdateKxEnvironmentNetworkInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_kx_environment_network();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateKxEnvironmentNetwork`.
///
/// <p>Updates environment network to connect to your internal network by using a transit gateway. This API supports request to create a transit gateway attachment from FinSpace VPC to your transit gateway ID and create a custom Route-53 outbound resolvers.</p>
/// <p>Once you send a request to update a network, you cannot change it again. Network update might require termination of any clusters that are running in the existing network.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateKxEnvironmentNetworkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_kx_environment_network::builders::UpdateKxEnvironmentNetworkInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetworkOutput,
        crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetworkError,
    > for UpdateKxEnvironmentNetworkFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetworkOutput,
            crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetworkError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateKxEnvironmentNetworkFluentBuilder {
    /// Creates a new `UpdateKxEnvironmentNetwork`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateKxEnvironmentNetwork as a reference.
    pub fn as_input(&self) -> &crate::operation::update_kx_environment_network::builders::UpdateKxEnvironmentNetworkInputBuilder {
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
        crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetwork::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetwork::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetworkOutput,
        crate::operation::update_kx_environment_network::UpdateKxEnvironmentNetworkError,
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
    /// <p>A unique identifier for the kdb environment.</p>
    pub fn environment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.environment_id(input.into());
        self
    }
    /// <p>A unique identifier for the kdb environment.</p>
    pub fn set_environment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_environment_id(input);
        self
    }
    /// <p>A unique identifier for the kdb environment.</p>
    pub fn get_environment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_environment_id()
    }
    /// <p>Specifies the transit gateway and network configuration to connect the kdb environment to an internal network.</p>
    pub fn transit_gateway_configuration(mut self, input: crate::types::TransitGatewayConfiguration) -> Self {
        self.inner = self.inner.transit_gateway_configuration(input);
        self
    }
    /// <p>Specifies the transit gateway and network configuration to connect the kdb environment to an internal network.</p>
    pub fn set_transit_gateway_configuration(mut self, input: ::std::option::Option<crate::types::TransitGatewayConfiguration>) -> Self {
        self.inner = self.inner.set_transit_gateway_configuration(input);
        self
    }
    /// <p>Specifies the transit gateway and network configuration to connect the kdb environment to an internal network.</p>
    pub fn get_transit_gateway_configuration(&self) -> &::std::option::Option<crate::types::TransitGatewayConfiguration> {
        self.inner.get_transit_gateway_configuration()
    }
    /// Appends an item to `customDNSConfiguration`.
    ///
    /// To override the contents of this collection use [`set_custom_dns_configuration`](Self::set_custom_dns_configuration).
    ///
    /// <p>A list of DNS server name and server IP. This is used to set up Route-53 outbound resolvers.</p>
    pub fn custom_dns_configuration(mut self, input: crate::types::CustomDnsServer) -> Self {
        self.inner = self.inner.custom_dns_configuration(input);
        self
    }
    /// <p>A list of DNS server name and server IP. This is used to set up Route-53 outbound resolvers.</p>
    pub fn set_custom_dns_configuration(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CustomDnsServer>>) -> Self {
        self.inner = self.inner.set_custom_dns_configuration(input);
        self
    }
    /// <p>A list of DNS server name and server IP. This is used to set up Route-53 outbound resolvers.</p>
    pub fn get_custom_dns_configuration(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CustomDnsServer>> {
        self.inner.get_custom_dns_configuration()
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
