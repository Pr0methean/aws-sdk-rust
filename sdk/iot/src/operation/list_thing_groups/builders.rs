// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_thing_groups::_list_thing_groups_output::ListThingGroupsOutputBuilder;

pub use crate::operation::list_thing_groups::_list_thing_groups_input::ListThingGroupsInputBuilder;

impl ListThingGroupsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_thing_groups::ListThingGroupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_thing_groups::ListThingGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_thing_groups();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListThingGroups`.
///
/// <p>List the thing groups in your account.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListThingGroups</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListThingGroupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_thing_groups::builders::ListThingGroupsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_thing_groups::ListThingGroupsOutput,
        crate::operation::list_thing_groups::ListThingGroupsError,
    > for ListThingGroupsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_thing_groups::ListThingGroupsOutput,
            crate::operation::list_thing_groups::ListThingGroupsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListThingGroupsFluentBuilder {
    /// Creates a new `ListThingGroups`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListThingGroups as a reference.
    pub fn as_input(&self) -> &crate::operation::list_thing_groups::builders::ListThingGroupsInputBuilder {
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
        crate::operation::list_thing_groups::ListThingGroupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_thing_groups::ListThingGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_thing_groups::ListThingGroups::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_thing_groups::ListThingGroups::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_thing_groups::ListThingGroupsOutput,
        crate::operation::list_thing_groups::ListThingGroupsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_thing_groups::paginator::ListThingGroupsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_thing_groups::paginator::ListThingGroupsPaginator {
        crate::operation::list_thing_groups::paginator::ListThingGroupsPaginator::new(self.handle, self.inner)
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A filter that limits the results to those with the specified parent group.</p>
    pub fn parent_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.parent_group(input.into());
        self
    }
    /// <p>A filter that limits the results to those with the specified parent group.</p>
    pub fn set_parent_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_parent_group(input);
        self
    }
    /// <p>A filter that limits the results to those with the specified parent group.</p>
    pub fn get_parent_group(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_parent_group()
    }
    /// <p>A filter that limits the results to those with the specified name prefix.</p>
    pub fn name_prefix_filter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name_prefix_filter(input.into());
        self
    }
    /// <p>A filter that limits the results to those with the specified name prefix.</p>
    pub fn set_name_prefix_filter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name_prefix_filter(input);
        self
    }
    /// <p>A filter that limits the results to those with the specified name prefix.</p>
    pub fn get_name_prefix_filter(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name_prefix_filter()
    }
    /// <p>If true, return child groups as well.</p>
    pub fn recursive(mut self, input: bool) -> Self {
        self.inner = self.inner.recursive(input);
        self
    }
    /// <p>If true, return child groups as well.</p>
    pub fn set_recursive(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_recursive(input);
        self
    }
    /// <p>If true, return child groups as well.</p>
    pub fn get_recursive(&self) -> &::std::option::Option<bool> {
        self.inner.get_recursive()
    }
}
