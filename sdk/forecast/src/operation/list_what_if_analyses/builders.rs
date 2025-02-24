// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_what_if_analyses::_list_what_if_analyses_output::ListWhatIfAnalysesOutputBuilder;

pub use crate::operation::list_what_if_analyses::_list_what_if_analyses_input::ListWhatIfAnalysesInputBuilder;

impl ListWhatIfAnalysesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_what_if_analyses::ListWhatIfAnalysesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_what_if_analyses::ListWhatIfAnalysesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_what_if_analyses();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListWhatIfAnalyses`.
///
/// <p>Returns a list of what-if analyses created using the <code>CreateWhatIfAnalysis</code> operation. For each what-if analysis, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). You can retrieve the complete set of properties by using the what-if analysis ARN with the <code>DescribeWhatIfAnalysis</code> operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListWhatIfAnalysesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_what_if_analyses::builders::ListWhatIfAnalysesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_what_if_analyses::ListWhatIfAnalysesOutput,
        crate::operation::list_what_if_analyses::ListWhatIfAnalysesError,
    > for ListWhatIfAnalysesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_what_if_analyses::ListWhatIfAnalysesOutput,
            crate::operation::list_what_if_analyses::ListWhatIfAnalysesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListWhatIfAnalysesFluentBuilder {
    /// Creates a new `ListWhatIfAnalyses`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListWhatIfAnalyses as a reference.
    pub fn as_input(&self) -> &crate::operation::list_what_if_analyses::builders::ListWhatIfAnalysesInputBuilder {
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
        crate::operation::list_what_if_analyses::ListWhatIfAnalysesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_what_if_analyses::ListWhatIfAnalysesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_what_if_analyses::ListWhatIfAnalyses::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_what_if_analyses::ListWhatIfAnalyses::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_what_if_analyses::ListWhatIfAnalysesOutput,
        crate::operation::list_what_if_analyses::ListWhatIfAnalysesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_what_if_analyses::paginator::ListWhatIfAnalysesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_what_if_analyses::paginator::ListWhatIfAnalysesPaginator {
        crate::operation::list_what_if_analyses::paginator::ListWhatIfAnalysesPaginator::new(self.handle, self.inner)
    }
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The number of items to return in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The number of items to return in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The number of items to return in the response.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>An array of filters. For each filter, you provide a condition and a match statement. The condition is either <code>IS</code> or <code>IS_NOT</code>, which specifies whether to include or exclude the what-if analysis jobs that match the statement from the list, respectively. The match statement consists of a key and a value.</p>
    /// <p><b>Filter properties</b></p>
    /// <ul>
    /// <li>
    /// <p><code>Condition</code> - The condition to apply. Valid values are <code>IS</code> and <code>IS_NOT</code>. To include the what-if analysis jobs that match the statement, specify <code>IS</code>. To exclude matching what-if analysis jobs, specify <code>IS_NOT</code>.</p></li>
    /// <li>
    /// <p><code>Key</code> - The name of the parameter to filter on. Valid values are <code>WhatIfAnalysisArn</code> and <code>Status</code>.</p></li>
    /// <li>
    /// <p><code>Value</code> - The value to match.</p></li>
    /// </ul>
    /// <p>For example, to list all jobs that export a forecast named <i>electricityWhatIf</i>, specify the following filter:</p>
    /// <p><code>"Filters": [ { "Condition": "IS", "Key": "WhatIfAnalysisArn", "Value": "arn:aws:forecast:us-west-2:<acct-id>
    /// :forecast/electricityWhatIf" } ]
    /// </acct-id></code></p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>An array of filters. For each filter, you provide a condition and a match statement. The condition is either <code>IS</code> or <code>IS_NOT</code>, which specifies whether to include or exclude the what-if analysis jobs that match the statement from the list, respectively. The match statement consists of a key and a value.</p>
    /// <p><b>Filter properties</b></p>
    /// <ul>
    /// <li>
    /// <p><code>Condition</code> - The condition to apply. Valid values are <code>IS</code> and <code>IS_NOT</code>. To include the what-if analysis jobs that match the statement, specify <code>IS</code>. To exclude matching what-if analysis jobs, specify <code>IS_NOT</code>.</p></li>
    /// <li>
    /// <p><code>Key</code> - The name of the parameter to filter on. Valid values are <code>WhatIfAnalysisArn</code> and <code>Status</code>.</p></li>
    /// <li>
    /// <p><code>Value</code> - The value to match.</p></li>
    /// </ul>
    /// <p>For example, to list all jobs that export a forecast named <i>electricityWhatIf</i>, specify the following filter:</p>
    /// <p><code>"Filters": [ { "Condition": "IS", "Key": "WhatIfAnalysisArn", "Value": "arn:aws:forecast:us-west-2:<acct-id>
    /// :forecast/electricityWhatIf" } ]
    /// </acct-id></code></p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>An array of filters. For each filter, you provide a condition and a match statement. The condition is either <code>IS</code> or <code>IS_NOT</code>, which specifies whether to include or exclude the what-if analysis jobs that match the statement from the list, respectively. The match statement consists of a key and a value.</p>
    /// <p><b>Filter properties</b></p>
    /// <ul>
    /// <li>
    /// <p><code>Condition</code> - The condition to apply. Valid values are <code>IS</code> and <code>IS_NOT</code>. To include the what-if analysis jobs that match the statement, specify <code>IS</code>. To exclude matching what-if analysis jobs, specify <code>IS_NOT</code>.</p></li>
    /// <li>
    /// <p><code>Key</code> - The name of the parameter to filter on. Valid values are <code>WhatIfAnalysisArn</code> and <code>Status</code>.</p></li>
    /// <li>
    /// <p><code>Value</code> - The value to match.</p></li>
    /// </ul>
    /// <p>For example, to list all jobs that export a forecast named <i>electricityWhatIf</i>, specify the following filter:</p>
    /// <p><code>"Filters": [ { "Condition": "IS", "Key": "WhatIfAnalysisArn", "Value": "arn:aws:forecast:us-west-2:<acct-id>
    /// :forecast/electricityWhatIf" } ]
    /// </acct-id></code></p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
}
