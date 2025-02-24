// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_directory_config::_delete_directory_config_output::DeleteDirectoryConfigOutputBuilder;

pub use crate::operation::delete_directory_config::_delete_directory_config_input::DeleteDirectoryConfigInputBuilder;

impl DeleteDirectoryConfigInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_directory_config::DeleteDirectoryConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_directory_config::DeleteDirectoryConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_directory_config();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteDirectoryConfig`.
///
/// <p>Deletes the specified Directory Config object from AppStream 2.0. This object includes the information required to join streaming instances to an Active Directory domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDirectoryConfigFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_directory_config::builders::DeleteDirectoryConfigInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_directory_config::DeleteDirectoryConfigOutput,
        crate::operation::delete_directory_config::DeleteDirectoryConfigError,
    > for DeleteDirectoryConfigFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_directory_config::DeleteDirectoryConfigOutput,
            crate::operation::delete_directory_config::DeleteDirectoryConfigError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteDirectoryConfigFluentBuilder {
    /// Creates a new `DeleteDirectoryConfig`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteDirectoryConfig as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_directory_config::builders::DeleteDirectoryConfigInputBuilder {
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
        crate::operation::delete_directory_config::DeleteDirectoryConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_directory_config::DeleteDirectoryConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_directory_config::DeleteDirectoryConfig::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_directory_config::DeleteDirectoryConfig::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_directory_config::DeleteDirectoryConfigOutput,
        crate::operation::delete_directory_config::DeleteDirectoryConfigError,
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
    /// <p>The name of the directory configuration.</p>
    pub fn directory_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_name(input.into());
        self
    }
    /// <p>The name of the directory configuration.</p>
    pub fn set_directory_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_name(input);
        self
    }
    /// <p>The name of the directory configuration.</p>
    pub fn get_directory_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_name()
    }
}
