// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_vpc_endpoint_connection_notification::_modify_vpc_endpoint_connection_notification_output::ModifyVpcEndpointConnectionNotificationOutputBuilder;

pub use crate::operation::modify_vpc_endpoint_connection_notification::_modify_vpc_endpoint_connection_notification_input::ModifyVpcEndpointConnectionNotificationInputBuilder;

impl ModifyVpcEndpointConnectionNotificationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_vpc_endpoint_connection_notification();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyVpcEndpointConnectionNotification`.
///
/// <p>Modifies a connection notification for VPC endpoint or VPC endpoint service. You can change the SNS topic for the notification, or the events for which to be notified.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyVpcEndpointConnectionNotificationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationOutput,
        crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationError,
    > for ModifyVpcEndpointConnectionNotificationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationOutput,
            crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyVpcEndpointConnectionNotificationFluentBuilder {
    /// Creates a new `ModifyVpcEndpointConnectionNotification`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyVpcEndpointConnectionNotification as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationInputBuilder {
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
        crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotification::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotification::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationOutput,
        crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationError,
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
    /// <p>The ID of the notification.</p>
    pub fn connection_notification_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_notification_id(input.into());
        self
    }
    /// <p>The ID of the notification.</p>
    pub fn set_connection_notification_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connection_notification_id(input);
        self
    }
    /// <p>The ID of the notification.</p>
    pub fn get_connection_notification_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connection_notification_id()
    }
    /// <p>The ARN for the SNS topic for the notification.</p>
    pub fn connection_notification_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_notification_arn(input.into());
        self
    }
    /// <p>The ARN for the SNS topic for the notification.</p>
    pub fn set_connection_notification_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connection_notification_arn(input);
        self
    }
    /// <p>The ARN for the SNS topic for the notification.</p>
    pub fn get_connection_notification_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connection_notification_arn()
    }
    /// Appends an item to `ConnectionEvents`.
    ///
    /// To override the contents of this collection use [`set_connection_events`](Self::set_connection_events).
    ///
    /// <p>The events for the endpoint. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    pub fn connection_events(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_events(input.into());
        self
    }
    /// <p>The events for the endpoint. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    pub fn set_connection_events(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_connection_events(input);
        self
    }
    /// <p>The events for the endpoint. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p>
    pub fn get_connection_events(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_connection_events()
    }
}
