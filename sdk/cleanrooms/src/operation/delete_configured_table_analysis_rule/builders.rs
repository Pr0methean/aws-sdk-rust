// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_configured_table_analysis_rule::_delete_configured_table_analysis_rule_output::DeleteConfiguredTableAnalysisRuleOutputBuilder;

pub use crate::operation::delete_configured_table_analysis_rule::_delete_configured_table_analysis_rule_input::DeleteConfiguredTableAnalysisRuleInputBuilder;

impl DeleteConfiguredTableAnalysisRuleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_configured_table_analysis_rule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteConfiguredTableAnalysisRule`.
///
/// <p>Deletes a configured table analysis rule.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteConfiguredTableAnalysisRuleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_configured_table_analysis_rule::builders::DeleteConfiguredTableAnalysisRuleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRuleOutput,
        crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRuleError,
    > for DeleteConfiguredTableAnalysisRuleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRuleOutput,
            crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRuleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteConfiguredTableAnalysisRuleFluentBuilder {
    /// Creates a new `DeleteConfiguredTableAnalysisRule`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteConfiguredTableAnalysisRule as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_configured_table_analysis_rule::builders::DeleteConfiguredTableAnalysisRuleInputBuilder {
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
        crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRule::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRule::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRuleOutput,
        crate::operation::delete_configured_table_analysis_rule::DeleteConfiguredTableAnalysisRuleError,
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
    /// <p>The unique identifier for the configured table that the analysis rule applies to. Currently accepts the configured table ID.</p>
    pub fn configured_table_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.configured_table_identifier(input.into());
        self
    }
    /// <p>The unique identifier for the configured table that the analysis rule applies to. Currently accepts the configured table ID.</p>
    pub fn set_configured_table_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_configured_table_identifier(input);
        self
    }
    /// <p>The unique identifier for the configured table that the analysis rule applies to. Currently accepts the configured table ID.</p>
    pub fn get_configured_table_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_configured_table_identifier()
    }
    /// <p>The analysis rule type to be deleted. Configured table analysis rules are uniquely identified by their configured table identifier and analysis rule type.</p>
    pub fn analysis_rule_type(mut self, input: crate::types::ConfiguredTableAnalysisRuleType) -> Self {
        self.inner = self.inner.analysis_rule_type(input);
        self
    }
    /// <p>The analysis rule type to be deleted. Configured table analysis rules are uniquely identified by their configured table identifier and analysis rule type.</p>
    pub fn set_analysis_rule_type(mut self, input: ::std::option::Option<crate::types::ConfiguredTableAnalysisRuleType>) -> Self {
        self.inner = self.inner.set_analysis_rule_type(input);
        self
    }
    /// <p>The analysis rule type to be deleted. Configured table analysis rules are uniquely identified by their configured table identifier and analysis rule type.</p>
    pub fn get_analysis_rule_type(&self) -> &::std::option::Option<crate::types::ConfiguredTableAnalysisRuleType> {
        self.inner.get_analysis_rule_type()
    }
}
