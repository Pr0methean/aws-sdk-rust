// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_notification::_delete_notification_output::DeleteNotificationOutputBuilder;

pub use crate::operation::delete_notification::_delete_notification_input::DeleteNotificationInputBuilder;

impl DeleteNotificationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_notification::DeleteNotificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_notification::DeleteNotificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_notification();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteNotification`.
///
/// <p>Deletes a notification.</p><important>
/// <p>Deleting a notification also deletes the subscribers that are associated with the notification.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteNotificationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_notification::builders::DeleteNotificationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_notification::DeleteNotificationOutput,
        crate::operation::delete_notification::DeleteNotificationError,
    > for DeleteNotificationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_notification::DeleteNotificationOutput,
            crate::operation::delete_notification::DeleteNotificationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteNotificationFluentBuilder {
    /// Creates a new `DeleteNotification`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteNotification as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_notification::builders::DeleteNotificationInputBuilder {
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
        crate::operation::delete_notification::DeleteNotificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_notification::DeleteNotificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_notification::DeleteNotification::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_notification::DeleteNotification::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_notification::DeleteNotificationOutput,
        crate::operation::delete_notification::DeleteNotificationError,
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
    /// <p>The <code>accountId</code> that is associated with the budget whose notification you want to delete.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The <code>accountId</code> that is associated with the budget whose notification you want to delete.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The <code>accountId</code> that is associated with the budget whose notification you want to delete.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
    /// <p>The name of the budget whose notification you want to delete.</p>
    pub fn budget_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.budget_name(input.into());
        self
    }
    /// <p>The name of the budget whose notification you want to delete.</p>
    pub fn set_budget_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_budget_name(input);
        self
    }
    /// <p>The name of the budget whose notification you want to delete.</p>
    pub fn get_budget_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_budget_name()
    }
    /// <p>The notification that you want to delete.</p>
    pub fn notification(mut self, input: crate::types::Notification) -> Self {
        self.inner = self.inner.notification(input);
        self
    }
    /// <p>The notification that you want to delete.</p>
    pub fn set_notification(mut self, input: ::std::option::Option<crate::types::Notification>) -> Self {
        self.inner = self.inner.set_notification(input);
        self
    }
    /// <p>The notification that you want to delete.</p>
    pub fn get_notification(&self) -> &::std::option::Option<crate::types::Notification> {
        self.inner.get_notification()
    }
}
