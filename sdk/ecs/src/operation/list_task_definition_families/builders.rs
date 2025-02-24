// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_task_definition_families::_list_task_definition_families_output::ListTaskDefinitionFamiliesOutputBuilder;

pub use crate::operation::list_task_definition_families::_list_task_definition_families_input::ListTaskDefinitionFamiliesInputBuilder;

impl ListTaskDefinitionFamiliesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_task_definition_families();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListTaskDefinitionFamilies`.
///
/// <p>Returns a list of task definition families that are registered to your account. This list includes task definition families that no longer have any <code>ACTIVE</code> task definition revisions.</p>
/// <p>You can filter out task definition families that don't contain any <code>ACTIVE</code> task definition revisions by setting the <code>status</code> parameter to <code>ACTIVE</code>. You can also filter the results with the <code>familyPrefix</code> parameter.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTaskDefinitionFamiliesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_task_definition_families::builders::ListTaskDefinitionFamiliesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesOutput,
        crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesError,
    > for ListTaskDefinitionFamiliesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesOutput,
            crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListTaskDefinitionFamiliesFluentBuilder {
    /// Creates a new `ListTaskDefinitionFamilies`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListTaskDefinitionFamilies as a reference.
    pub fn as_input(&self) -> &crate::operation::list_task_definition_families::builders::ListTaskDefinitionFamiliesInputBuilder {
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
        crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_task_definition_families::ListTaskDefinitionFamilies::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_task_definition_families::ListTaskDefinitionFamilies::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesOutput,
        crate::operation::list_task_definition_families::ListTaskDefinitionFamiliesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_task_definition_families::paginator::ListTaskDefinitionFamiliesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_task_definition_families::paginator::ListTaskDefinitionFamiliesPaginator {
        crate::operation::list_task_definition_families::paginator::ListTaskDefinitionFamiliesPaginator::new(self.handle, self.inner)
    }
    /// <p>The <code>familyPrefix</code> is a string that's used to filter the results of <code>ListTaskDefinitionFamilies</code>. If you specify a <code>familyPrefix</code>, only task definition family names that begin with the <code>familyPrefix</code> string are returned.</p>
    pub fn family_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.family_prefix(input.into());
        self
    }
    /// <p>The <code>familyPrefix</code> is a string that's used to filter the results of <code>ListTaskDefinitionFamilies</code>. If you specify a <code>familyPrefix</code>, only task definition family names that begin with the <code>familyPrefix</code> string are returned.</p>
    pub fn set_family_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_family_prefix(input);
        self
    }
    /// <p>The <code>familyPrefix</code> is a string that's used to filter the results of <code>ListTaskDefinitionFamilies</code>. If you specify a <code>familyPrefix</code>, only task definition family names that begin with the <code>familyPrefix</code> string are returned.</p>
    pub fn get_family_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_family_prefix()
    }
    /// <p>The task definition family status to filter the <code>ListTaskDefinitionFamilies</code> results with. By default, both <code>ACTIVE</code> and <code>INACTIVE</code> task definition families are listed. If this parameter is set to <code>ACTIVE</code>, only task definition families that have an <code>ACTIVE</code> task definition revision are returned. If this parameter is set to <code>INACTIVE</code>, only task definition families that do not have any <code>ACTIVE</code> task definition revisions are returned. If you paginate the resulting output, be sure to keep the <code>status</code> value constant in each subsequent request.</p>
    pub fn status(mut self, input: crate::types::TaskDefinitionFamilyStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>The task definition family status to filter the <code>ListTaskDefinitionFamilies</code> results with. By default, both <code>ACTIVE</code> and <code>INACTIVE</code> task definition families are listed. If this parameter is set to <code>ACTIVE</code>, only task definition families that have an <code>ACTIVE</code> task definition revision are returned. If this parameter is set to <code>INACTIVE</code>, only task definition families that do not have any <code>ACTIVE</code> task definition revisions are returned. If you paginate the resulting output, be sure to keep the <code>status</code> value constant in each subsequent request.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::TaskDefinitionFamilyStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>The task definition family status to filter the <code>ListTaskDefinitionFamilies</code> results with. By default, both <code>ACTIVE</code> and <code>INACTIVE</code> task definition families are listed. If this parameter is set to <code>ACTIVE</code>, only task definition families that have an <code>ACTIVE</code> task definition revision are returned. If this parameter is set to <code>INACTIVE</code>, only task definition families that do not have any <code>ACTIVE</code> task definition revisions are returned. If you paginate the resulting output, be sure to keep the <code>status</code> value constant in each subsequent request.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::TaskDefinitionFamilyStatus> {
        self.inner.get_status()
    }
    /// <p>The <code>nextToken</code> value returned from a <code>ListTaskDefinitionFamilies</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it is possible the number of results to be fewer than <code>maxResults</code>.</p><note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> value returned from a <code>ListTaskDefinitionFamilies</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it is possible the number of results to be fewer than <code>maxResults</code>.</p><note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The <code>nextToken</code> value returned from a <code>ListTaskDefinitionFamilies</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it is possible the number of results to be fewer than <code>maxResults</code>.</p><note>
    /// <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of task definition family results that <code>ListTaskDefinitionFamilies</code> returned in paginated output. When this parameter is used, <code>ListTaskDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTaskDefinitionFamilies</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListTaskDefinitionFamilies</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of task definition family results that <code>ListTaskDefinitionFamilies</code> returned in paginated output. When this parameter is used, <code>ListTaskDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTaskDefinitionFamilies</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListTaskDefinitionFamilies</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of task definition family results that <code>ListTaskDefinitionFamilies</code> returned in paginated output. When this parameter is used, <code>ListTaskDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTaskDefinitionFamilies</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListTaskDefinitionFamilies</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
