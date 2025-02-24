// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_response_headers_policy::_update_response_headers_policy_output::UpdateResponseHeadersPolicyOutputBuilder;

pub use crate::operation::update_response_headers_policy::_update_response_headers_policy_input::UpdateResponseHeadersPolicyInputBuilder;

impl UpdateResponseHeadersPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_response_headers_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateResponseHeadersPolicy`.
///
/// <p>Updates a response headers policy.</p>
/// <p>When you update a response headers policy, the entire policy is replaced. You cannot update some policy fields independent of others. To update a response headers policy configuration:</p>
/// <ol>
/// <li>
/// <p>Use <code>GetResponseHeadersPolicyConfig</code> to get the current policy's configuration.</p></li>
/// <li>
/// <p>Modify the fields in the response headers policy configuration that you want to update.</p></li>
/// <li>
/// <p>Call <code>UpdateResponseHeadersPolicy</code>, providing the entire response headers policy configuration, including the fields that you modified and those that you didn't.</p></li>
/// </ol>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateResponseHeadersPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_response_headers_policy::builders::UpdateResponseHeadersPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyOutput,
        crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyError,
    > for UpdateResponseHeadersPolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyOutput,
            crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateResponseHeadersPolicyFluentBuilder {
    /// Creates a new `UpdateResponseHeadersPolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateResponseHeadersPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::update_response_headers_policy::builders::UpdateResponseHeadersPolicyInputBuilder {
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
        crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyOutput,
        crate::operation::update_response_headers_policy::UpdateResponseHeadersPolicyError,
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
    /// <p>A response headers policy configuration.</p>
    pub fn response_headers_policy_config(mut self, input: crate::types::ResponseHeadersPolicyConfig) -> Self {
        self.inner = self.inner.response_headers_policy_config(input);
        self
    }
    /// <p>A response headers policy configuration.</p>
    pub fn set_response_headers_policy_config(mut self, input: ::std::option::Option<crate::types::ResponseHeadersPolicyConfig>) -> Self {
        self.inner = self.inner.set_response_headers_policy_config(input);
        self
    }
    /// <p>A response headers policy configuration.</p>
    pub fn get_response_headers_policy_config(&self) -> &::std::option::Option<crate::types::ResponseHeadersPolicyConfig> {
        self.inner.get_response_headers_policy_config()
    }
    /// <p>The identifier for the response headers policy that you are updating.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The identifier for the response headers policy that you are updating.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The identifier for the response headers policy that you are updating.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The version of the response headers policy that you are updating.</p>
    /// <p>The version is returned in the cache policy's <code>ETag</code> field in the response to <code>GetResponseHeadersPolicyConfig</code>.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>The version of the response headers policy that you are updating.</p>
    /// <p>The version is returned in the cache policy's <code>ETag</code> field in the response to <code>GetResponseHeadersPolicyConfig</code>.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
    /// <p>The version of the response headers policy that you are updating.</p>
    /// <p>The version is returned in the cache policy's <code>ETag</code> field in the response to <code>GetResponseHeadersPolicyConfig</code>.</p>
    pub fn get_if_match(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_if_match()
    }
}
