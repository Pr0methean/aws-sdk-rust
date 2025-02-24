// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_phone_number_with_user::_associate_phone_number_with_user_output::AssociatePhoneNumberWithUserOutputBuilder;

pub use crate::operation::associate_phone_number_with_user::_associate_phone_number_with_user_input::AssociatePhoneNumberWithUserInputBuilder;

impl AssociatePhoneNumberWithUserInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_phone_number_with_user();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociatePhoneNumberWithUser`.
///
/// <p>Associates a phone number with the specified Amazon Chime user.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociatePhoneNumberWithUserFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_phone_number_with_user::builders::AssociatePhoneNumberWithUserInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUserOutput,
        crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUserError,
    > for AssociatePhoneNumberWithUserFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUserOutput,
            crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUserError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociatePhoneNumberWithUserFluentBuilder {
    /// Creates a new `AssociatePhoneNumberWithUser`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociatePhoneNumberWithUser as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_phone_number_with_user::builders::AssociatePhoneNumberWithUserInputBuilder {
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
        crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUser::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUser::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUserOutput,
        crate::operation::associate_phone_number_with_user::AssociatePhoneNumberWithUserError,
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
    /// <p>The Amazon Chime account ID.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Chime account ID.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The Amazon Chime account ID.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
    /// <p>The user ID.</p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_id(input.into());
        self
    }
    /// <p>The user ID.</p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_id(input);
        self
    }
    /// <p>The user ID.</p>
    pub fn get_user_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_id()
    }
    /// <p>The phone number, in E.164 format.</p>
    pub fn e164_phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.e164_phone_number(input.into());
        self
    }
    /// <p>The phone number, in E.164 format.</p>
    pub fn set_e164_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_e164_phone_number(input);
        self
    }
    /// <p>The phone number, in E.164 format.</p>
    pub fn get_e164_phone_number(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_e164_phone_number()
    }
}
