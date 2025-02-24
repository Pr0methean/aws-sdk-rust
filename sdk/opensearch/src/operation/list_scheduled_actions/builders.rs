// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_scheduled_actions::_list_scheduled_actions_output::ListScheduledActionsOutputBuilder;

pub use crate::operation::list_scheduled_actions::_list_scheduled_actions_input::ListScheduledActionsInputBuilder;

impl ListScheduledActionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_scheduled_actions::ListScheduledActionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_scheduled_actions::ListScheduledActionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_scheduled_actions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListScheduledActions`.
///
/// <p>Retrieves a list of configuration changes that are scheduled for a domain. These changes can be <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/service-software.html">service software updates</a> or <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/auto-tune.html#auto-tune-types">blue/green Auto-Tune enhancements</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListScheduledActionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_scheduled_actions::builders::ListScheduledActionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_scheduled_actions::ListScheduledActionsOutput,
        crate::operation::list_scheduled_actions::ListScheduledActionsError,
    > for ListScheduledActionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_scheduled_actions::ListScheduledActionsOutput,
            crate::operation::list_scheduled_actions::ListScheduledActionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListScheduledActionsFluentBuilder {
    /// Creates a new `ListScheduledActions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListScheduledActions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_scheduled_actions::builders::ListScheduledActionsInputBuilder {
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
        crate::operation::list_scheduled_actions::ListScheduledActionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_scheduled_actions::ListScheduledActionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_scheduled_actions::ListScheduledActions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_scheduled_actions::ListScheduledActions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_scheduled_actions::ListScheduledActionsOutput,
        crate::operation::list_scheduled_actions::ListScheduledActionsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_scheduled_actions::paginator::ListScheduledActionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_scheduled_actions::paginator::ListScheduledActionsPaginator {
        crate::operation::list_scheduled_actions::paginator::ListScheduledActionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the domain.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the domain.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of the domain.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>If your initial <code>ListScheduledActions</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListScheduledActions</code> operations, which returns results in the next page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If your initial <code>ListScheduledActions</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListScheduledActions</code> operations, which returns results in the next page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If your initial <code>ListScheduledActions</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListScheduledActions</code> operations, which returns results in the next page.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
