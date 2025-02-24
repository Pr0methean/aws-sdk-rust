// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_managed_insight_rules::_put_managed_insight_rules_output::PutManagedInsightRulesOutputBuilder;

pub use crate::operation::put_managed_insight_rules::_put_managed_insight_rules_input::PutManagedInsightRulesInputBuilder;

impl PutManagedInsightRulesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_managed_insight_rules::PutManagedInsightRulesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_managed_insight_rules::PutManagedInsightRulesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_managed_insight_rules();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutManagedInsightRules`.
///
/// <p>Creates a managed Contributor Insights rule for a specified Amazon Web Services resource. When you enable a managed rule, you create a Contributor Insights rule that collects data from Amazon Web Services services. You cannot edit these rules with <code>PutInsightRule</code>. The rules can be enabled, disabled, and deleted using <code>EnableInsightRules</code>, <code>DisableInsightRules</code>, and <code>DeleteInsightRules</code>. If a previously created managed rule is currently disabled, a subsequent call to this API will re-enable it. Use <code>ListManagedInsightRules</code> to describe all available rules.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutManagedInsightRulesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_managed_insight_rules::builders::PutManagedInsightRulesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_managed_insight_rules::PutManagedInsightRulesOutput,
        crate::operation::put_managed_insight_rules::PutManagedInsightRulesError,
    > for PutManagedInsightRulesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_managed_insight_rules::PutManagedInsightRulesOutput,
            crate::operation::put_managed_insight_rules::PutManagedInsightRulesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutManagedInsightRulesFluentBuilder {
    /// Creates a new `PutManagedInsightRules`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutManagedInsightRules as a reference.
    pub fn as_input(&self) -> &crate::operation::put_managed_insight_rules::builders::PutManagedInsightRulesInputBuilder {
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
        crate::operation::put_managed_insight_rules::PutManagedInsightRulesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_managed_insight_rules::PutManagedInsightRulesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_managed_insight_rules::PutManagedInsightRules::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_managed_insight_rules::PutManagedInsightRules::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_managed_insight_rules::PutManagedInsightRulesOutput,
        crate::operation::put_managed_insight_rules::PutManagedInsightRulesError,
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
    /// Appends an item to `ManagedRules`.
    ///
    /// To override the contents of this collection use [`set_managed_rules`](Self::set_managed_rules).
    ///
    /// <p>A list of <code>ManagedRules</code> to enable.</p>
    pub fn managed_rules(mut self, input: crate::types::ManagedRule) -> Self {
        self.inner = self.inner.managed_rules(input);
        self
    }
    /// <p>A list of <code>ManagedRules</code> to enable.</p>
    pub fn set_managed_rules(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ManagedRule>>) -> Self {
        self.inner = self.inner.set_managed_rules(input);
        self
    }
    /// <p>A list of <code>ManagedRules</code> to enable.</p>
    pub fn get_managed_rules(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ManagedRule>> {
        self.inner.get_managed_rules()
    }
}
