// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_serverless_cache_snapshots::_describe_serverless_cache_snapshots_output::DescribeServerlessCacheSnapshotsOutputBuilder;

pub use crate::operation::describe_serverless_cache_snapshots::_describe_serverless_cache_snapshots_input::DescribeServerlessCacheSnapshotsInputBuilder;

impl DescribeServerlessCacheSnapshotsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshotsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshotsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_serverless_cache_snapshots();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeServerlessCacheSnapshots`.
///
/// <p>Returns information about serverless cache snapshots. By default, this API lists all of the customer’s serverless cache snapshots. It can also describe a single serverless cache snapshot, or the snapshots associated with a particular serverless cache. Available for Redis only.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeServerlessCacheSnapshotsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_serverless_cache_snapshots::builders::DescribeServerlessCacheSnapshotsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshotsOutput,
        crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshotsError,
    > for DescribeServerlessCacheSnapshotsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshotsOutput,
            crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshotsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeServerlessCacheSnapshotsFluentBuilder {
    /// Creates a new `DescribeServerlessCacheSnapshots`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeServerlessCacheSnapshots as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_serverless_cache_snapshots::builders::DescribeServerlessCacheSnapshotsInputBuilder {
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
        crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshotsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshotsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshots::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshots::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshotsOutput,
        crate::operation::describe_serverless_cache_snapshots::DescribeServerlessCacheSnapshotsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_serverless_cache_snapshots::paginator::DescribeServerlessCacheSnapshotsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_serverless_cache_snapshots::paginator::DescribeServerlessCacheSnapshotsPaginator {
        crate::operation::describe_serverless_cache_snapshots::paginator::DescribeServerlessCacheSnapshotsPaginator::new(self.handle, self.inner)
    }
    /// <p>The identifier of serverless cache. If this parameter is specified, only snapshots associated with that specific serverless cache are described. Available for Redis only.</p>
    pub fn serverless_cache_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.serverless_cache_name(input.into());
        self
    }
    /// <p>The identifier of serverless cache. If this parameter is specified, only snapshots associated with that specific serverless cache are described. Available for Redis only.</p>
    pub fn set_serverless_cache_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_serverless_cache_name(input);
        self
    }
    /// <p>The identifier of serverless cache. If this parameter is specified, only snapshots associated with that specific serverless cache are described. Available for Redis only.</p>
    pub fn get_serverless_cache_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_serverless_cache_name()
    }
    /// <p>The identifier of the serverless cache’s snapshot. If this parameter is specified, only this snapshot is described. Available for Redis only.</p>
    pub fn serverless_cache_snapshot_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.serverless_cache_snapshot_name(input.into());
        self
    }
    /// <p>The identifier of the serverless cache’s snapshot. If this parameter is specified, only this snapshot is described. Available for Redis only.</p>
    pub fn set_serverless_cache_snapshot_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_serverless_cache_snapshot_name(input);
        self
    }
    /// <p>The identifier of the serverless cache’s snapshot. If this parameter is specified, only this snapshot is described. Available for Redis only.</p>
    pub fn get_serverless_cache_snapshot_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_serverless_cache_snapshot_name()
    }
    /// <p>The type of snapshot that is being described. Available for Redis only.</p>
    pub fn snapshot_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.snapshot_type(input.into());
        self
    }
    /// <p>The type of snapshot that is being described. Available for Redis only.</p>
    pub fn set_snapshot_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_type(input);
        self
    }
    /// <p>The type of snapshot that is being described. Available for Redis only.</p>
    pub fn get_snapshot_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_snapshot_type()
    }
    /// <p>An optional marker returned from a prior request to support pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by max-results. Available for Redis only.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An optional marker returned from a prior request to support pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by max-results. Available for Redis only.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>An optional marker returned from a prior request to support pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by max-results. Available for Redis only.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified max-results value, a market is included in the response so that remaining results can be retrieved. Available for Redis only.The default is 50. The Validation Constraints are a maximum of 50.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified max-results value, a market is included in the response so that remaining results can be retrieved. Available for Redis only.The default is 50. The Validation Constraints are a maximum of 50.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified max-results value, a market is included in the response so that remaining results can be retrieved. Available for Redis only.The default is 50. The Validation Constraints are a maximum of 50.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
