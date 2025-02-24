// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_active_receipt_rule_set::_describe_active_receipt_rule_set_output::DescribeActiveReceiptRuleSetOutputBuilder;

pub use crate::operation::describe_active_receipt_rule_set::_describe_active_receipt_rule_set_input::DescribeActiveReceiptRuleSetInputBuilder;

impl DescribeActiveReceiptRuleSetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_active_receipt_rule_set();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeActiveReceiptRuleSet`.
///
/// <p>Returns the metadata and receipt rules for the receipt rule set that is currently active.</p>
/// <p>For information about setting up receipt rule sets, see the <a href="https://docs.aws.amazon.com/ses/latest/dg/receiving-email-concepts.html#receiving-email-concepts-rules">Amazon SES Developer Guide</a>.</p>
/// <p>You can execute this operation no more than once per second.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeActiveReceiptRuleSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_active_receipt_rule_set::builders::DescribeActiveReceiptRuleSetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSetOutput,
        crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSetError,
    > for DescribeActiveReceiptRuleSetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSetOutput,
            crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeActiveReceiptRuleSetFluentBuilder {
    /// Creates a new `DescribeActiveReceiptRuleSet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeActiveReceiptRuleSet as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_active_receipt_rule_set::builders::DescribeActiveReceiptRuleSetInputBuilder {
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
        crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSetOutput,
        crate::operation::describe_active_receipt_rule_set::DescribeActiveReceiptRuleSetError,
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
}
