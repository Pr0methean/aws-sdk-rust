// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::discover_poll_endpoint::_discover_poll_endpoint_output::DiscoverPollEndpointOutputBuilder;

pub use crate::operation::discover_poll_endpoint::_discover_poll_endpoint_input::DiscoverPollEndpointInputBuilder;

impl DiscoverPollEndpointInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::discover_poll_endpoint::DiscoverPollEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::discover_poll_endpoint::DiscoverPollEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.discover_poll_endpoint();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DiscoverPollEndpoint`.
///
/// <note>
/// <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p>
/// </note>
/// <p>Returns an endpoint for the Amazon ECS agent to poll for updates.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DiscoverPollEndpointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::discover_poll_endpoint::builders::DiscoverPollEndpointInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::discover_poll_endpoint::DiscoverPollEndpointOutput,
        crate::operation::discover_poll_endpoint::DiscoverPollEndpointError,
    > for DiscoverPollEndpointFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::discover_poll_endpoint::DiscoverPollEndpointOutput,
            crate::operation::discover_poll_endpoint::DiscoverPollEndpointError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DiscoverPollEndpointFluentBuilder {
    /// Creates a new `DiscoverPollEndpoint`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DiscoverPollEndpoint as a reference.
    pub fn as_input(&self) -> &crate::operation::discover_poll_endpoint::builders::DiscoverPollEndpointInputBuilder {
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
        crate::operation::discover_poll_endpoint::DiscoverPollEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::discover_poll_endpoint::DiscoverPollEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::discover_poll_endpoint::DiscoverPollEndpoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::discover_poll_endpoint::DiscoverPollEndpoint::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::discover_poll_endpoint::DiscoverPollEndpointOutput,
        crate::operation::discover_poll_endpoint::DiscoverPollEndpointError,
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
    /// <p>The container instance ID or full ARN of the container instance. For more information about the ARN format, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html#ecs-resource-ids">Amazon Resource Name (ARN)</a> in the <i>Amazon ECS Developer Guide</i>.</p>
    pub fn container_instance(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.container_instance(input.into());
        self
    }
    /// <p>The container instance ID or full ARN of the container instance. For more information about the ARN format, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html#ecs-resource-ids">Amazon Resource Name (ARN)</a> in the <i>Amazon ECS Developer Guide</i>.</p>
    pub fn set_container_instance(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_container_instance(input);
        self
    }
    /// <p>The container instance ID or full ARN of the container instance. For more information about the ARN format, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html#ecs-resource-ids">Amazon Resource Name (ARN)</a> in the <i>Amazon ECS Developer Guide</i>.</p>
    pub fn get_container_instance(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_container_instance()
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that the container instance belongs to.</p>
    pub fn cluster(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that the container instance belongs to.</p>
    pub fn set_cluster(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster(input);
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that the container instance belongs to.</p>
    pub fn get_cluster(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cluster()
    }
}
