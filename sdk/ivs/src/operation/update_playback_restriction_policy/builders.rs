// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_playback_restriction_policy::_update_playback_restriction_policy_output::UpdatePlaybackRestrictionPolicyOutputBuilder;

pub use crate::operation::update_playback_restriction_policy::_update_playback_restriction_policy_input::UpdatePlaybackRestrictionPolicyInputBuilder;

impl UpdatePlaybackRestrictionPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_playback_restriction_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdatePlaybackRestrictionPolicy`.
///
/// <p>Updates a specified playback restriction policy.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePlaybackRestrictionPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_playback_restriction_policy::builders::UpdatePlaybackRestrictionPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicyOutput,
        crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicyError,
    > for UpdatePlaybackRestrictionPolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicyOutput,
            crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdatePlaybackRestrictionPolicyFluentBuilder {
    /// Creates a new `UpdatePlaybackRestrictionPolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdatePlaybackRestrictionPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::update_playback_restriction_policy::builders::UpdatePlaybackRestrictionPolicyInputBuilder {
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
        crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicyOutput,
        crate::operation::update_playback_restriction_policy::UpdatePlaybackRestrictionPolicyError,
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
    /// <p>ARN of the playback-restriction-policy to be updated.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>ARN of the playback-restriction-policy to be updated.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>ARN of the playback-restriction-policy to be updated.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_arn()
    }
    /// Appends an item to `allowedCountries`.
    ///
    /// To override the contents of this collection use [`set_allowed_countries`](Self::set_allowed_countries).
    ///
    /// <p>A list of country codes that control geoblocking restriction. Allowed values are the officially assigned <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a> codes. Default: All countries (an empty array).</p>
    pub fn allowed_countries(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.allowed_countries(input.into());
        self
    }
    /// <p>A list of country codes that control geoblocking restriction. Allowed values are the officially assigned <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a> codes. Default: All countries (an empty array).</p>
    pub fn set_allowed_countries(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_allowed_countries(input);
        self
    }
    /// <p>A list of country codes that control geoblocking restriction. Allowed values are the officially assigned <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a> codes. Default: All countries (an empty array).</p>
    pub fn get_allowed_countries(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_allowed_countries()
    }
    /// Appends an item to `allowedOrigins`.
    ///
    /// To override the contents of this collection use [`set_allowed_origins`](Self::set_allowed_origins).
    ///
    /// <p>A list of origin sites that control CORS restriction. Allowed values are the same as valid values of the Origin header defined at <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Origin">https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Origin</a>. Default: All origins (an empty array).</p>
    pub fn allowed_origins(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.allowed_origins(input.into());
        self
    }
    /// <p>A list of origin sites that control CORS restriction. Allowed values are the same as valid values of the Origin header defined at <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Origin">https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Origin</a>. Default: All origins (an empty array).</p>
    pub fn set_allowed_origins(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_allowed_origins(input);
        self
    }
    /// <p>A list of origin sites that control CORS restriction. Allowed values are the same as valid values of the Origin header defined at <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Origin">https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Origin</a>. Default: All origins (an empty array).</p>
    pub fn get_allowed_origins(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_allowed_origins()
    }
    /// <p>Whether channel playback is constrained by origin site. Default: <code>false</code>.</p>
    pub fn enable_strict_origin_enforcement(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_strict_origin_enforcement(input);
        self
    }
    /// <p>Whether channel playback is constrained by origin site. Default: <code>false</code>.</p>
    pub fn set_enable_strict_origin_enforcement(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_strict_origin_enforcement(input);
        self
    }
    /// <p>Whether channel playback is constrained by origin site. Default: <code>false</code>.</p>
    pub fn get_enable_strict_origin_enforcement(&self) -> &::std::option::Option<bool> {
        self.inner.get_enable_strict_origin_enforcement()
    }
    /// <p>Playback-restriction-policy name. The value does not need to be unique.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>Playback-restriction-policy name. The value does not need to be unique.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>Playback-restriction-policy name. The value does not need to be unique.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
}
