// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_vpce_configuration::_update_vpce_configuration_output::UpdateVpceConfigurationOutputBuilder;

pub use crate::operation::update_vpce_configuration::_update_vpce_configuration_input::UpdateVpceConfigurationInputBuilder;

impl UpdateVpceConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_vpce_configuration::UpdateVpceConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_vpce_configuration::UpdateVPCEConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_vpce_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateVPCEConfiguration`.
///
/// <p>Updates information about an Amazon Virtual Private Cloud (VPC) endpoint configuration.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateVPCEConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_vpce_configuration::builders::UpdateVpceConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_vpce_configuration::UpdateVpceConfigurationOutput,
        crate::operation::update_vpce_configuration::UpdateVPCEConfigurationError,
    > for UpdateVPCEConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_vpce_configuration::UpdateVpceConfigurationOutput,
            crate::operation::update_vpce_configuration::UpdateVPCEConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateVPCEConfigurationFluentBuilder {
    /// Creates a new `UpdateVPCEConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateVPCEConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::update_vpce_configuration::builders::UpdateVpceConfigurationInputBuilder {
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
        crate::operation::update_vpce_configuration::UpdateVpceConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_vpce_configuration::UpdateVPCEConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_vpce_configuration::UpdateVPCEConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_vpce_configuration::UpdateVPCEConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_vpce_configuration::UpdateVpceConfigurationOutput,
        crate::operation::update_vpce_configuration::UpdateVPCEConfigurationError,
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
    /// <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration you want to update.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration you want to update.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration you want to update.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_arn()
    }
    /// <p>The friendly name you give to your VPC endpoint configuration to manage your configurations more easily.</p>
    pub fn vpce_configuration_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpce_configuration_name(input.into());
        self
    }
    /// <p>The friendly name you give to your VPC endpoint configuration to manage your configurations more easily.</p>
    pub fn set_vpce_configuration_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpce_configuration_name(input);
        self
    }
    /// <p>The friendly name you give to your VPC endpoint configuration to manage your configurations more easily.</p>
    pub fn get_vpce_configuration_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpce_configuration_name()
    }
    /// <p>The name of the VPC endpoint service running in your AWS account that you want Device Farm to test.</p>
    pub fn vpce_service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpce_service_name(input.into());
        self
    }
    /// <p>The name of the VPC endpoint service running in your AWS account that you want Device Farm to test.</p>
    pub fn set_vpce_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpce_service_name(input);
        self
    }
    /// <p>The name of the VPC endpoint service running in your AWS account that you want Device Farm to test.</p>
    pub fn get_vpce_service_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpce_service_name()
    }
    /// <p>The DNS (domain) name used to connect to your private service in your VPC. The DNS name must not already be in use on the internet.</p>
    pub fn service_dns_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_dns_name(input.into());
        self
    }
    /// <p>The DNS (domain) name used to connect to your private service in your VPC. The DNS name must not already be in use on the internet.</p>
    pub fn set_service_dns_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_dns_name(input);
        self
    }
    /// <p>The DNS (domain) name used to connect to your private service in your VPC. The DNS name must not already be in use on the internet.</p>
    pub fn get_service_dns_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_dns_name()
    }
    /// <p>An optional description that provides details about your VPC endpoint configuration.</p>
    pub fn vpce_configuration_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpce_configuration_description(input.into());
        self
    }
    /// <p>An optional description that provides details about your VPC endpoint configuration.</p>
    pub fn set_vpce_configuration_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpce_configuration_description(input);
        self
    }
    /// <p>An optional description that provides details about your VPC endpoint configuration.</p>
    pub fn get_vpce_configuration_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpce_configuration_description()
    }
}
