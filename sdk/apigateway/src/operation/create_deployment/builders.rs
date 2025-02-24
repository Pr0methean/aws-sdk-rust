// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_deployment::_create_deployment_output::CreateDeploymentOutputBuilder;

pub use crate::operation::create_deployment::_create_deployment_input::CreateDeploymentInputBuilder;

impl CreateDeploymentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_deployment::CreateDeploymentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_deployment::CreateDeploymentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_deployment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDeployment`.
///
/// <p>Creates a Deployment resource, which makes a specified RestApi callable over the internet.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDeploymentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_deployment::builders::CreateDeploymentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_deployment::CreateDeploymentOutput,
        crate::operation::create_deployment::CreateDeploymentError,
    > for CreateDeploymentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_deployment::CreateDeploymentOutput,
            crate::operation::create_deployment::CreateDeploymentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDeploymentFluentBuilder {
    /// Creates a new `CreateDeployment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDeployment as a reference.
    pub fn as_input(&self) -> &crate::operation::create_deployment::builders::CreateDeploymentInputBuilder {
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
        crate::operation::create_deployment::CreateDeploymentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_deployment::CreateDeploymentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_deployment::CreateDeployment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_deployment::CreateDeployment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_deployment::CreateDeploymentOutput,
        crate::operation::create_deployment::CreateDeploymentError,
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
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn rest_api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rest_api_id(input.into());
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn set_rest_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rest_api_id(input);
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn get_rest_api_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rest_api_id()
    }
    /// <p>The name of the Stage resource for the Deployment resource to create.</p>
    pub fn stage_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stage_name(input.into());
        self
    }
    /// <p>The name of the Stage resource for the Deployment resource to create.</p>
    pub fn set_stage_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stage_name(input);
        self
    }
    /// <p>The name of the Stage resource for the Deployment resource to create.</p>
    pub fn get_stage_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stage_name()
    }
    /// <p>The description of the Stage resource for the Deployment resource to create.</p>
    pub fn stage_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stage_description(input.into());
        self
    }
    /// <p>The description of the Stage resource for the Deployment resource to create.</p>
    pub fn set_stage_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stage_description(input);
        self
    }
    /// <p>The description of the Stage resource for the Deployment resource to create.</p>
    pub fn get_stage_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stage_description()
    }
    /// <p>The description for the Deployment resource to create.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description for the Deployment resource to create.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description for the Deployment resource to create.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>Enables a cache cluster for the Stage resource specified in the input.</p>
    pub fn cache_cluster_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.cache_cluster_enabled(input);
        self
    }
    /// <p>Enables a cache cluster for the Stage resource specified in the input.</p>
    pub fn set_cache_cluster_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_cache_cluster_enabled(input);
        self
    }
    /// <p>Enables a cache cluster for the Stage resource specified in the input.</p>
    pub fn get_cache_cluster_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_cache_cluster_enabled()
    }
    /// <p>The stage's cache capacity in GB. For more information about choosing a cache size, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-caching.html">Enabling API caching to enhance responsiveness</a>.</p>
    pub fn cache_cluster_size(mut self, input: crate::types::CacheClusterSize) -> Self {
        self.inner = self.inner.cache_cluster_size(input);
        self
    }
    /// <p>The stage's cache capacity in GB. For more information about choosing a cache size, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-caching.html">Enabling API caching to enhance responsiveness</a>.</p>
    pub fn set_cache_cluster_size(mut self, input: ::std::option::Option<crate::types::CacheClusterSize>) -> Self {
        self.inner = self.inner.set_cache_cluster_size(input);
        self
    }
    /// <p>The stage's cache capacity in GB. For more information about choosing a cache size, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-caching.html">Enabling API caching to enhance responsiveness</a>.</p>
    pub fn get_cache_cluster_size(&self) -> &::std::option::Option<crate::types::CacheClusterSize> {
        self.inner.get_cache_cluster_size()
    }
    /// Adds a key-value pair to `variables`.
    ///
    /// To override the contents of this collection use [`set_variables`](Self::set_variables).
    ///
    /// <p>A map that defines the stage variables for the Stage resource that is associated with the new deployment. Variable names can have alphanumeric and underscore characters, and the values must match <code>[A-Za-z0-9-._~:/?#&amp;=,]+</code>.</p>
    pub fn variables(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.variables(k.into(), v.into());
        self
    }
    /// <p>A map that defines the stage variables for the Stage resource that is associated with the new deployment. Variable names can have alphanumeric and underscore characters, and the values must match <code>[A-Za-z0-9-._~:/?#&amp;=,]+</code>.</p>
    pub fn set_variables(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_variables(input);
        self
    }
    /// <p>A map that defines the stage variables for the Stage resource that is associated with the new deployment. Variable names can have alphanumeric and underscore characters, and the values must match <code>[A-Za-z0-9-._~:/?#&amp;=,]+</code>.</p>
    pub fn get_variables(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_variables()
    }
    /// <p>The input configuration for the canary deployment when the deployment is a canary release deployment.</p>
    pub fn canary_settings(mut self, input: crate::types::DeploymentCanarySettings) -> Self {
        self.inner = self.inner.canary_settings(input);
        self
    }
    /// <p>The input configuration for the canary deployment when the deployment is a canary release deployment.</p>
    pub fn set_canary_settings(mut self, input: ::std::option::Option<crate::types::DeploymentCanarySettings>) -> Self {
        self.inner = self.inner.set_canary_settings(input);
        self
    }
    /// <p>The input configuration for the canary deployment when the deployment is a canary release deployment.</p>
    pub fn get_canary_settings(&self) -> &::std::option::Option<crate::types::DeploymentCanarySettings> {
        self.inner.get_canary_settings()
    }
    /// <p>Specifies whether active tracing with X-ray is enabled for the Stage.</p>
    pub fn tracing_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.tracing_enabled(input);
        self
    }
    /// <p>Specifies whether active tracing with X-ray is enabled for the Stage.</p>
    pub fn set_tracing_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_tracing_enabled(input);
        self
    }
    /// <p>Specifies whether active tracing with X-ray is enabled for the Stage.</p>
    pub fn get_tracing_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_tracing_enabled()
    }
}
