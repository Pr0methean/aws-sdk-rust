// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_connectivity_info::_update_connectivity_info_output::UpdateConnectivityInfoOutputBuilder;

pub use crate::operation::update_connectivity_info::_update_connectivity_info_input::UpdateConnectivityInfoInputBuilder;

impl UpdateConnectivityInfoInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_connectivity_info::UpdateConnectivityInfoOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_connectivity_info::UpdateConnectivityInfoError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_connectivity_info();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateConnectivityInfo`.
///
/// <p>Updates connectivity information for a Greengrass core device.</p>
/// <p>Connectivity information includes endpoints and ports where client devices can connect to an MQTT broker on the core device. When a client device calls the <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/greengrass-discover-api.html">IoT Greengrass discovery API</a>, IoT Greengrass returns connectivity information for all of the core devices where the client device can connect. For more information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/connect-client-devices.html">Connect client devices to core devices</a> in the <i>IoT Greengrass Version 2 Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateConnectivityInfoFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_connectivity_info::builders::UpdateConnectivityInfoInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_connectivity_info::UpdateConnectivityInfoOutput,
        crate::operation::update_connectivity_info::UpdateConnectivityInfoError,
    > for UpdateConnectivityInfoFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_connectivity_info::UpdateConnectivityInfoOutput,
            crate::operation::update_connectivity_info::UpdateConnectivityInfoError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateConnectivityInfoFluentBuilder {
    /// Creates a new `UpdateConnectivityInfo`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateConnectivityInfo as a reference.
    pub fn as_input(&self) -> &crate::operation::update_connectivity_info::builders::UpdateConnectivityInfoInputBuilder {
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
        crate::operation::update_connectivity_info::UpdateConnectivityInfoOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_connectivity_info::UpdateConnectivityInfoError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_connectivity_info::UpdateConnectivityInfo::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_connectivity_info::UpdateConnectivityInfo::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_connectivity_info::UpdateConnectivityInfoOutput,
        crate::operation::update_connectivity_info::UpdateConnectivityInfoError,
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
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    pub fn thing_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.thing_name(input.into());
        self
    }
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    pub fn set_thing_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_thing_name(input);
        self
    }
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    pub fn get_thing_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_thing_name()
    }
    /// Appends an item to `connectivityInfo`.
    ///
    /// To override the contents of this collection use [`set_connectivity_info`](Self::set_connectivity_info).
    ///
    /// <p>The connectivity information for the core device.</p>
    pub fn connectivity_info(mut self, input: crate::types::ConnectivityInfo) -> Self {
        self.inner = self.inner.connectivity_info(input);
        self
    }
    /// <p>The connectivity information for the core device.</p>
    pub fn set_connectivity_info(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ConnectivityInfo>>) -> Self {
        self.inner = self.inner.set_connectivity_info(input);
        self
    }
    /// <p>The connectivity information for the core device.</p>
    pub fn get_connectivity_info(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ConnectivityInfo>> {
        self.inner.get_connectivity_info()
    }
}
