// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_update_cluster::_batch_update_cluster_output::BatchUpdateClusterOutputBuilder;

pub use crate::operation::batch_update_cluster::_batch_update_cluster_input::BatchUpdateClusterInputBuilder;

impl BatchUpdateClusterInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_update_cluster::BatchUpdateClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_update_cluster::BatchUpdateClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_update_cluster();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchUpdateCluster`.
///
/// <p>Apply the service update to a list of clusters supplied. For more information on service updates and applying them, see <a href="https://docs.aws.amazon.com/MemoryDB/latest/devguide/managing-updates.html#applying-updates">Applying the service updates</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchUpdateClusterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_update_cluster::builders::BatchUpdateClusterInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_update_cluster::BatchUpdateClusterOutput,
        crate::operation::batch_update_cluster::BatchUpdateClusterError,
    > for BatchUpdateClusterFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_update_cluster::BatchUpdateClusterOutput,
            crate::operation::batch_update_cluster::BatchUpdateClusterError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchUpdateClusterFluentBuilder {
    /// Creates a new `BatchUpdateCluster`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchUpdateCluster as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_update_cluster::builders::BatchUpdateClusterInputBuilder {
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
        crate::operation::batch_update_cluster::BatchUpdateClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_update_cluster::BatchUpdateClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_update_cluster::BatchUpdateCluster::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::batch_update_cluster::BatchUpdateCluster::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_update_cluster::BatchUpdateClusterOutput,
        crate::operation::batch_update_cluster::BatchUpdateClusterError,
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
    /// Appends an item to `ClusterNames`.
    ///
    /// To override the contents of this collection use [`set_cluster_names`](Self::set_cluster_names).
    ///
    /// <p>The cluster names to apply the updates.</p>
    pub fn cluster_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_names(input.into());
        self
    }
    /// <p>The cluster names to apply the updates.</p>
    pub fn set_cluster_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_cluster_names(input);
        self
    }
    /// <p>The cluster names to apply the updates.</p>
    pub fn get_cluster_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_cluster_names()
    }
    /// <p>The unique ID of the service update</p>
    pub fn service_update(mut self, input: crate::types::ServiceUpdateRequest) -> Self {
        self.inner = self.inner.service_update(input);
        self
    }
    /// <p>The unique ID of the service update</p>
    pub fn set_service_update(mut self, input: ::std::option::Option<crate::types::ServiceUpdateRequest>) -> Self {
        self.inner = self.inner.set_service_update(input);
        self
    }
    /// <p>The unique ID of the service update</p>
    pub fn get_service_update(&self) -> &::std::option::Option<crate::types::ServiceUpdateRequest> {
        self.inner.get_service_update()
    }
}
