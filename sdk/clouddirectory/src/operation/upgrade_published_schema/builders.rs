// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::upgrade_published_schema::_upgrade_published_schema_output::UpgradePublishedSchemaOutputBuilder;

pub use crate::operation::upgrade_published_schema::_upgrade_published_schema_input::UpgradePublishedSchemaInputBuilder;

impl UpgradePublishedSchemaInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::upgrade_published_schema::UpgradePublishedSchemaOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::upgrade_published_schema::UpgradePublishedSchemaError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.upgrade_published_schema();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpgradePublishedSchema`.
///
/// <p>Upgrades a published schema under a new minor version revision using the current contents of <code>DevelopmentSchemaArn</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpgradePublishedSchemaFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::upgrade_published_schema::UpgradePublishedSchemaOutput,
        crate::operation::upgrade_published_schema::UpgradePublishedSchemaError,
    > for UpgradePublishedSchemaFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::upgrade_published_schema::UpgradePublishedSchemaOutput,
            crate::operation::upgrade_published_schema::UpgradePublishedSchemaError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpgradePublishedSchemaFluentBuilder {
    /// Creates a new `UpgradePublishedSchema`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpgradePublishedSchema as a reference.
    pub fn as_input(&self) -> &crate::operation::upgrade_published_schema::builders::UpgradePublishedSchemaInputBuilder {
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
        crate::operation::upgrade_published_schema::UpgradePublishedSchemaOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::upgrade_published_schema::UpgradePublishedSchemaError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::upgrade_published_schema::UpgradePublishedSchema::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::upgrade_published_schema::UpgradePublishedSchema::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::upgrade_published_schema::UpgradePublishedSchemaOutput,
        crate::operation::upgrade_published_schema::UpgradePublishedSchemaError,
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
    /// <p>The ARN of the development schema with the changes used for the upgrade.</p>
    pub fn development_schema_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.development_schema_arn(input.into());
        self
    }
    /// <p>The ARN of the development schema with the changes used for the upgrade.</p>
    pub fn set_development_schema_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_development_schema_arn(input);
        self
    }
    /// <p>The ARN of the development schema with the changes used for the upgrade.</p>
    pub fn get_development_schema_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_development_schema_arn()
    }
    /// <p>The ARN of the published schema to be upgraded.</p>
    pub fn published_schema_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.published_schema_arn(input.into());
        self
    }
    /// <p>The ARN of the published schema to be upgraded.</p>
    pub fn set_published_schema_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_published_schema_arn(input);
        self
    }
    /// <p>The ARN of the published schema to be upgraded.</p>
    pub fn get_published_schema_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_published_schema_arn()
    }
    /// <p>Identifies the minor version of the published schema that will be created. This parameter is NOT optional.</p>
    pub fn minor_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.minor_version(input.into());
        self
    }
    /// <p>Identifies the minor version of the published schema that will be created. This parameter is NOT optional.</p>
    pub fn set_minor_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_minor_version(input);
        self
    }
    /// <p>Identifies the minor version of the published schema that will be created. This parameter is NOT optional.</p>
    pub fn get_minor_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_minor_version()
    }
    /// <p>Used for testing whether the Development schema provided is backwards compatible, or not, with the publish schema provided by the user to be upgraded. If schema compatibility fails, an exception would be thrown else the call would succeed. This parameter is optional and defaults to false.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Used for testing whether the Development schema provided is backwards compatible, or not, with the publish schema provided by the user to be upgraded. If schema compatibility fails, an exception would be thrown else the call would succeed. This parameter is optional and defaults to false.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Used for testing whether the Development schema provided is backwards compatible, or not, with the publish schema provided by the user to be upgraded. If schema compatibility fails, an exception would be thrown else the call would succeed. This parameter is optional and defaults to false.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
