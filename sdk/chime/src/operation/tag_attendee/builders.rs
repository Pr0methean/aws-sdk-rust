// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::tag_attendee::_tag_attendee_output::TagAttendeeOutputBuilder;

pub use crate::operation::tag_attendee::_tag_attendee_input::TagAttendeeInputBuilder;

impl TagAttendeeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::tag_attendee::TagAttendeeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::tag_attendee::TagAttendeeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.tag_attendee();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TagAttendee`.
///
/// <p>Applies the specified tags to the specified Amazon Chime attendee.</p><important>
/// <p>TagAttendee is not supported in the Amazon Chime SDK Meetings Namespace. Update your application to remove calls to this API.</p>
/// </important>
#[deprecated(
    note = "Attendee Tags are not supported in the Amazon Chime SDK Meetings Namespace. Update your application to remove calls to this API."
)]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TagAttendeeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::tag_attendee::builders::TagAttendeeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::tag_attendee::TagAttendeeOutput,
        crate::operation::tag_attendee::TagAttendeeError,
    > for TagAttendeeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::tag_attendee::TagAttendeeOutput,
            crate::operation::tag_attendee::TagAttendeeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl TagAttendeeFluentBuilder {
    /// Creates a new `TagAttendee`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the TagAttendee as a reference.
    pub fn as_input(&self) -> &crate::operation::tag_attendee::builders::TagAttendeeInputBuilder {
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
        crate::operation::tag_attendee::TagAttendeeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::tag_attendee::TagAttendeeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::tag_attendee::TagAttendee::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::tag_attendee::TagAttendee::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::tag_attendee::TagAttendeeOutput,
        crate::operation::tag_attendee::TagAttendeeError,
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
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.meeting_id(input.into());
        self
    }
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn set_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_meeting_id(input);
        self
    }
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn get_meeting_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_meeting_id()
    }
    /// <p>The Amazon Chime SDK attendee ID.</p>
    pub fn attendee_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.attendee_id(input.into());
        self
    }
    /// <p>The Amazon Chime SDK attendee ID.</p>
    pub fn set_attendee_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_attendee_id(input);
        self
    }
    /// <p>The Amazon Chime SDK attendee ID.</p>
    pub fn get_attendee_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_attendee_id()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tag key-value pairs.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tag key-value pairs.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tag key-value pairs.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
