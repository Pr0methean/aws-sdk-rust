// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_partition_index::_create_partition_index_output::CreatePartitionIndexOutputBuilder;

pub use crate::operation::create_partition_index::_create_partition_index_input::CreatePartitionIndexInputBuilder;

impl CreatePartitionIndexInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_partition_index::CreatePartitionIndexOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_partition_index::CreatePartitionIndexError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_partition_index();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreatePartitionIndex`.
///
/// <p>Creates a specified partition index in an existing table.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePartitionIndexFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_partition_index::builders::CreatePartitionIndexInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_partition_index::CreatePartitionIndexOutput,
        crate::operation::create_partition_index::CreatePartitionIndexError,
    > for CreatePartitionIndexFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_partition_index::CreatePartitionIndexOutput,
            crate::operation::create_partition_index::CreatePartitionIndexError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreatePartitionIndexFluentBuilder {
    /// Creates a new `CreatePartitionIndex`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreatePartitionIndex as a reference.
    pub fn as_input(&self) -> &crate::operation::create_partition_index::builders::CreatePartitionIndexInputBuilder {
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
        crate::operation::create_partition_index::CreatePartitionIndexOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_partition_index::CreatePartitionIndexError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_partition_index::CreatePartitionIndex::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_partition_index::CreatePartitionIndex::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_partition_index::CreatePartitionIndexOutput,
        crate::operation::create_partition_index::CreatePartitionIndexError,
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
    /// <p>The catalog ID where the table resides.</p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog_id(input.into());
        self
    }
    /// <p>The catalog ID where the table resides.</p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog_id(input);
        self
    }
    /// <p>The catalog ID where the table resides.</p>
    pub fn get_catalog_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_catalog_id()
    }
    /// <p>Specifies the name of a database in which you want to create a partition index.</p>
    pub fn database_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.database_name(input.into());
        self
    }
    /// <p>Specifies the name of a database in which you want to create a partition index.</p>
    pub fn set_database_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_database_name(input);
        self
    }
    /// <p>Specifies the name of a database in which you want to create a partition index.</p>
    pub fn get_database_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_database_name()
    }
    /// <p>Specifies the name of a table in which you want to create a partition index.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>Specifies the name of a table in which you want to create a partition index.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>Specifies the name of a table in which you want to create a partition index.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_name()
    }
    /// <p>Specifies a <code>PartitionIndex</code> structure to create a partition index in an existing table.</p>
    pub fn partition_index(mut self, input: crate::types::PartitionIndex) -> Self {
        self.inner = self.inner.partition_index(input);
        self
    }
    /// <p>Specifies a <code>PartitionIndex</code> structure to create a partition index in an existing table.</p>
    pub fn set_partition_index(mut self, input: ::std::option::Option<crate::types::PartitionIndex>) -> Self {
        self.inner = self.inner.set_partition_index(input);
        self
    }
    /// <p>Specifies a <code>PartitionIndex</code> structure to create a partition index in an existing table.</p>
    pub fn get_partition_index(&self) -> &::std::option::Option<crate::types::PartitionIndex> {
        self.inner.get_partition_index()
    }
}
