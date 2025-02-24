// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::execute_provisioned_product_plan::_execute_provisioned_product_plan_output::ExecuteProvisionedProductPlanOutputBuilder;

pub use crate::operation::execute_provisioned_product_plan::_execute_provisioned_product_plan_input::ExecuteProvisionedProductPlanInputBuilder;

impl ExecuteProvisionedProductPlanInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.execute_provisioned_product_plan();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ExecuteProvisionedProductPlan`.
///
/// <p>Provisions or modifies a product based on the resource changes for the specified plan.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ExecuteProvisionedProductPlanFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::execute_provisioned_product_plan::builders::ExecuteProvisionedProductPlanInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanOutput,
        crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanError,
    > for ExecuteProvisionedProductPlanFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanOutput,
            crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ExecuteProvisionedProductPlanFluentBuilder {
    /// Creates a new `ExecuteProvisionedProductPlan`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ExecuteProvisionedProductPlan as a reference.
    pub fn as_input(&self) -> &crate::operation::execute_provisioned_product_plan::builders::ExecuteProvisionedProductPlanInputBuilder {
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
        crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlan::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlan::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanOutput,
        crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanError,
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
    /// <p>The language code.</p>
    /// <ul>
    /// <li>
    /// <p><code>jp</code> - Japanese</p></li>
    /// <li>
    /// <p><code>zh</code> - Chinese</p></li>
    /// </ul>
    pub fn accept_language(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.accept_language(input.into());
        self
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li>
    /// <p><code>jp</code> - Japanese</p></li>
    /// <li>
    /// <p><code>zh</code> - Chinese</p></li>
    /// </ul>
    pub fn set_accept_language(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_accept_language(input);
        self
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li>
    /// <p><code>jp</code> - Japanese</p></li>
    /// <li>
    /// <p><code>zh</code> - Chinese</p></li>
    /// </ul>
    pub fn get_accept_language(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_accept_language()
    }
    /// <p>The plan identifier.</p>
    pub fn plan_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.plan_id(input.into());
        self
    }
    /// <p>The plan identifier.</p>
    pub fn set_plan_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_plan_id(input);
        self
    }
    /// <p>The plan identifier.</p>
    pub fn get_plan_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_plan_id()
    }
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    pub fn idempotency_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.idempotency_token(input.into());
        self
    }
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    pub fn set_idempotency_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_idempotency_token(input);
        self
    }
    /// <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p>
    pub fn get_idempotency_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_idempotency_token()
    }
}
