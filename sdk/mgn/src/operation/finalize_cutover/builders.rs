// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::finalize_cutover::_finalize_cutover_output::FinalizeCutoverOutputBuilder;

pub use crate::operation::finalize_cutover::_finalize_cutover_input::FinalizeCutoverInputBuilder;

impl FinalizeCutoverInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::finalize_cutover::FinalizeCutoverOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::finalize_cutover::FinalizeCutoverError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.finalize_cutover();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `FinalizeCutover`.
///
/// <p>Finalizes the cutover immediately for specific Source Servers. All AWS resources created by Application Migration Service for enabling the replication of these source servers will be terminated / deleted within 90 minutes. Launched Test or Cutover instances will NOT be terminated. The AWS Replication Agent will receive a command to uninstall itself (within 10 minutes). The following properties of the SourceServer will be changed immediately: dataReplicationInfo.dataReplicationState will be changed to DISCONNECTED; The SourceServer.lifeCycle.state will be changed to CUTOVER; The totalStorageBytes property fo each of dataReplicationInfo.replicatedDisks will be set to zero; dataReplicationInfo.lagDuration and dataReplicationInfo.lagDuration will be nullified.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct FinalizeCutoverFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::finalize_cutover::builders::FinalizeCutoverInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::finalize_cutover::FinalizeCutoverOutput,
        crate::operation::finalize_cutover::FinalizeCutoverError,
    > for FinalizeCutoverFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::finalize_cutover::FinalizeCutoverOutput,
            crate::operation::finalize_cutover::FinalizeCutoverError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl FinalizeCutoverFluentBuilder {
    /// Creates a new `FinalizeCutover`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the FinalizeCutover as a reference.
    pub fn as_input(&self) -> &crate::operation::finalize_cutover::builders::FinalizeCutoverInputBuilder {
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
        crate::operation::finalize_cutover::FinalizeCutoverOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::finalize_cutover::FinalizeCutoverError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::finalize_cutover::FinalizeCutover::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::finalize_cutover::FinalizeCutover::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::finalize_cutover::FinalizeCutoverOutput,
        crate::operation::finalize_cutover::FinalizeCutoverError,
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
    /// <p>Request to finalize Cutover by Source Server ID.</p>
    pub fn source_server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_server_id(input.into());
        self
    }
    /// <p>Request to finalize Cutover by Source Server ID.</p>
    pub fn set_source_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_server_id(input);
        self
    }
    /// <p>Request to finalize Cutover by Source Server ID.</p>
    pub fn get_source_server_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_server_id()
    }
    /// <p>Request to finalize Cutover by Source Account ID.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>Request to finalize Cutover by Source Account ID.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>Request to finalize Cutover by Source Account ID.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
}
