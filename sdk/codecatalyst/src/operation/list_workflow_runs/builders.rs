// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_workflow_runs::_list_workflow_runs_output::ListWorkflowRunsOutputBuilder;

pub use crate::operation::list_workflow_runs::_list_workflow_runs_input::ListWorkflowRunsInputBuilder;

impl ListWorkflowRunsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_workflow_runs::ListWorkflowRunsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_workflow_runs::ListWorkflowRunsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_workflow_runs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListWorkflowRuns`.
///
/// <p>Retrieves a list of workflow runs of a specified workflow.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListWorkflowRunsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_workflow_runs::builders::ListWorkflowRunsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_workflow_runs::ListWorkflowRunsOutput,
        crate::operation::list_workflow_runs::ListWorkflowRunsError,
    > for ListWorkflowRunsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_workflow_runs::ListWorkflowRunsOutput,
            crate::operation::list_workflow_runs::ListWorkflowRunsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListWorkflowRunsFluentBuilder {
    /// Creates a new `ListWorkflowRuns`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListWorkflowRuns as a reference.
    pub fn as_input(&self) -> &crate::operation::list_workflow_runs::builders::ListWorkflowRunsInputBuilder {
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
        crate::operation::list_workflow_runs::ListWorkflowRunsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_workflow_runs::ListWorkflowRunsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_workflow_runs::ListWorkflowRuns::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_workflow_runs::ListWorkflowRuns::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_workflow_runs::ListWorkflowRunsOutput,
        crate::operation::list_workflow_runs::ListWorkflowRunsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_workflow_runs::paginator::ListWorkflowRunsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_workflow_runs::paginator::ListWorkflowRunsPaginator {
        crate::operation::list_workflow_runs::paginator::ListWorkflowRunsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the space.</p>
    pub fn space_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.space_name(input.into());
        self
    }
    /// <p>The name of the space.</p>
    pub fn set_space_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_space_name(input);
        self
    }
    /// <p>The name of the space.</p>
    pub fn get_space_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_space_name()
    }
    /// <p>The ID of the workflow. To retrieve a list of workflow IDs, use <code>ListWorkflows</code>.</p>
    pub fn workflow_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workflow_id(input.into());
        self
    }
    /// <p>The ID of the workflow. To retrieve a list of workflow IDs, use <code>ListWorkflows</code>.</p>
    pub fn set_workflow_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workflow_id(input);
        self
    }
    /// <p>The ID of the workflow. To retrieve a list of workflow IDs, use <code>ListWorkflows</code>.</p>
    pub fn get_workflow_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workflow_id()
    }
    /// <p>The name of the project in the space.</p>
    pub fn project_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_name(input.into());
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn set_project_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_name(input);
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn get_project_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_name()
    }
    /// <p>A token returned from a call to this API to indicate the next batch of results to return, if any.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token returned from a call to this API to indicate the next batch of results to return, if any.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token returned from a call to this API to indicate the next batch of results to return, if any.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to show in a single call to this API. If the number of results is larger than the number you specified, the response will include a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to show in a single call to this API. If the number of results is larger than the number you specified, the response will include a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to show in a single call to this API. If the number of results is larger than the number you specified, the response will include a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// Appends an item to `sortBy`.
    ///
    /// To override the contents of this collection use [`set_sort_by`](Self::set_sort_by).
    ///
    /// <p>Information used to sort the items in the returned list.</p>
    pub fn sort_by(mut self, input: crate::types::WorkflowRunSortCriteria) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>Information used to sort the items in the returned list.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::WorkflowRunSortCriteria>>) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>Information used to sort the items in the returned list.</p>
    pub fn get_sort_by(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::WorkflowRunSortCriteria>> {
        self.inner.get_sort_by()
    }
}
