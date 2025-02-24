// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_code_repository::_update_code_repository_output::UpdateCodeRepositoryOutputBuilder;

pub use crate::operation::update_code_repository::_update_code_repository_input::UpdateCodeRepositoryInputBuilder;

impl UpdateCodeRepositoryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_code_repository::UpdateCodeRepositoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_code_repository::UpdateCodeRepositoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_code_repository();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateCodeRepository`.
///
/// <p>Updates the specified Git repository with the specified values.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateCodeRepositoryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_code_repository::builders::UpdateCodeRepositoryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_code_repository::UpdateCodeRepositoryOutput,
        crate::operation::update_code_repository::UpdateCodeRepositoryError,
    > for UpdateCodeRepositoryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_code_repository::UpdateCodeRepositoryOutput,
            crate::operation::update_code_repository::UpdateCodeRepositoryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateCodeRepositoryFluentBuilder {
    /// Creates a new `UpdateCodeRepository`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateCodeRepository as a reference.
    pub fn as_input(&self) -> &crate::operation::update_code_repository::builders::UpdateCodeRepositoryInputBuilder {
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
        crate::operation::update_code_repository::UpdateCodeRepositoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_code_repository::UpdateCodeRepositoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_code_repository::UpdateCodeRepository::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_code_repository::UpdateCodeRepository::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_code_repository::UpdateCodeRepositoryOutput,
        crate::operation::update_code_repository::UpdateCodeRepositoryError,
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
    /// <p>The name of the Git repository to update.</p>
    pub fn code_repository_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.code_repository_name(input.into());
        self
    }
    /// <p>The name of the Git repository to update.</p>
    pub fn set_code_repository_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_code_repository_name(input);
        self
    }
    /// <p>The name of the Git repository to update.</p>
    pub fn get_code_repository_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_code_repository_name()
    }
    /// <p>The configuration of the git repository, including the URL and the Amazon Resource Name (ARN) of the Amazon Web Services Secrets Manager secret that contains the credentials used to access the repository. The secret must have a staging label of <code>AWSCURRENT</code> and must be in the following format:</p>
    /// <p><code>{"username": <i>UserName</i>, "password": <i>Password</i>}</code></p>
    pub fn git_config(mut self, input: crate::types::GitConfigForUpdate) -> Self {
        self.inner = self.inner.git_config(input);
        self
    }
    /// <p>The configuration of the git repository, including the URL and the Amazon Resource Name (ARN) of the Amazon Web Services Secrets Manager secret that contains the credentials used to access the repository. The secret must have a staging label of <code>AWSCURRENT</code> and must be in the following format:</p>
    /// <p><code>{"username": <i>UserName</i>, "password": <i>Password</i>}</code></p>
    pub fn set_git_config(mut self, input: ::std::option::Option<crate::types::GitConfigForUpdate>) -> Self {
        self.inner = self.inner.set_git_config(input);
        self
    }
    /// <p>The configuration of the git repository, including the URL and the Amazon Resource Name (ARN) of the Amazon Web Services Secrets Manager secret that contains the credentials used to access the repository. The secret must have a staging label of <code>AWSCURRENT</code> and must be in the following format:</p>
    /// <p><code>{"username": <i>UserName</i>, "password": <i>Password</i>}</code></p>
    pub fn get_git_config(&self) -> &::std::option::Option<crate::types::GitConfigForUpdate> {
        self.inner.get_git_config()
    }
}
