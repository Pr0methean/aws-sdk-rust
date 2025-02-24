// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_conversation::_start_conversation_output::StartConversationOutputBuilder;

pub use crate::operation::start_conversation::_start_conversation_input::StartConversationInputBuilder;

impl StartConversationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_conversation::StartConversationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_conversation::StartConversationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_conversation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartConversation`.
///
/// <p>Starts an HTTP/2 bidirectional event stream that enables you to send audio, text, or DTMF input in real time. After your application starts a conversation, users send input to Amazon Lex V2 as a stream of events. Amazon Lex V2 processes the incoming events and responds with streaming text or audio events.</p>
/// <p>Audio input must be in the following format: <code>audio/lpcm sample-rate=8000 sample-size-bits=16 channel-count=1; is-big-endian=false</code>.</p>
/// <p>If the optional post-fulfillment response is specified, the messages are returned as follows. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/API_PostFulfillmentStatusSpecification.html">PostFulfillmentStatusSpecification</a>.</p>
/// <ul>
/// <li>
/// <p><b>Success message</b> - Returned if the Lambda function completes successfully and the intent state is fulfilled or ready fulfillment if the message is present.</p></li>
/// <li>
/// <p><b>Failed message</b> - The failed message is returned if the Lambda function throws an exception or if the Lambda function returns a failed intent state without a message.</p></li>
/// <li>
/// <p><b>Timeout message</b> - If you don't configure a timeout message and a timeout, and the Lambda function doesn't return within 30 seconds, the timeout message is returned. If you configure a timeout, the timeout message is returned when the period times out.</p></li>
/// </ul>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/streaming-progress.html#progress-complete.html">Completion message</a>.</p>
/// <p>If the optional update message is configured, it is played at the specified frequency while the Lambda function is running and the update message state is active. If the fulfillment update message is not active, the Lambda function runs with a 30 second timeout.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/streaming-progress.html#progress-update.html">Update message </a></p>
/// <p>The <code>StartConversation</code> operation is supported only in the following SDKs:</p>
/// <ul>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/goto/SdkForCpp/runtime.lex.v2-2020-08-07/StartConversation">AWS SDK for C++</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/goto/SdkForJavaV2/runtime.lex.v2-2020-08-07/StartConversation">AWS SDK for Java V2</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/goto/SdkForRubyV3/runtime.lex.v2-2020-08-07/StartConversation">AWS SDK for Ruby V3</a></p></li>
/// </ul>
#[derive(::std::fmt::Debug)]
pub struct StartConversationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_conversation::builders::StartConversationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_conversation::StartConversationOutput,
        crate::operation::start_conversation::StartConversationError,
    > for StartConversationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_conversation::StartConversationOutput,
            crate::operation::start_conversation::StartConversationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartConversationFluentBuilder {
    /// Creates a new `StartConversation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartConversation as a reference.
    pub fn as_input(&self) -> &crate::operation::start_conversation::builders::StartConversationInputBuilder {
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
        crate::operation::start_conversation::StartConversationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_conversation::StartConversationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_conversation::StartConversation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_conversation::StartConversation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_conversation::StartConversationOutput,
        crate::operation::start_conversation::StartConversationError,
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
    /// <p>The identifier of the bot to process the request.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The identifier of the bot to process the request.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The identifier of the bot to process the request.</p>
    pub fn get_bot_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_id()
    }
    /// <p>The alias identifier in use for the bot that processes the request.</p>
    pub fn bot_alias_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_alias_id(input.into());
        self
    }
    /// <p>The alias identifier in use for the bot that processes the request.</p>
    pub fn set_bot_alias_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_alias_id(input);
        self
    }
    /// <p>The alias identifier in use for the bot that processes the request.</p>
    pub fn get_bot_alias_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_alias_id()
    }
    /// <p>The locale where the session is in use.</p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The locale where the session is in use.</p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
    /// <p>The locale where the session is in use.</p>
    pub fn get_locale_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_locale_id()
    }
    /// <p>The identifier of the user session that is having the conversation.</p>
    pub fn session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.session_id(input.into());
        self
    }
    /// <p>The identifier of the user session that is having the conversation.</p>
    pub fn set_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_session_id(input);
        self
    }
    /// <p>The identifier of the user session that is having the conversation.</p>
    pub fn get_session_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_session_id()
    }
    /// <p>The conversation type that you are using the Amazon Lex V2. If the conversation mode is <code>AUDIO</code> you can send both audio and DTMF information. If the mode is <code>TEXT</code> you can only send text.</p>
    pub fn conversation_mode(mut self, input: crate::types::ConversationMode) -> Self {
        self.inner = self.inner.conversation_mode(input);
        self
    }
    /// <p>The conversation type that you are using the Amazon Lex V2. If the conversation mode is <code>AUDIO</code> you can send both audio and DTMF information. If the mode is <code>TEXT</code> you can only send text.</p>
    pub fn set_conversation_mode(mut self, input: ::std::option::Option<crate::types::ConversationMode>) -> Self {
        self.inner = self.inner.set_conversation_mode(input);
        self
    }
    /// <p>The conversation type that you are using the Amazon Lex V2. If the conversation mode is <code>AUDIO</code> you can send both audio and DTMF information. If the mode is <code>TEXT</code> you can only send text.</p>
    pub fn get_conversation_mode(&self) -> &::std::option::Option<crate::types::ConversationMode> {
        self.inner.get_conversation_mode()
    }
    /// <p>Represents the stream of events to Amazon Lex V2 from your application. The events are encoded as HTTP/2 data frames.</p>
    pub fn request_event_stream(
        mut self,
        input: ::aws_smithy_http::event_stream::EventStreamSender<
            crate::types::StartConversationRequestEventStream,
            crate::types::error::StartConversationRequestEventStreamError,
        >,
    ) -> Self {
        self.inner = self.inner.request_event_stream(input);
        self
    }
    /// <p>Represents the stream of events to Amazon Lex V2 from your application. The events are encoded as HTTP/2 data frames.</p>
    pub fn set_request_event_stream(
        mut self,
        input: ::std::option::Option<
            ::aws_smithy_http::event_stream::EventStreamSender<
                crate::types::StartConversationRequestEventStream,
                crate::types::error::StartConversationRequestEventStreamError,
            >,
        >,
    ) -> Self {
        self.inner = self.inner.set_request_event_stream(input);
        self
    }
    /// <p>Represents the stream of events to Amazon Lex V2 from your application. The events are encoded as HTTP/2 data frames.</p>
    pub fn get_request_event_stream(
        &self,
    ) -> &::std::option::Option<
        ::aws_smithy_http::event_stream::EventStreamSender<
            crate::types::StartConversationRequestEventStream,
            crate::types::error::StartConversationRequestEventStreamError,
        >,
    > {
        self.inner.get_request_event_stream()
    }
}
