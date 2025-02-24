// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_budget_performance_history::_describe_budget_performance_history_output::DescribeBudgetPerformanceHistoryOutputBuilder;

pub use crate::operation::describe_budget_performance_history::_describe_budget_performance_history_input::DescribeBudgetPerformanceHistoryInputBuilder;

impl DescribeBudgetPerformanceHistoryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_budget_performance_history();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeBudgetPerformanceHistory`.
///
/// <p>Describes the history for <code>DAILY</code>, <code>MONTHLY</code>, and <code>QUARTERLY</code> budgets. Budget history isn't available for <code>ANNUAL</code> budgets.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeBudgetPerformanceHistoryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_budget_performance_history::builders::DescribeBudgetPerformanceHistoryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistoryOutput,
        crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistoryError,
    > for DescribeBudgetPerformanceHistoryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistoryOutput,
            crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistoryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeBudgetPerformanceHistoryFluentBuilder {
    /// Creates a new `DescribeBudgetPerformanceHistory`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeBudgetPerformanceHistory as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_budget_performance_history::builders::DescribeBudgetPerformanceHistoryInputBuilder {
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
        crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistory::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistory::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistoryOutput,
        crate::operation::describe_budget_performance_history::DescribeBudgetPerformanceHistoryError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_budget_performance_history::paginator::DescribeBudgetPerformanceHistoryPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_budget_performance_history::paginator::DescribeBudgetPerformanceHistoryPaginator {
        crate::operation::describe_budget_performance_history::paginator::DescribeBudgetPerformanceHistoryPaginator::new(self.handle, self.inner)
    }
    /// <p>The account ID of the user. It's a 12-digit number.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The account ID of the user. It's a 12-digit number.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The account ID of the user. It's a 12-digit number.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
    /// <p>A string that represents the budget name. The ":" and "\" characters, and the "/action/" substring, aren't allowed.</p>
    pub fn budget_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.budget_name(input.into());
        self
    }
    /// <p>A string that represents the budget name. The ":" and "\" characters, and the "/action/" substring, aren't allowed.</p>
    pub fn set_budget_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_budget_name(input);
        self
    }
    /// <p>A string that represents the budget name. The ":" and "\" characters, and the "/action/" substring, aren't allowed.</p>
    pub fn get_budget_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_budget_name()
    }
    /// <p>Retrieves how often the budget went into an <code>ALARM</code> state for the specified time period.</p>
    pub fn time_period(mut self, input: crate::types::TimePeriod) -> Self {
        self.inner = self.inner.time_period(input);
        self
    }
    /// <p>Retrieves how often the budget went into an <code>ALARM</code> state for the specified time period.</p>
    pub fn set_time_period(mut self, input: ::std::option::Option<crate::types::TimePeriod>) -> Self {
        self.inner = self.inner.set_time_period(input);
        self
    }
    /// <p>Retrieves how often the budget went into an <code>ALARM</code> state for the specified time period.</p>
    pub fn get_time_period(&self) -> &::std::option::Option<crate::types::TimePeriod> {
        self.inner.get_time_period()
    }
    /// <p>An integer that represents how many entries a paginated response contains. The maximum is 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>An integer that represents how many entries a paginated response contains. The maximum is 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>An integer that represents how many entries a paginated response contains. The maximum is 100.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A generic string.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A generic string.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A generic string.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
