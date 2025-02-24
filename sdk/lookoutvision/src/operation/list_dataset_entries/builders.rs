// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_dataset_entries::_list_dataset_entries_output::ListDatasetEntriesOutputBuilder;

pub use crate::operation::list_dataset_entries::_list_dataset_entries_input::ListDatasetEntriesInputBuilder;

impl ListDatasetEntriesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_dataset_entries::ListDatasetEntriesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_dataset_entries::ListDatasetEntriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_dataset_entries();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListDatasetEntries`.
///
/// <p>Lists the JSON Lines within a dataset. An Amazon Lookout for Vision JSON Line contains the anomaly information for a single image, including the image location and the assigned label.</p>
/// <p>This operation requires permissions to perform the <code>lookoutvision:ListDatasetEntries</code> operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListDatasetEntriesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_dataset_entries::builders::ListDatasetEntriesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_dataset_entries::ListDatasetEntriesOutput,
        crate::operation::list_dataset_entries::ListDatasetEntriesError,
    > for ListDatasetEntriesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_dataset_entries::ListDatasetEntriesOutput,
            crate::operation::list_dataset_entries::ListDatasetEntriesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListDatasetEntriesFluentBuilder {
    /// Creates a new `ListDatasetEntries`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListDatasetEntries as a reference.
    pub fn as_input(&self) -> &crate::operation::list_dataset_entries::builders::ListDatasetEntriesInputBuilder {
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
        crate::operation::list_dataset_entries::ListDatasetEntriesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_dataset_entries::ListDatasetEntriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_dataset_entries::ListDatasetEntries::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_dataset_entries::ListDatasetEntries::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_dataset_entries::ListDatasetEntriesOutput,
        crate::operation::list_dataset_entries::ListDatasetEntriesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_dataset_entries::paginator::ListDatasetEntriesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_dataset_entries::paginator::ListDatasetEntriesPaginator {
        crate::operation::list_dataset_entries::paginator::ListDatasetEntriesPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the project that contains the dataset that you want to list.</p>
    pub fn project_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_name(input.into());
        self
    }
    /// <p>The name of the project that contains the dataset that you want to list.</p>
    pub fn set_project_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_name(input);
        self
    }
    /// <p>The name of the project that contains the dataset that you want to list.</p>
    pub fn get_project_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_name()
    }
    /// <p>The type of the dataset that you want to list. Specify <code>train</code> to list the training dataset. Specify <code>test</code> to list the test dataset. If you have a single dataset project, specify <code>train</code>.</p>
    pub fn dataset_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dataset_type(input.into());
        self
    }
    /// <p>The type of the dataset that you want to list. Specify <code>train</code> to list the training dataset. Specify <code>test</code> to list the test dataset. If you have a single dataset project, specify <code>train</code>.</p>
    pub fn set_dataset_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dataset_type(input);
        self
    }
    /// <p>The type of the dataset that you want to list. Specify <code>train</code> to list the training dataset. Specify <code>test</code> to list the test dataset. If you have a single dataset project, specify <code>train</code>.</p>
    pub fn get_dataset_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dataset_type()
    }
    /// <p>Specify <code>true</code> to include labeled entries, otherwise specify <code>false</code>. If you don't specify a value, Lookout for Vision returns all entries.</p>
    pub fn labeled(mut self, input: bool) -> Self {
        self.inner = self.inner.labeled(input);
        self
    }
    /// <p>Specify <code>true</code> to include labeled entries, otherwise specify <code>false</code>. If you don't specify a value, Lookout for Vision returns all entries.</p>
    pub fn set_labeled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_labeled(input);
        self
    }
    /// <p>Specify <code>true</code> to include labeled entries, otherwise specify <code>false</code>. If you don't specify a value, Lookout for Vision returns all entries.</p>
    pub fn get_labeled(&self) -> &::std::option::Option<bool> {
        self.inner.get_labeled()
    }
    /// <p>Specify <code>normal</code> to include only normal images. Specify <code>anomaly</code> to only include anomalous entries. If you don't specify a value, Amazon Lookout for Vision returns normal and anomalous images.</p>
    pub fn anomaly_class(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.anomaly_class(input.into());
        self
    }
    /// <p>Specify <code>normal</code> to include only normal images. Specify <code>anomaly</code> to only include anomalous entries. If you don't specify a value, Amazon Lookout for Vision returns normal and anomalous images.</p>
    pub fn set_anomaly_class(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_anomaly_class(input);
        self
    }
    /// <p>Specify <code>normal</code> to include only normal images. Specify <code>anomaly</code> to only include anomalous entries. If you don't specify a value, Amazon Lookout for Vision returns normal and anomalous images.</p>
    pub fn get_anomaly_class(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_anomaly_class()
    }
    /// <p>Only includes entries before the specified date in the response. For example, <code>2020-06-23T00:00:00</code>.</p>
    pub fn before_creation_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.before_creation_date(input);
        self
    }
    /// <p>Only includes entries before the specified date in the response. For example, <code>2020-06-23T00:00:00</code>.</p>
    pub fn set_before_creation_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_before_creation_date(input);
        self
    }
    /// <p>Only includes entries before the specified date in the response. For example, <code>2020-06-23T00:00:00</code>.</p>
    pub fn get_before_creation_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_before_creation_date()
    }
    /// <p>Only includes entries after the specified date in the response. For example, <code>2020-06-23T00:00:00</code>.</p>
    pub fn after_creation_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.after_creation_date(input);
        self
    }
    /// <p>Only includes entries after the specified date in the response. For example, <code>2020-06-23T00:00:00</code>.</p>
    pub fn set_after_creation_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_after_creation_date(input);
        self
    }
    /// <p>Only includes entries after the specified date in the response. For example, <code>2020-06-23T00:00:00</code>.</p>
    pub fn get_after_creation_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_after_creation_date()
    }
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Lookout for Vision returns a pagination token in the response. You can use this pagination token to retrieve the next set of dataset entries.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Lookout for Vision returns a pagination token in the response. You can use this pagination token to retrieve the next set of dataset entries.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Lookout for Vision returns a pagination token in the response. You can use this pagination token to retrieve the next set of dataset entries.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>Perform a "contains" search on the values of the <code>source-ref</code> key within the dataset. For example a value of "IMG_17" returns all JSON Lines where the <code>source-ref</code> key value matches <i>*IMG_17*</i>.</p>
    pub fn source_ref_contains(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_ref_contains(input.into());
        self
    }
    /// <p>Perform a "contains" search on the values of the <code>source-ref</code> key within the dataset. For example a value of "IMG_17" returns all JSON Lines where the <code>source-ref</code> key value matches <i>*IMG_17*</i>.</p>
    pub fn set_source_ref_contains(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_ref_contains(input);
        self
    }
    /// <p>Perform a "contains" search on the values of the <code>source-ref</code> key within the dataset. For example a value of "IMG_17" returns all JSON Lines where the <code>source-ref</code> key value matches <i>*IMG_17*</i>.</p>
    pub fn get_source_ref_contains(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_ref_contains()
    }
}
