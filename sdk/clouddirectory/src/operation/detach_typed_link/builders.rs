// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::detach_typed_link::_detach_typed_link_output::DetachTypedLinkOutputBuilder;

pub use crate::operation::detach_typed_link::_detach_typed_link_input::DetachTypedLinkInputBuilder;

impl DetachTypedLinkInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::detach_typed_link::DetachTypedLinkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::detach_typed_link::DetachTypedLinkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.detach_typed_link();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DetachTypedLink`.
///
/// <p>Detaches a typed link from a specified source and target object. For more information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DetachTypedLinkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::detach_typed_link::builders::DetachTypedLinkInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::detach_typed_link::DetachTypedLinkOutput,
        crate::operation::detach_typed_link::DetachTypedLinkError,
    > for DetachTypedLinkFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::detach_typed_link::DetachTypedLinkOutput,
            crate::operation::detach_typed_link::DetachTypedLinkError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DetachTypedLinkFluentBuilder {
    /// Creates a new `DetachTypedLink`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DetachTypedLink as a reference.
    pub fn as_input(&self) -> &crate::operation::detach_typed_link::builders::DetachTypedLinkInputBuilder {
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
        crate::operation::detach_typed_link::DetachTypedLinkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::detach_typed_link::DetachTypedLinkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::detach_typed_link::DetachTypedLink::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::detach_typed_link::DetachTypedLink::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::detach_typed_link::DetachTypedLinkOutput,
        crate::operation::detach_typed_link::DetachTypedLinkError,
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
    /// <p>The Amazon Resource Name (ARN) of the directory where you want to detach the typed link.</p>
    pub fn directory_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the directory where you want to detach the typed link.</p>
    pub fn set_directory_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the directory where you want to detach the typed link.</p>
    pub fn get_directory_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_arn()
    }
    /// <p>Used to accept a typed link specifier as input.</p>
    pub fn typed_link_specifier(mut self, input: crate::types::TypedLinkSpecifier) -> Self {
        self.inner = self.inner.typed_link_specifier(input);
        self
    }
    /// <p>Used to accept a typed link specifier as input.</p>
    pub fn set_typed_link_specifier(mut self, input: ::std::option::Option<crate::types::TypedLinkSpecifier>) -> Self {
        self.inner = self.inner.set_typed_link_specifier(input);
        self
    }
    /// <p>Used to accept a typed link specifier as input.</p>
    pub fn get_typed_link_specifier(&self) -> &::std::option::Option<crate::types::TypedLinkSpecifier> {
        self.inner.get_typed_link_specifier()
    }
}
