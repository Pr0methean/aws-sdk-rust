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
/// <p>Lists jobs.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListJobs</a> action.</p>
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
    /// <p>An optional filter that lets you search for jobs that have the specified status.</p>
    pub fn status(mut self, input: crate::types::JobStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>An optional filter that lets you search for jobs that have the specified status.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::JobStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>An optional filter that lets you search for jobs that have the specified status.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::JobStatus> {
        self.inner.get_status()
    }
    /// <p>Specifies whether the job will continue to run (CONTINUOUS), or will be complete after all those things specified as targets have completed the job (SNAPSHOT). If continuous, the job may also be run on a thing when a change is detected in a target. For example, a job will run on a thing when the thing is added to a target group, even after the job was completed by all things originally in the group.</p><note>
    /// <p>We recommend that you use continuous jobs instead of snapshot jobs for dynamic thing group targets. By using continuous jobs, devices that join the group receive the job execution even after the job has been created.</p>
    /// </note>
    pub fn target_selection(mut self, input: crate::types::TargetSelection) -> Self {
        self.inner = self.inner.target_selection(input);
        self
    }
    /// <p>Specifies whether the job will continue to run (CONTINUOUS), or will be complete after all those things specified as targets have completed the job (SNAPSHOT). If continuous, the job may also be run on a thing when a change is detected in a target. For example, a job will run on a thing when the thing is added to a target group, even after the job was completed by all things originally in the group.</p><note>
    /// <p>We recommend that you use continuous jobs instead of snapshot jobs for dynamic thing group targets. By using continuous jobs, devices that join the group receive the job execution even after the job has been created.</p>
    /// </note>
    pub fn set_target_selection(mut self, input: ::std::option::Option<crate::types::TargetSelection>) -> Self {
        self.inner = self.inner.set_target_selection(input);
        self
    }
    /// <p>Specifies whether the job will continue to run (CONTINUOUS), or will be complete after all those things specified as targets have completed the job (SNAPSHOT). If continuous, the job may also be run on a thing when a change is detected in a target. For example, a job will run on a thing when the thing is added to a target group, even after the job was completed by all things originally in the group.</p><note>
    /// <p>We recommend that you use continuous jobs instead of snapshot jobs for dynamic thing group targets. By using continuous jobs, devices that join the group receive the job execution even after the job has been created.</p>
    /// </note>
    pub fn get_target_selection(&self) -> &::std::option::Option<crate::types::TargetSelection> {
        self.inner.get_target_selection()
    }
    /// <p>The maximum number of results to return per request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return per request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return per request.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>A filter that limits the returned jobs to those for the specified group.</p>
    pub fn thing_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.thing_group_name(input.into());
        self
    }
    /// <p>A filter that limits the returned jobs to those for the specified group.</p>
    pub fn set_thing_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_thing_group_name(input);
        self
    }
    /// <p>A filter that limits the returned jobs to those for the specified group.</p>
    pub fn get_thing_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_thing_group_name()
    }
    /// <p>A filter that limits the returned jobs to those for the specified group.</p>
    pub fn thing_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.thing_group_id(input.into());
        self
    }
    /// <p>A filter that limits the returned jobs to those for the specified group.</p>
    pub fn set_thing_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_thing_group_id(input);
        self
    }
    /// <p>A filter that limits the returned jobs to those for the specified group.</p>
    pub fn get_thing_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_thing_group_id()
    }
    /// <p>The namespace used to indicate that a job is a customer-managed job.</p>
    /// <p>When you specify a value for this parameter, Amazon Web Services IoT Core sends jobs notifications to MQTT topics that contain the value in the following format.</p>
    /// <p><code>$aws/things/<i>THING_NAME</i>/jobs/<i>JOB_ID</i>/notify-namespace-<i>NAMESPACE_ID</i>/</code></p><note>
    /// <p>The <code>namespaceId</code> feature is only supported by IoT Greengrass at this time. For more information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/setting-up.html">Setting up IoT Greengrass core devices.</a></p>
    /// </note>
    pub fn namespace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.namespace_id(input.into());
        self
    }
    /// <p>The namespace used to indicate that a job is a customer-managed job.</p>
    /// <p>When you specify a value for this parameter, Amazon Web Services IoT Core sends jobs notifications to MQTT topics that contain the value in the following format.</p>
    /// <p><code>$aws/things/<i>THING_NAME</i>/jobs/<i>JOB_ID</i>/notify-namespace-<i>NAMESPACE_ID</i>/</code></p><note>
    /// <p>The <code>namespaceId</code> feature is only supported by IoT Greengrass at this time. For more information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/setting-up.html">Setting up IoT Greengrass core devices.</a></p>
    /// </note>
    pub fn set_namespace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_namespace_id(input);
        self
    }
    /// <p>The namespace used to indicate that a job is a customer-managed job.</p>
    /// <p>When you specify a value for this parameter, Amazon Web Services IoT Core sends jobs notifications to MQTT topics that contain the value in the following format.</p>
    /// <p><code>$aws/things/<i>THING_NAME</i>/jobs/<i>JOB_ID</i>/notify-namespace-<i>NAMESPACE_ID</i>/</code></p><note>
    /// <p>The <code>namespaceId</code> feature is only supported by IoT Greengrass at this time. For more information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/setting-up.html">Setting up IoT Greengrass core devices.</a></p>
    /// </note>
    pub fn get_namespace_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_namespace_id()
    }
}
