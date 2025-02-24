// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_application_version::_create_application_version_output::CreateApplicationVersionOutputBuilder;

pub use crate::operation::create_application_version::_create_application_version_input::CreateApplicationVersionInputBuilder;

impl CreateApplicationVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_application_version::CreateApplicationVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_application_version::CreateApplicationVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_application_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateApplicationVersion`.
///
/// <p>Creates an application version.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateApplicationVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_application_version::builders::CreateApplicationVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_application_version::CreateApplicationVersionOutput,
        crate::operation::create_application_version::CreateApplicationVersionError,
    > for CreateApplicationVersionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_application_version::CreateApplicationVersionOutput,
            crate::operation::create_application_version::CreateApplicationVersionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateApplicationVersionFluentBuilder {
    /// Creates a new `CreateApplicationVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateApplicationVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::create_application_version::builders::CreateApplicationVersionInputBuilder {
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
        crate::operation::create_application_version::CreateApplicationVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_application_version::CreateApplicationVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_application_version::CreateApplicationVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_application_version::CreateApplicationVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_application_version::CreateApplicationVersionOutput,
        crate::operation::create_application_version::CreateApplicationVersionError,
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
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    pub fn application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    pub fn set_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    pub fn get_application_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_id()
    }
    /// <p>The semantic version of the new version.</p>
    pub fn semantic_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.semantic_version(input.into());
        self
    }
    /// <p>The semantic version of the new version.</p>
    pub fn set_semantic_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_semantic_version(input);
        self
    }
    /// <p>The semantic version of the new version.</p>
    pub fn get_semantic_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_semantic_version()
    }
    /// <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p>
    /// <p>Maximum size 50 MB</p>
    pub fn source_code_archive_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_code_archive_url(input.into());
        self
    }
    /// <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p>
    /// <p>Maximum size 50 MB</p>
    pub fn set_source_code_archive_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_code_archive_url(input);
        self
    }
    /// <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p>
    /// <p>Maximum size 50 MB</p>
    pub fn get_source_code_archive_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_code_archive_url()
    }
    /// <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p>
    pub fn source_code_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_code_url(input.into());
        self
    }
    /// <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p>
    pub fn set_source_code_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_code_url(input);
        self
    }
    /// <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p>
    pub fn get_source_code_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_code_url()
    }
    /// <p>The raw packaged AWS SAM template of your application.</p>
    pub fn template_body(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_body(input.into());
        self
    }
    /// <p>The raw packaged AWS SAM template of your application.</p>
    pub fn set_template_body(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_body(input);
        self
    }
    /// <p>The raw packaged AWS SAM template of your application.</p>
    pub fn get_template_body(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_body()
    }
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    pub fn template_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_url(input.into());
        self
    }
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    pub fn set_template_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_url(input);
        self
    }
    /// <p>A link to the packaged AWS SAM template of your application.</p>
    pub fn get_template_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_url()
    }
}
