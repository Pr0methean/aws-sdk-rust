// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_delete_events_by_event_type_status::_get_delete_events_by_event_type_status_output::GetDeleteEventsByEventTypeStatusOutputBuilder;

pub use crate::operation::get_delete_events_by_event_type_status::_get_delete_events_by_event_type_status_input::GetDeleteEventsByEventTypeStatusInputBuilder;

impl GetDeleteEventsByEventTypeStatusInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_delete_events_by_event_type_status();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetDeleteEventsByEventTypeStatus`.
///
/// <p>Retrieves the status of a <code>DeleteEventsByEventType</code> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetDeleteEventsByEventTypeStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_delete_events_by_event_type_status::builders::GetDeleteEventsByEventTypeStatusInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatusOutput,
        crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatusError,
    > for GetDeleteEventsByEventTypeStatusFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatusOutput,
            crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatusError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetDeleteEventsByEventTypeStatusFluentBuilder {
    /// Creates a new `GetDeleteEventsByEventTypeStatus`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetDeleteEventsByEventTypeStatus as a reference.
    pub fn as_input(&self) -> &crate::operation::get_delete_events_by_event_type_status::builders::GetDeleteEventsByEventTypeStatusInputBuilder {
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
        crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatus::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatus::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatusOutput,
        crate::operation::get_delete_events_by_event_type_status::GetDeleteEventsByEventTypeStatusError,
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
    /// <p>Name of event type for which to get the deletion status.</p>
    pub fn event_type_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_type_name(input.into());
        self
    }
    /// <p>Name of event type for which to get the deletion status.</p>
    pub fn set_event_type_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_event_type_name(input);
        self
    }
    /// <p>Name of event type for which to get the deletion status.</p>
    pub fn get_event_type_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_event_type_name()
    }
}
