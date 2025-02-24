// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_resource_policy::_put_resource_policy_output::PutResourcePolicyOutputBuilder;

pub use crate::operation::put_resource_policy::_put_resource_policy_input::PutResourcePolicyInputBuilder;

impl PutResourcePolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_resource_policy::PutResourcePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_resource_policy::PutResourcePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_resource_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutResourcePolicy`.
///
/// <p>Sets the Data Catalog resource policy for access control.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutResourcePolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_resource_policy::builders::PutResourcePolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_resource_policy::PutResourcePolicyOutput,
        crate::operation::put_resource_policy::PutResourcePolicyError,
    > for PutResourcePolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_resource_policy::PutResourcePolicyOutput,
            crate::operation::put_resource_policy::PutResourcePolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutResourcePolicyFluentBuilder {
    /// Creates a new `PutResourcePolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutResourcePolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::put_resource_policy::builders::PutResourcePolicyInputBuilder {
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
        crate::operation::put_resource_policy::PutResourcePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_resource_policy::PutResourcePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_resource_policy::PutResourcePolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_resource_policy::PutResourcePolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_resource_policy::PutResourcePolicyOutput,
        crate::operation::put_resource_policy::PutResourcePolicyError,
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
    /// <p>Contains the policy document to set, in JSON format.</p>
    pub fn policy_in_json(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_in_json(input.into());
        self
    }
    /// <p>Contains the policy document to set, in JSON format.</p>
    pub fn set_policy_in_json(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_in_json(input);
        self
    }
    /// <p>Contains the policy document to set, in JSON format.</p>
    pub fn get_policy_in_json(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_in_json()
    }
    /// <p>Do not use. For internal use only.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>Do not use. For internal use only.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>Do not use. For internal use only.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_arn()
    }
    /// <p>The hash value returned when the previous policy was set using <code>PutResourcePolicy</code>. Its purpose is to prevent concurrent modifications of a policy. Do not use this parameter if no previous policy has been set.</p>
    pub fn policy_hash_condition(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_hash_condition(input.into());
        self
    }
    /// <p>The hash value returned when the previous policy was set using <code>PutResourcePolicy</code>. Its purpose is to prevent concurrent modifications of a policy. Do not use this parameter if no previous policy has been set.</p>
    pub fn set_policy_hash_condition(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_hash_condition(input);
        self
    }
    /// <p>The hash value returned when the previous policy was set using <code>PutResourcePolicy</code>. Its purpose is to prevent concurrent modifications of a policy. Do not use this parameter if no previous policy has been set.</p>
    pub fn get_policy_hash_condition(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_hash_condition()
    }
    /// <p>A value of <code>MUST_EXIST</code> is used to update a policy. A value of <code>NOT_EXIST</code> is used to create a new policy. If a value of <code>NONE</code> or a null value is used, the call does not depend on the existence of a policy.</p>
    pub fn policy_exists_condition(mut self, input: crate::types::ExistCondition) -> Self {
        self.inner = self.inner.policy_exists_condition(input);
        self
    }
    /// <p>A value of <code>MUST_EXIST</code> is used to update a policy. A value of <code>NOT_EXIST</code> is used to create a new policy. If a value of <code>NONE</code> or a null value is used, the call does not depend on the existence of a policy.</p>
    pub fn set_policy_exists_condition(mut self, input: ::std::option::Option<crate::types::ExistCondition>) -> Self {
        self.inner = self.inner.set_policy_exists_condition(input);
        self
    }
    /// <p>A value of <code>MUST_EXIST</code> is used to update a policy. A value of <code>NOT_EXIST</code> is used to create a new policy. If a value of <code>NONE</code> or a null value is used, the call does not depend on the existence of a policy.</p>
    pub fn get_policy_exists_condition(&self) -> &::std::option::Option<crate::types::ExistCondition> {
        self.inner.get_policy_exists_condition()
    }
    /// <p>If <code>'TRUE'</code>, indicates that you are using both methods to grant cross-account access to Data Catalog resources:</p>
    /// <ul>
    /// <li>
    /// <p>By directly updating the resource policy with <code>PutResourePolicy</code></p></li>
    /// <li>
    /// <p>By using the <b>Grant permissions</b> command on the Amazon Web Services Management Console.</p></li>
    /// </ul>
    /// <p>Must be set to <code>'TRUE'</code> if you have already used the Management Console to grant cross-account access, otherwise the call fails. Default is 'FALSE'.</p>
    pub fn enable_hybrid(mut self, input: crate::types::EnableHybridValues) -> Self {
        self.inner = self.inner.enable_hybrid(input);
        self
    }
    /// <p>If <code>'TRUE'</code>, indicates that you are using both methods to grant cross-account access to Data Catalog resources:</p>
    /// <ul>
    /// <li>
    /// <p>By directly updating the resource policy with <code>PutResourePolicy</code></p></li>
    /// <li>
    /// <p>By using the <b>Grant permissions</b> command on the Amazon Web Services Management Console.</p></li>
    /// </ul>
    /// <p>Must be set to <code>'TRUE'</code> if you have already used the Management Console to grant cross-account access, otherwise the call fails. Default is 'FALSE'.</p>
    pub fn set_enable_hybrid(mut self, input: ::std::option::Option<crate::types::EnableHybridValues>) -> Self {
        self.inner = self.inner.set_enable_hybrid(input);
        self
    }
    /// <p>If <code>'TRUE'</code>, indicates that you are using both methods to grant cross-account access to Data Catalog resources:</p>
    /// <ul>
    /// <li>
    /// <p>By directly updating the resource policy with <code>PutResourePolicy</code></p></li>
    /// <li>
    /// <p>By using the <b>Grant permissions</b> command on the Amazon Web Services Management Console.</p></li>
    /// </ul>
    /// <p>Must be set to <code>'TRUE'</code> if you have already used the Management Console to grant cross-account access, otherwise the call fails. Default is 'FALSE'.</p>
    pub fn get_enable_hybrid(&self) -> &::std::option::Option<crate::types::EnableHybridValues> {
        self.inner.get_enable_hybrid()
    }
}
