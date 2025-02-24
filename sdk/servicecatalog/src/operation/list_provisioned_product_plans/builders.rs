// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_provisioned_product_plans::_list_provisioned_product_plans_output::ListProvisionedProductPlansOutputBuilder;

pub use crate::operation::list_provisioned_product_plans::_list_provisioned_product_plans_input::ListProvisionedProductPlansInputBuilder;

impl ListProvisionedProductPlansInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_provisioned_product_plans::ListProvisionedProductPlansOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_provisioned_product_plans::ListProvisionedProductPlansError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_provisioned_product_plans();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListProvisionedProductPlans`.
///
/// <p>Lists the plans for the specified provisioned product or all plans to which the user has access.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListProvisionedProductPlansFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_provisioned_product_plans::builders::ListProvisionedProductPlansInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_provisioned_product_plans::ListProvisionedProductPlansOutput,
        crate::operation::list_provisioned_product_plans::ListProvisionedProductPlansError,
    > for ListProvisionedProductPlansFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_provisioned_product_plans::ListProvisionedProductPlansOutput,
            crate::operation::list_provisioned_product_plans::ListProvisionedProductPlansError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListProvisionedProductPlansFluentBuilder {
    /// Creates a new `ListProvisionedProductPlans`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListProvisionedProductPlans as a reference.
    pub fn as_input(&self) -> &crate::operation::list_provisioned_product_plans::builders::ListProvisionedProductPlansInputBuilder {
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
        crate::operation::list_provisioned_product_plans::ListProvisionedProductPlansOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_provisioned_product_plans::ListProvisionedProductPlansError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_provisioned_product_plans::ListProvisionedProductPlans::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_provisioned_product_plans::ListProvisionedProductPlans::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_provisioned_product_plans::ListProvisionedProductPlansOutput,
        crate::operation::list_provisioned_product_plans::ListProvisionedProductPlansError,
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
    /// <p>The product identifier.</p>
    pub fn provision_product_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.provision_product_id(input.into());
        self
    }
    /// <p>The product identifier.</p>
    pub fn set_provision_product_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_provision_product_id(input);
        self
    }
    /// <p>The product identifier.</p>
    pub fn get_provision_product_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_provision_product_id()
    }
    /// <p>The maximum number of items to return with this call.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The maximum number of items to return with this call.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
    /// <p>The maximum number of items to return with this call.</p>
    pub fn get_page_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_page_size()
    }
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    pub fn page_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.page_token(input.into());
        self
    }
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    pub fn set_page_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_page_token(input);
        self
    }
    /// <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    pub fn get_page_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_page_token()
    }
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    pub fn access_level_filter(mut self, input: crate::types::AccessLevelFilter) -> Self {
        self.inner = self.inner.access_level_filter(input);
        self
    }
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    pub fn set_access_level_filter(mut self, input: ::std::option::Option<crate::types::AccessLevelFilter>) -> Self {
        self.inner = self.inner.set_access_level_filter(input);
        self
    }
    /// <p>The access level to use to obtain results. The default is <code>User</code>.</p>
    pub fn get_access_level_filter(&self) -> &::std::option::Option<crate::types::AccessLevelFilter> {
        self.inner.get_access_level_filter()
    }
}
