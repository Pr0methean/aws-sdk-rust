// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_jobs::_list_jobs_output::ListJobsOutputBuilder;

pub use crate::operation::list_jobs::_list_jobs_input::ListJobsInputBuilder;

impl ListJobsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_jobs::ListJobsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_jobs::ListJobsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_jobs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListJobs`.
///
/// Retrieve a JSON array of up to twenty of your most recently created jobs. This array includes in-process, completed, and errored jobs. This will return the jobs themselves, not just a list of the jobs. To retrieve the twenty next most recent jobs, use the nextToken string returned with the array.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListJobsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_jobs::builders::ListJobsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::list_jobs::ListJobsOutput, crate::operation::list_jobs::ListJobsError>
    for ListJobsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::list_jobs::ListJobsOutput, crate::operation::list_jobs::ListJobsError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListJobsFluentBuilder {
    /// Creates a new `ListJobs`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListJobs as a reference.
    pub fn as_input(&self) -> &crate::operation::list_jobs::builders::ListJobsInputBuilder {
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
        crate::operation::list_jobs::ListJobsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_jobs::ListJobsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_jobs::ListJobs::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_jobs::ListJobs::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::list_jobs::ListJobsOutput, crate::operation::list_jobs::ListJobsError, Self>
    {
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_jobs::paginator::ListJobsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_jobs::paginator::ListJobsPaginator {
        crate::operation::list_jobs::paginator::ListJobsPaginator::new(self.handle, self.inner)
    }
    /// Optional. Number of jobs, up to twenty, that will be returned at one time.
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// Optional. Number of jobs, up to twenty, that will be returned at one time.
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// Optional. Number of jobs, up to twenty, that will be returned at one time.
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// Optional. Use this string, provided with the response to a previous request, to request the next batch of jobs.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// Optional. Use this string, provided with the response to a previous request, to request the next batch of jobs.
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// Optional. Use this string, provided with the response to a previous request, to request the next batch of jobs.
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// Optional. When you request lists of resources, you can specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.
    pub fn order(mut self, input: crate::types::Order) -> Self {
        self.inner = self.inner.order(input);
        self
    }
    /// Optional. When you request lists of resources, you can specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.
    pub fn set_order(mut self, input: ::std::option::Option<crate::types::Order>) -> Self {
        self.inner = self.inner.set_order(input);
        self
    }
    /// Optional. When you request lists of resources, you can specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.
    pub fn get_order(&self) -> &::std::option::Option<crate::types::Order> {
        self.inner.get_order()
    }
    /// Optional. Provide a queue name to get back only jobs from that queue.
    pub fn queue(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue(input.into());
        self
    }
    /// Optional. Provide a queue name to get back only jobs from that queue.
    pub fn set_queue(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue(input);
        self
    }
    /// Optional. Provide a queue name to get back only jobs from that queue.
    pub fn get_queue(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue()
    }
    /// Optional. A job's status can be SUBMITTED, PROGRESSING, COMPLETE, CANCELED, or ERROR.
    pub fn status(mut self, input: crate::types::JobStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// Optional. A job's status can be SUBMITTED, PROGRESSING, COMPLETE, CANCELED, or ERROR.
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::JobStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// Optional. A job's status can be SUBMITTED, PROGRESSING, COMPLETE, CANCELED, or ERROR.
    pub fn get_status(&self) -> &::std::option::Option<crate::types::JobStatus> {
        self.inner.get_status()
    }
}
