// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_project_memberships::_list_project_memberships_output::ListProjectMembershipsOutputBuilder;

pub use crate::operation::list_project_memberships::_list_project_memberships_input::ListProjectMembershipsInputBuilder;

impl ListProjectMembershipsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_project_memberships::ListProjectMembershipsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_project_memberships::ListProjectMembershipsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_project_memberships();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListProjectMemberships`.
///
/// <p>Lists all members of the specified project.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListProjectMembershipsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_project_memberships::builders::ListProjectMembershipsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_project_memberships::ListProjectMembershipsOutput,
        crate::operation::list_project_memberships::ListProjectMembershipsError,
    > for ListProjectMembershipsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_project_memberships::ListProjectMembershipsOutput,
            crate::operation::list_project_memberships::ListProjectMembershipsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListProjectMembershipsFluentBuilder {
    /// Creates a new `ListProjectMemberships`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListProjectMemberships as a reference.
    pub fn as_input(&self) -> &crate::operation::list_project_memberships::builders::ListProjectMembershipsInputBuilder {
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
        crate::operation::list_project_memberships::ListProjectMembershipsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_project_memberships::ListProjectMembershipsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_project_memberships::ListProjectMemberships::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_project_memberships::ListProjectMemberships::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_project_memberships::ListProjectMembershipsOutput,
        crate::operation::list_project_memberships::ListProjectMembershipsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_project_memberships::paginator::ListProjectMembershipsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_project_memberships::paginator::ListProjectMembershipsPaginator {
        crate::operation::list_project_memberships::paginator::ListProjectMembershipsPaginator::new(self.handle, self.inner)
    }
    /// <p>The identifier of the Amazon DataZone domain in which you want to list project memberships.</p>
    pub fn domain_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_identifier(input.into());
        self
    }
    /// <p>The identifier of the Amazon DataZone domain in which you want to list project memberships.</p>
    pub fn set_domain_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_identifier(input);
        self
    }
    /// <p>The identifier of the Amazon DataZone domain in which you want to list project memberships.</p>
    pub fn get_domain_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_identifier()
    }
    /// <p>The identifier of the project whose memberships you want to list.</p>
    pub fn project_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_identifier(input.into());
        self
    }
    /// <p>The identifier of the project whose memberships you want to list.</p>
    pub fn set_project_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_identifier(input);
        self
    }
    /// <p>The identifier of the project whose memberships you want to list.</p>
    pub fn get_project_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_identifier()
    }
    /// <p>The method by which you want to sort the project memberships.</p>
    pub fn sort_by(mut self, input: crate::types::SortFieldProject) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>The method by which you want to sort the project memberships.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<crate::types::SortFieldProject>) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>The method by which you want to sort the project memberships.</p>
    pub fn get_sort_by(&self) -> &::std::option::Option<crate::types::SortFieldProject> {
        self.inner.get_sort_by()
    }
    /// <p>The sort order of the project memberships.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>The sort order of the project memberships.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>The sort order of the project memberships.</p>
    pub fn get_sort_order(&self) -> &::std::option::Option<crate::types::SortOrder> {
        self.inner.get_sort_order()
    }
    /// <p>When the number of memberships is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of memberships, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListProjectMemberships</code> to list the next set of memberships.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>When the number of memberships is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of memberships, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListProjectMemberships</code> to list the next set of memberships.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>When the number of memberships is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of memberships, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListProjectMemberships</code> to list the next set of memberships.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of memberships to return in a single call to <code>ListProjectMemberships</code>. When the number of memberships to be listed is greater than the value of <code>MaxResults</code>, the response contains a <code>NextToken</code> value that you can use in a subsequent call to <code>ListProjectMemberships</code> to list the next set of memberships.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of memberships to return in a single call to <code>ListProjectMemberships</code>. When the number of memberships to be listed is greater than the value of <code>MaxResults</code>, the response contains a <code>NextToken</code> value that you can use in a subsequent call to <code>ListProjectMemberships</code> to list the next set of memberships.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of memberships to return in a single call to <code>ListProjectMemberships</code>. When the number of memberships to be listed is greater than the value of <code>MaxResults</code>, the response contains a <code>NextToken</code> value that you can use in a subsequent call to <code>ListProjectMemberships</code> to list the next set of memberships.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
