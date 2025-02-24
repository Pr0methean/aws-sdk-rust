// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_connect_attachment::_create_connect_attachment_output::CreateConnectAttachmentOutputBuilder;

pub use crate::operation::create_connect_attachment::_create_connect_attachment_input::CreateConnectAttachmentInputBuilder;

impl CreateConnectAttachmentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_connect_attachment::CreateConnectAttachmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_connect_attachment::CreateConnectAttachmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_connect_attachment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateConnectAttachment`.
///
/// <p>Creates a core network Connect attachment from a specified core network attachment.</p>
/// <p>A core network Connect attachment is a GRE-based tunnel attachment that you can use to establish a connection between a core network and an appliance. A core network Connect attachment uses an existing VPC attachment as the underlying transport mechanism.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateConnectAttachmentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_connect_attachment::builders::CreateConnectAttachmentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_connect_attachment::CreateConnectAttachmentOutput,
        crate::operation::create_connect_attachment::CreateConnectAttachmentError,
    > for CreateConnectAttachmentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_connect_attachment::CreateConnectAttachmentOutput,
            crate::operation::create_connect_attachment::CreateConnectAttachmentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateConnectAttachmentFluentBuilder {
    /// Creates a new `CreateConnectAttachment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateConnectAttachment as a reference.
    pub fn as_input(&self) -> &crate::operation::create_connect_attachment::builders::CreateConnectAttachmentInputBuilder {
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
        crate::operation::create_connect_attachment::CreateConnectAttachmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_connect_attachment::CreateConnectAttachmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_connect_attachment::CreateConnectAttachment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_connect_attachment::CreateConnectAttachment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_connect_attachment::CreateConnectAttachmentOutput,
        crate::operation::create_connect_attachment::CreateConnectAttachmentError,
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
    /// <p>The ID of a core network where you want to create the attachment.</p>
    pub fn core_network_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.core_network_id(input.into());
        self
    }
    /// <p>The ID of a core network where you want to create the attachment.</p>
    pub fn set_core_network_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_core_network_id(input);
        self
    }
    /// <p>The ID of a core network where you want to create the attachment.</p>
    pub fn get_core_network_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_core_network_id()
    }
    /// <p>The Region where the edge is located.</p>
    pub fn edge_location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.edge_location(input.into());
        self
    }
    /// <p>The Region where the edge is located.</p>
    pub fn set_edge_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_edge_location(input);
        self
    }
    /// <p>The Region where the edge is located.</p>
    pub fn get_edge_location(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_edge_location()
    }
    /// <p>The ID of the attachment between the two connections.</p>
    pub fn transport_attachment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transport_attachment_id(input.into());
        self
    }
    /// <p>The ID of the attachment between the two connections.</p>
    pub fn set_transport_attachment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transport_attachment_id(input);
        self
    }
    /// <p>The ID of the attachment between the two connections.</p>
    pub fn get_transport_attachment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transport_attachment_id()
    }
    /// <p>Options for creating an attachment.</p>
    pub fn options(mut self, input: crate::types::ConnectAttachmentOptions) -> Self {
        self.inner = self.inner.options(input);
        self
    }
    /// <p>Options for creating an attachment.</p>
    pub fn set_options(mut self, input: ::std::option::Option<crate::types::ConnectAttachmentOptions>) -> Self {
        self.inner = self.inner.set_options(input);
        self
    }
    /// <p>Options for creating an attachment.</p>
    pub fn get_options(&self) -> &::std::option::Option<crate::types::ConnectAttachmentOptions> {
        self.inner.get_options()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of key-value tags associated with the request.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The list of key-value tags associated with the request.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The list of key-value tags associated with the request.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>The client token associated with the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The client token associated with the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The client token associated with the request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
