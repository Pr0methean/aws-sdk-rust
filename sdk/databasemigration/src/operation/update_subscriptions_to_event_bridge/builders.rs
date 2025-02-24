// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_subscriptions_to_event_bridge::_update_subscriptions_to_event_bridge_output::UpdateSubscriptionsToEventBridgeOutputBuilder;

pub use crate::operation::update_subscriptions_to_event_bridge::_update_subscriptions_to_event_bridge_input::UpdateSubscriptionsToEventBridgeInputBuilder;

impl UpdateSubscriptionsToEventBridgeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridgeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridgeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_subscriptions_to_event_bridge();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateSubscriptionsToEventBridge`.
///
/// <p>Migrates 10 active and enabled Amazon SNS subscriptions at a time and converts them to corresponding Amazon EventBridge rules. By default, this operation migrates subscriptions only when all your replication instance versions are 3.4.5 or higher. If any replication instances are from versions earlier than 3.4.5, the operation raises an error and tells you to upgrade these instances to version 3.4.5 or higher. To enable migration regardless of version, set the <code>Force</code> option to true. However, if you don't upgrade instances earlier than version 3.4.5, some types of events might not be available when you use Amazon EventBridge.</p>
/// <p>To call this operation, make sure that you have certain permissions added to your user account. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html#CHAP_Events-migrate-to-eventbridge">Migrating event subscriptions to Amazon EventBridge</a> in the <i>Amazon Web Services Database Migration Service User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSubscriptionsToEventBridgeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_subscriptions_to_event_bridge::builders::UpdateSubscriptionsToEventBridgeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridgeOutput,
        crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridgeError,
    > for UpdateSubscriptionsToEventBridgeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridgeOutput,
            crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridgeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateSubscriptionsToEventBridgeFluentBuilder {
    /// Creates a new `UpdateSubscriptionsToEventBridge`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateSubscriptionsToEventBridge as a reference.
    pub fn as_input(&self) -> &crate::operation::update_subscriptions_to_event_bridge::builders::UpdateSubscriptionsToEventBridgeInputBuilder {
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
        crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridgeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridgeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridge::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridge::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridgeOutput,
        crate::operation::update_subscriptions_to_event_bridge::UpdateSubscriptionsToEventBridgeError,
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
    /// <p>When set to true, this operation migrates DMS subscriptions for Amazon SNS notifications no matter what your replication instance version is. If not set or set to false, this operation runs only when all your replication instances are from DMS version 3.4.5 or higher.</p>
    pub fn force_move(mut self, input: bool) -> Self {
        self.inner = self.inner.force_move(input);
        self
    }
    /// <p>When set to true, this operation migrates DMS subscriptions for Amazon SNS notifications no matter what your replication instance version is. If not set or set to false, this operation runs only when all your replication instances are from DMS version 3.4.5 or higher.</p>
    pub fn set_force_move(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force_move(input);
        self
    }
    /// <p>When set to true, this operation migrates DMS subscriptions for Amazon SNS notifications no matter what your replication instance version is. If not set or set to false, this operation runs only when all your replication instances are from DMS version 3.4.5 or higher.</p>
    pub fn get_force_move(&self) -> &::std::option::Option<bool> {
        self.inner.get_force_move()
    }
}
