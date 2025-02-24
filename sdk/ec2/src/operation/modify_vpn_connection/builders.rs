// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_vpn_connection::_modify_vpn_connection_output::ModifyVpnConnectionOutputBuilder;

pub use crate::operation::modify_vpn_connection::_modify_vpn_connection_input::ModifyVpnConnectionInputBuilder;

impl ModifyVpnConnectionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_vpn_connection::ModifyVpnConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_vpn_connection::ModifyVpnConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_vpn_connection();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyVpnConnection`.
///
/// <p>Modifies the customer gateway or the target gateway of an Amazon Web Services Site-to-Site VPN connection. To modify the target gateway, the following migration options are available:</p>
/// <ul>
/// <li>
/// <p>An existing virtual private gateway to a new virtual private gateway</p></li>
/// <li>
/// <p>An existing virtual private gateway to a transit gateway</p></li>
/// <li>
/// <p>An existing transit gateway to a new transit gateway</p></li>
/// <li>
/// <p>An existing transit gateway to a virtual private gateway</p></li>
/// </ul>
/// <p>Before you perform the migration to the new gateway, you must configure the new gateway. Use <code>CreateVpnGateway</code> to create a virtual private gateway, or <code>CreateTransitGateway</code> to create a transit gateway.</p>
/// <p>This step is required when you migrate from a virtual private gateway with static routes to a transit gateway.</p>
/// <p>You must delete the static routes before you migrate to the new gateway.</p>
/// <p>Keep a copy of the static route before you delete it. You will need to add back these routes to the transit gateway after the VPN connection migration is complete.</p>
/// <p>After you migrate to the new gateway, you might need to modify your VPC route table. Use <code>CreateRoute</code> and <code>DeleteRoute</code> to make the changes described in <a href="https://docs.aws.amazon.com/vpn/latest/s2svpn/modify-vpn-target.html#step-update-routing">Update VPC route tables</a> in the <i>Amazon Web Services Site-to-Site VPN User Guide</i>.</p>
/// <p>When the new gateway is a transit gateway, modify the transit gateway route table to allow traffic between the VPC and the Amazon Web Services Site-to-Site VPN connection. Use <code>CreateTransitGatewayRoute</code> to add the routes.</p>
/// <p>If you deleted VPN static routes, you must add the static routes to the transit gateway route table.</p>
/// <p>After you perform this operation, the VPN endpoint's IP addresses on the Amazon Web Services side and the tunnel options remain intact. Your Amazon Web Services Site-to-Site VPN connection will be temporarily unavailable for a brief period while we provision the new endpoints.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyVpnConnectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_vpn_connection::builders::ModifyVpnConnectionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_vpn_connection::ModifyVpnConnectionOutput,
        crate::operation::modify_vpn_connection::ModifyVpnConnectionError,
    > for ModifyVpnConnectionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_vpn_connection::ModifyVpnConnectionOutput,
            crate::operation::modify_vpn_connection::ModifyVpnConnectionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyVpnConnectionFluentBuilder {
    /// Creates a new `ModifyVpnConnection`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyVpnConnection as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_vpn_connection::builders::ModifyVpnConnectionInputBuilder {
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
        crate::operation::modify_vpn_connection::ModifyVpnConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_vpn_connection::ModifyVpnConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_vpn_connection::ModifyVpnConnection::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_vpn_connection::ModifyVpnConnection::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_vpn_connection::ModifyVpnConnectionOutput,
        crate::operation::modify_vpn_connection::ModifyVpnConnectionError,
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
    /// <p>The ID of the VPN connection.</p>
    pub fn vpn_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpn_connection_id(input.into());
        self
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn set_vpn_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpn_connection_id(input);
        self
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn get_vpn_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpn_connection_id()
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn transit_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_id(input.into());
        self
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn set_transit_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transit_gateway_id(input);
        self
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn get_transit_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transit_gateway_id()
    }
    /// <p>The ID of the customer gateway at your end of the VPN connection.</p>
    pub fn customer_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.customer_gateway_id(input.into());
        self
    }
    /// <p>The ID of the customer gateway at your end of the VPN connection.</p>
    pub fn set_customer_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_customer_gateway_id(input);
        self
    }
    /// <p>The ID of the customer gateway at your end of the VPN connection.</p>
    pub fn get_customer_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_customer_gateway_id()
    }
    /// <p>The ID of the virtual private gateway at the Amazon Web Services side of the VPN connection.</p>
    pub fn vpn_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpn_gateway_id(input.into());
        self
    }
    /// <p>The ID of the virtual private gateway at the Amazon Web Services side of the VPN connection.</p>
    pub fn set_vpn_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpn_gateway_id(input);
        self
    }
    /// <p>The ID of the virtual private gateway at the Amazon Web Services side of the VPN connection.</p>
    pub fn get_vpn_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpn_gateway_id()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
