// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_flow_executions::_search_flow_executions_output::SearchFlowExecutionsOutputBuilder;

pub use crate::operation::search_flow_executions::_search_flow_executions_input::SearchFlowExecutionsInputBuilder;

impl SearchFlowExecutionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::search_flow_executions::SearchFlowExecutionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_flow_executions::SearchFlowExecutionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.search_flow_executions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SearchFlowExecutions`.
///
/// <p>Searches for AWS IoT Things Graph workflow execution instances.</p>
#[deprecated(note = "since: 2022-08-30")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SearchFlowExecutionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_flow_executions::builders::SearchFlowExecutionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::search_flow_executions::SearchFlowExecutionsOutput,
        crate::operation::search_flow_executions::SearchFlowExecutionsError,
    > for SearchFlowExecutionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::search_flow_executions::SearchFlowExecutionsOutput,
            crate::operation::search_flow_executions::SearchFlowExecutionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SearchFlowExecutionsFluentBuilder {
    /// Creates a new `SearchFlowExecutions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SearchFlowExecutions as a reference.
    pub fn as_input(&self) -> &crate::operation::search_flow_executions::builders::SearchFlowExecutionsInputBuilder {
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
        crate::operation::search_flow_executions::SearchFlowExecutionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_flow_executions::SearchFlowExecutionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::search_flow_executions::SearchFlowExecutions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::search_flow_executions::SearchFlowExecutions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::search_flow_executions::SearchFlowExecutionsOutput,
        crate::operation::search_flow_executions::SearchFlowExecutionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::search_flow_executions::paginator::SearchFlowExecutionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::search_flow_executions::paginator::SearchFlowExecutionsPaginator {
        crate::operation::search_flow_executions::paginator::SearchFlowExecutionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the system instance that contains the flow.</p>
    pub fn system_instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.system_instance_id(input.into());
        self
    }
    /// <p>The ID of the system instance that contains the flow.</p>
    pub fn set_system_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_system_instance_id(input);
        self
    }
    /// <p>The ID of the system instance that contains the flow.</p>
    pub fn get_system_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_system_instance_id()
    }
    /// <p>The ID of a flow execution.</p>
    pub fn flow_execution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.flow_execution_id(input.into());
        self
    }
    /// <p>The ID of a flow execution.</p>
    pub fn set_flow_execution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_flow_execution_id(input);
        self
    }
    /// <p>The ID of a flow execution.</p>
    pub fn get_flow_execution_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_flow_execution_id()
    }
    /// <p>The date and time of the earliest flow execution to return.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The date and time of the earliest flow execution to return.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The date and time of the earliest flow execution to return.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>The date and time of the latest flow execution to return.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The date and time of the latest flow execution to return.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The date and time of the latest flow execution to return.</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The string that specifies the next page of results. Use this when you're paginating results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
