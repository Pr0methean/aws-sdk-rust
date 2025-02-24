// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::authorize_data_share::_authorize_data_share_output::AuthorizeDataShareOutputBuilder;

pub use crate::operation::authorize_data_share::_authorize_data_share_input::AuthorizeDataShareInputBuilder;

impl AuthorizeDataShareInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::authorize_data_share::AuthorizeDataShareOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::authorize_data_share::AuthorizeDataShareError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.authorize_data_share();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AuthorizeDataShare`.
///
/// <p>From a data producer account, authorizes the sharing of a datashare with one or more consumer accounts or managing entities. To authorize a datashare for a data consumer, the producer account must have the correct access permissions.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AuthorizeDataShareFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::authorize_data_share::builders::AuthorizeDataShareInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::authorize_data_share::AuthorizeDataShareOutput,
        crate::operation::authorize_data_share::AuthorizeDataShareError,
    > for AuthorizeDataShareFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::authorize_data_share::AuthorizeDataShareOutput,
            crate::operation::authorize_data_share::AuthorizeDataShareError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AuthorizeDataShareFluentBuilder {
    /// Creates a new `AuthorizeDataShare`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AuthorizeDataShare as a reference.
    pub fn as_input(&self) -> &crate::operation::authorize_data_share::builders::AuthorizeDataShareInputBuilder {
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
        crate::operation::authorize_data_share::AuthorizeDataShareOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::authorize_data_share::AuthorizeDataShareError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::authorize_data_share::AuthorizeDataShare::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::authorize_data_share::AuthorizeDataShare::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::authorize_data_share::AuthorizeDataShareOutput,
        crate::operation::authorize_data_share::AuthorizeDataShareError,
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
    /// <p>The Amazon Resource Name (ARN) of the datashare namespace that producers are to authorize sharing for.</p>
    pub fn data_share_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.data_share_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the datashare namespace that producers are to authorize sharing for.</p>
    pub fn set_data_share_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_data_share_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the datashare namespace that producers are to authorize sharing for.</p>
    pub fn get_data_share_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_data_share_arn()
    }
    /// <p>The identifier of the data consumer that is authorized to access the datashare. This identifier is an Amazon Web Services account ID or a keyword, such as ADX.</p>
    pub fn consumer_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.consumer_identifier(input.into());
        self
    }
    /// <p>The identifier of the data consumer that is authorized to access the datashare. This identifier is an Amazon Web Services account ID or a keyword, such as ADX.</p>
    pub fn set_consumer_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_consumer_identifier(input);
        self
    }
    /// <p>The identifier of the data consumer that is authorized to access the datashare. This identifier is an Amazon Web Services account ID or a keyword, such as ADX.</p>
    pub fn get_consumer_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_consumer_identifier()
    }
    /// <p>If set to true, allows write operations for a datashare.</p>
    pub fn allow_writes(mut self, input: bool) -> Self {
        self.inner = self.inner.allow_writes(input);
        self
    }
    /// <p>If set to true, allows write operations for a datashare.</p>
    pub fn set_allow_writes(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_allow_writes(input);
        self
    }
    /// <p>If set to true, allows write operations for a datashare.</p>
    pub fn get_allow_writes(&self) -> &::std::option::Option<bool> {
        self.inner.get_allow_writes()
    }
}
