// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_link::_create_link_output::CreateLinkOutputBuilder;

pub use crate::operation::create_link::_create_link_input::CreateLinkInputBuilder;

impl CreateLinkInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_link::CreateLinkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_link::CreateLinkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_link();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateLink`.
///
/// <p>Creates a new link for a specified site.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLinkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_link::builders::CreateLinkInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_link::CreateLinkOutput,
        crate::operation::create_link::CreateLinkError,
    > for CreateLinkFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_link::CreateLinkOutput,
            crate::operation::create_link::CreateLinkError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateLinkFluentBuilder {
    /// Creates a new `CreateLink`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateLink as a reference.
    pub fn as_input(&self) -> &crate::operation::create_link::builders::CreateLinkInputBuilder {
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
        crate::operation::create_link::CreateLinkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_link::CreateLinkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_link::CreateLink::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_link::CreateLink::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_link::CreateLinkOutput,
        crate::operation::create_link::CreateLinkError,
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
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_network_id(input.into());
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn set_global_network_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_network_id(input);
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn get_global_network_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_network_id()
    }
    /// <p>A description of the link.</p>
    /// <p>Constraints: Maximum length of 256 characters.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the link.</p>
    /// <p>Constraints: Maximum length of 256 characters.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the link.</p>
    /// <p>Constraints: Maximum length of 256 characters.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The type of the link.</p>
    /// <p>Constraints: Maximum length of 128 characters. Cannot include the following characters: | \ ^</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.r#type(input.into());
        self
    }
    /// <p>The type of the link.</p>
    /// <p>Constraints: Maximum length of 128 characters. Cannot include the following characters: | \ ^</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of the link.</p>
    /// <p>Constraints: Maximum length of 128 characters. Cannot include the following characters: | \ ^</p>
    pub fn get_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_type()
    }
    /// <p>The upload speed and download speed in Mbps.</p>
    pub fn bandwidth(mut self, input: crate::types::Bandwidth) -> Self {
        self.inner = self.inner.bandwidth(input);
        self
    }
    /// <p>The upload speed and download speed in Mbps.</p>
    pub fn set_bandwidth(mut self, input: ::std::option::Option<crate::types::Bandwidth>) -> Self {
        self.inner = self.inner.set_bandwidth(input);
        self
    }
    /// <p>The upload speed and download speed in Mbps.</p>
    pub fn get_bandwidth(&self) -> &::std::option::Option<crate::types::Bandwidth> {
        self.inner.get_bandwidth()
    }
    /// <p>The provider of the link.</p>
    /// <p>Constraints: Maximum length of 128 characters. Cannot include the following characters: | \ ^</p>
    pub fn provider(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.provider(input.into());
        self
    }
    /// <p>The provider of the link.</p>
    /// <p>Constraints: Maximum length of 128 characters. Cannot include the following characters: | \ ^</p>
    pub fn set_provider(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_provider(input);
        self
    }
    /// <p>The provider of the link.</p>
    /// <p>Constraints: Maximum length of 128 characters. Cannot include the following characters: | \ ^</p>
    pub fn get_provider(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_provider()
    }
    /// <p>The ID of the site.</p>
    pub fn site_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.site_id(input.into());
        self
    }
    /// <p>The ID of the site.</p>
    pub fn set_site_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_site_id(input);
        self
    }
    /// <p>The ID of the site.</p>
    pub fn get_site_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_site_id()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to apply to the resource during creation.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to apply to the resource during creation.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to apply to the resource during creation.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
