// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deregister_db_proxy_targets::_deregister_db_proxy_targets_output::DeregisterDbProxyTargetsOutputBuilder;

pub use crate::operation::deregister_db_proxy_targets::_deregister_db_proxy_targets_input::DeregisterDbProxyTargetsInputBuilder;

impl DeregisterDbProxyTargetsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::deregister_db_proxy_targets::DeregisterDbProxyTargetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.deregister_db_proxy_targets();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeregisterDBProxyTargets`.
///
/// <p>Remove the association between one or more <code>DBProxyTarget</code> data structures and a <code>DBProxyTargetGroup</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeregisterDBProxyTargetsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::deregister_db_proxy_targets::builders::DeregisterDbProxyTargetsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::deregister_db_proxy_targets::DeregisterDbProxyTargetsOutput,
        crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError,
    > for DeregisterDBProxyTargetsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::deregister_db_proxy_targets::DeregisterDbProxyTargetsOutput,
            crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeregisterDBProxyTargetsFluentBuilder {
    /// Creates a new `DeregisterDBProxyTargets`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeregisterDBProxyTargets as a reference.
    pub fn as_input(&self) -> &crate::operation::deregister_db_proxy_targets::builders::DeregisterDbProxyTargetsInputBuilder {
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
        crate::operation::deregister_db_proxy_targets::DeregisterDbProxyTargetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargets::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargets::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::deregister_db_proxy_targets::DeregisterDbProxyTargetsOutput,
        crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError,
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
    /// <p>The identifier of the <code>DBProxy</code> that is associated with the <code>DBProxyTargetGroup</code>.</p>
    pub fn db_proxy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_proxy_name(input.into());
        self
    }
    /// <p>The identifier of the <code>DBProxy</code> that is associated with the <code>DBProxyTargetGroup</code>.</p>
    pub fn set_db_proxy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_proxy_name(input);
        self
    }
    /// <p>The identifier of the <code>DBProxy</code> that is associated with the <code>DBProxyTargetGroup</code>.</p>
    pub fn get_db_proxy_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_proxy_name()
    }
    /// <p>The identifier of the <code>DBProxyTargetGroup</code>.</p>
    pub fn target_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_group_name(input.into());
        self
    }
    /// <p>The identifier of the <code>DBProxyTargetGroup</code>.</p>
    pub fn set_target_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_group_name(input);
        self
    }
    /// <p>The identifier of the <code>DBProxyTargetGroup</code>.</p>
    pub fn get_target_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_group_name()
    }
    /// Appends an item to `DBInstanceIdentifiers`.
    ///
    /// To override the contents of this collection use [`set_db_instance_identifiers`](Self::set_db_instance_identifiers).
    ///
    /// <p>One or more DB instance identifiers.</p>
    pub fn db_instance_identifiers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_instance_identifiers(input.into());
        self
    }
    /// <p>One or more DB instance identifiers.</p>
    pub fn set_db_instance_identifiers(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_db_instance_identifiers(input);
        self
    }
    /// <p>One or more DB instance identifiers.</p>
    pub fn get_db_instance_identifiers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_db_instance_identifiers()
    }
    /// Appends an item to `DBClusterIdentifiers`.
    ///
    /// To override the contents of this collection use [`set_db_cluster_identifiers`](Self::set_db_cluster_identifiers).
    ///
    /// <p>One or more DB cluster identifiers.</p>
    pub fn db_cluster_identifiers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_cluster_identifiers(input.into());
        self
    }
    /// <p>One or more DB cluster identifiers.</p>
    pub fn set_db_cluster_identifiers(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_db_cluster_identifiers(input);
        self
    }
    /// <p>One or more DB cluster identifiers.</p>
    pub fn get_db_cluster_identifiers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_db_cluster_identifiers()
    }
}
