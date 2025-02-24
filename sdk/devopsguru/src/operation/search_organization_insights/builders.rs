// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_organization_insights::_search_organization_insights_output::SearchOrganizationInsightsOutputBuilder;

pub use crate::operation::search_organization_insights::_search_organization_insights_input::SearchOrganizationInsightsInputBuilder;

impl SearchOrganizationInsightsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::search_organization_insights::SearchOrganizationInsightsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_organization_insights::SearchOrganizationInsightsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.search_organization_insights();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SearchOrganizationInsights`.
///
/// <p>Returns a list of insights in your organization. You can specify which insights are returned by their start time, one or more statuses (<code>ONGOING</code>, <code>CLOSED</code>, and <code>CLOSED</code>), one or more severities (<code>LOW</code>, <code>MEDIUM</code>, and <code>HIGH</code>), and type (<code>REACTIVE</code> or <code>PROACTIVE</code>).</p>
/// <p>Use the <code>Filters</code> parameter to specify status and severity search parameters. Use the <code>Type</code> parameter to specify <code>REACTIVE</code> or <code>PROACTIVE</code> in your search.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SearchOrganizationInsightsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_organization_insights::builders::SearchOrganizationInsightsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::search_organization_insights::SearchOrganizationInsightsOutput,
        crate::operation::search_organization_insights::SearchOrganizationInsightsError,
    > for SearchOrganizationInsightsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::search_organization_insights::SearchOrganizationInsightsOutput,
            crate::operation::search_organization_insights::SearchOrganizationInsightsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SearchOrganizationInsightsFluentBuilder {
    /// Creates a new `SearchOrganizationInsights`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SearchOrganizationInsights as a reference.
    pub fn as_input(&self) -> &crate::operation::search_organization_insights::builders::SearchOrganizationInsightsInputBuilder {
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
        crate::operation::search_organization_insights::SearchOrganizationInsightsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_organization_insights::SearchOrganizationInsightsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::search_organization_insights::SearchOrganizationInsights::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::search_organization_insights::SearchOrganizationInsights::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::search_organization_insights::SearchOrganizationInsightsOutput,
        crate::operation::search_organization_insights::SearchOrganizationInsightsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::search_organization_insights::paginator::SearchOrganizationInsightsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::search_organization_insights::paginator::SearchOrganizationInsightsPaginator {
        crate::operation::search_organization_insights::paginator::SearchOrganizationInsightsPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `AccountIds`.
    ///
    /// To override the contents of this collection use [`set_account_ids`](Self::set_account_ids).
    ///
    /// <p>The ID of the Amazon Web Services account.</p>
    pub fn account_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_ids(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account.</p>
    pub fn set_account_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_account_ids(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account.</p>
    pub fn get_account_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_account_ids()
    }
    /// <p>A time range used to specify when the behavior of an insight or anomaly started.</p>
    pub fn start_time_range(mut self, input: crate::types::StartTimeRange) -> Self {
        self.inner = self.inner.start_time_range(input);
        self
    }
    /// <p>A time range used to specify when the behavior of an insight or anomaly started.</p>
    pub fn set_start_time_range(mut self, input: ::std::option::Option<crate::types::StartTimeRange>) -> Self {
        self.inner = self.inner.set_start_time_range(input);
        self
    }
    /// <p>A time range used to specify when the behavior of an insight or anomaly started.</p>
    pub fn get_start_time_range(&self) -> &::std::option::Option<crate::types::StartTimeRange> {
        self.inner.get_start_time_range()
    }
    /// <p>A <code>SearchOrganizationInsightsFilters</code> object that is used to set the severity and status filters on your insight search.</p>
    pub fn filters(mut self, input: crate::types::SearchOrganizationInsightsFilters) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>A <code>SearchOrganizationInsightsFilters</code> object that is used to set the severity and status filters on your insight search.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<crate::types::SearchOrganizationInsightsFilters>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>A <code>SearchOrganizationInsightsFilters</code> object that is used to set the severity and status filters on your insight search.</p>
    pub fn get_filters(&self) -> &::std::option::Option<crate::types::SearchOrganizationInsightsFilters> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The type of insights you are searching for (<code>REACTIVE</code> or <code>PROACTIVE</code>).</p>
    pub fn r#type(mut self, input: crate::types::InsightType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of insights you are searching for (<code>REACTIVE</code> or <code>PROACTIVE</code>).</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::InsightType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of insights you are searching for (<code>REACTIVE</code> or <code>PROACTIVE</code>).</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::InsightType> {
        self.inner.get_type()
    }
}
