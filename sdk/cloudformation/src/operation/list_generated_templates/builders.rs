// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_generated_templates::_list_generated_templates_output::ListGeneratedTemplatesOutputBuilder;

pub use crate::operation::list_generated_templates::_list_generated_templates_input::ListGeneratedTemplatesInputBuilder;

impl ListGeneratedTemplatesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_generated_templates::ListGeneratedTemplatesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_generated_templates::ListGeneratedTemplatesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_generated_templates();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListGeneratedTemplates`.
///
/// <p>Lists your generated templates in this Region.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListGeneratedTemplatesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_generated_templates::builders::ListGeneratedTemplatesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_generated_templates::ListGeneratedTemplatesOutput,
        crate::operation::list_generated_templates::ListGeneratedTemplatesError,
    > for ListGeneratedTemplatesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_generated_templates::ListGeneratedTemplatesOutput,
            crate::operation::list_generated_templates::ListGeneratedTemplatesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListGeneratedTemplatesFluentBuilder {
    /// Creates a new `ListGeneratedTemplates`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListGeneratedTemplates as a reference.
    pub fn as_input(&self) -> &crate::operation::list_generated_templates::builders::ListGeneratedTemplatesInputBuilder {
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
        crate::operation::list_generated_templates::ListGeneratedTemplatesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_generated_templates::ListGeneratedTemplatesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_generated_templates::ListGeneratedTemplates::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_generated_templates::ListGeneratedTemplates::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_generated_templates::ListGeneratedTemplatesOutput,
        crate::operation::list_generated_templates::ListGeneratedTemplatesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_generated_templates::paginator::ListGeneratedTemplatesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_generated_templates::paginator::ListGeneratedTemplatesPaginator {
        crate::operation::list_generated_templates::paginator::ListGeneratedTemplatesPaginator::new(self.handle, self.inner)
    }
    /// <p>A string that identifies the next page of resource scan results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A string that identifies the next page of resource scan results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A string that identifies the next page of resource scan results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can use for the <code>NextToken</code> parameter to get the next set of results. By default the <code>ListGeneratedTemplates</code> API action will return at most 50 results in each response. The maximum value is 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can use for the <code>NextToken</code> parameter to get the next set of results. By default the <code>ListGeneratedTemplates</code> API action will return at most 50 results in each response. The maximum value is 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can use for the <code>NextToken</code> parameter to get the next set of results. By default the <code>ListGeneratedTemplates</code> API action will return at most 50 results in each response. The maximum value is 100.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
