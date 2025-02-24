// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_projects::_list_projects_output::ListProjectsOutputBuilder;

pub use crate::operation::list_projects::_list_projects_input::ListProjectsInputBuilder;

impl ListProjectsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_projects::ListProjectsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_projects::ListProjectsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_projects();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListProjects`.
///
/// <p>Gets a list of build project names, with each build project name representing a single build project.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListProjectsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_projects::builders::ListProjectsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_projects::ListProjectsOutput,
        crate::operation::list_projects::ListProjectsError,
    > for ListProjectsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_projects::ListProjectsOutput,
            crate::operation::list_projects::ListProjectsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListProjectsFluentBuilder {
    /// Creates a new `ListProjects`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListProjects as a reference.
    pub fn as_input(&self) -> &crate::operation::list_projects::builders::ListProjectsInputBuilder {
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
        crate::operation::list_projects::ListProjectsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_projects::ListProjectsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_projects::ListProjects::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_projects::ListProjects::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_projects::ListProjectsOutput,
        crate::operation::list_projects::ListProjectsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_projects::paginator::ListProjectsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_projects::paginator::ListProjectsPaginator {
        crate::operation::list_projects::paginator::ListProjectsPaginator::new(self.handle, self.inner)
    }
    /// <p>The criterion to be used to list build project names. Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>CREATED_TIME</code>: List based on when each build project was created.</p></li>
    /// <li>
    /// <p><code>LAST_MODIFIED_TIME</code>: List based on when information about each build project was last changed.</p></li>
    /// <li>
    /// <p><code>NAME</code>: List based on each build project's name.</p></li>
    /// </ul>
    /// <p>Use <code>sortOrder</code> to specify in what order to list the build project names based on the preceding criteria.</p>
    pub fn sort_by(mut self, input: crate::types::ProjectSortByType) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>The criterion to be used to list build project names. Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>CREATED_TIME</code>: List based on when each build project was created.</p></li>
    /// <li>
    /// <p><code>LAST_MODIFIED_TIME</code>: List based on when information about each build project was last changed.</p></li>
    /// <li>
    /// <p><code>NAME</code>: List based on each build project's name.</p></li>
    /// </ul>
    /// <p>Use <code>sortOrder</code> to specify in what order to list the build project names based on the preceding criteria.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<crate::types::ProjectSortByType>) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>The criterion to be used to list build project names. Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>CREATED_TIME</code>: List based on when each build project was created.</p></li>
    /// <li>
    /// <p><code>LAST_MODIFIED_TIME</code>: List based on when information about each build project was last changed.</p></li>
    /// <li>
    /// <p><code>NAME</code>: List based on each build project's name.</p></li>
    /// </ul>
    /// <p>Use <code>sortOrder</code> to specify in what order to list the build project names based on the preceding criteria.</p>
    pub fn get_sort_by(&self) -> &::std::option::Option<crate::types::ProjectSortByType> {
        self.inner.get_sort_by()
    }
    /// <p>The order in which to list build projects. Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>ASCENDING</code>: List in ascending order.</p></li>
    /// <li>
    /// <p><code>DESCENDING</code>: List in descending order.</p></li>
    /// </ul>
    /// <p>Use <code>sortBy</code> to specify the criterion to be used to list build project names.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrderType) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>The order in which to list build projects. Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>ASCENDING</code>: List in ascending order.</p></li>
    /// <li>
    /// <p><code>DESCENDING</code>: List in descending order.</p></li>
    /// </ul>
    /// <p>Use <code>sortBy</code> to specify the criterion to be used to list build project names.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrderType>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>The order in which to list build projects. Valid values include:</p>
    /// <ul>
    /// <li>
    /// <p><code>ASCENDING</code>: List in ascending order.</p></li>
    /// <li>
    /// <p><code>DESCENDING</code>: List in descending order.</p></li>
    /// </ul>
    /// <p>Use <code>sortBy</code> to specify the criterion to be used to list build project names.</p>
    pub fn get_sort_order(&self) -> &::std::option::Option<crate::types::SortOrderType> {
        self.inner.get_sort_order()
    }
    /// <p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>nextToken</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>nextToken</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>nextToken</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
