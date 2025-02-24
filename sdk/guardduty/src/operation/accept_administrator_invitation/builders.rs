// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::accept_administrator_invitation::_accept_administrator_invitation_output::AcceptAdministratorInvitationOutputBuilder;

pub use crate::operation::accept_administrator_invitation::_accept_administrator_invitation_input::AcceptAdministratorInvitationInputBuilder;

impl AcceptAdministratorInvitationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.accept_administrator_invitation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AcceptAdministratorInvitation`.
///
/// <p>Accepts the invitation to be a member account and get monitored by a GuardDuty administrator account that sent the invitation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AcceptAdministratorInvitationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::accept_administrator_invitation::builders::AcceptAdministratorInvitationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationOutput,
        crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationError,
    > for AcceptAdministratorInvitationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationOutput,
            crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AcceptAdministratorInvitationFluentBuilder {
    /// Creates a new `AcceptAdministratorInvitation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AcceptAdministratorInvitation as a reference.
    pub fn as_input(&self) -> &crate::operation::accept_administrator_invitation::builders::AcceptAdministratorInvitationInputBuilder {
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
        crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::accept_administrator_invitation::AcceptAdministratorInvitation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::accept_administrator_invitation::AcceptAdministratorInvitation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationOutput,
        crate::operation::accept_administrator_invitation::AcceptAdministratorInvitationError,
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
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The unique ID of the detector of the GuardDuty member account.</p>
    pub fn get_detector_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_detector_id()
    }
    /// <p>The account ID of the GuardDuty administrator account whose invitation you're accepting.</p>
    pub fn administrator_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.administrator_id(input.into());
        self
    }
    /// <p>The account ID of the GuardDuty administrator account whose invitation you're accepting.</p>
    pub fn set_administrator_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_administrator_id(input);
        self
    }
    /// <p>The account ID of the GuardDuty administrator account whose invitation you're accepting.</p>
    pub fn get_administrator_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_administrator_id()
    }
    /// <p>The value that is used to validate the administrator account to the member account.</p>
    pub fn invitation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.invitation_id(input.into());
        self
    }
    /// <p>The value that is used to validate the administrator account to the member account.</p>
    pub fn set_invitation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_invitation_id(input);
        self
    }
    /// <p>The value that is used to validate the administrator account to the member account.</p>
    pub fn get_invitation_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_invitation_id()
    }
}
