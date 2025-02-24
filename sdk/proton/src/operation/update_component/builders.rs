// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_component::_update_component_output::UpdateComponentOutputBuilder;

pub use crate::operation::update_component::_update_component_input::UpdateComponentInputBuilder;

impl UpdateComponentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_component::UpdateComponentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_component::UpdateComponentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_component();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateComponent`.
///
/// <p>Update a component.</p>
/// <p>There are a few modes for updating a component. The <code>deploymentType</code> field defines the mode.</p><note>
/// <p>You can't update a component while its deployment status, or the deployment status of a service instance attached to it, is <code>IN_PROGRESS</code>.</p>
/// </note>
/// <p>For more information about components, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/ag-components.html">Proton components</a> in the <i>Proton User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateComponentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_component::builders::UpdateComponentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_component::UpdateComponentOutput,
        crate::operation::update_component::UpdateComponentError,
    > for UpdateComponentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_component::UpdateComponentOutput,
            crate::operation::update_component::UpdateComponentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateComponentFluentBuilder {
    /// Creates a new `UpdateComponent`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateComponent as a reference.
    pub fn as_input(&self) -> &crate::operation::update_component::builders::UpdateComponentInputBuilder {
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
        crate::operation::update_component::UpdateComponentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_component::UpdateComponentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_component::UpdateComponent::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_component::UpdateComponent::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_component::UpdateComponentOutput,
        crate::operation::update_component::UpdateComponentError,
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
    /// <p>The name of the component to update.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the component to update.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the component to update.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The deployment type. It defines the mode for updating a component, as follows:</p>
    /// <dl>
    /// <dt></dt>
    /// <dd>
    /// <p><code>NONE</code></p>
    /// <p>In this mode, a deployment <i>doesn't</i> occur. Only the requested metadata parameters are updated. You can only specify <code>description</code> in this mode.</p>
    /// </dd>
    /// <dt></dt>
    /// <dd>
    /// <p><code>CURRENT_VERSION</code></p>
    /// <p>In this mode, the component is deployed and updated with the new <code>serviceSpec</code>, <code>templateSource</code>, and/or <code>type</code> that you provide. Only requested parameters are updated.</p>
    /// </dd>
    /// </dl>
    pub fn deployment_type(mut self, input: crate::types::ComponentDeploymentUpdateType) -> Self {
        self.inner = self.inner.deployment_type(input);
        self
    }
    /// <p>The deployment type. It defines the mode for updating a component, as follows:</p>
    /// <dl>
    /// <dt></dt>
    /// <dd>
    /// <p><code>NONE</code></p>
    /// <p>In this mode, a deployment <i>doesn't</i> occur. Only the requested metadata parameters are updated. You can only specify <code>description</code> in this mode.</p>
    /// </dd>
    /// <dt></dt>
    /// <dd>
    /// <p><code>CURRENT_VERSION</code></p>
    /// <p>In this mode, the component is deployed and updated with the new <code>serviceSpec</code>, <code>templateSource</code>, and/or <code>type</code> that you provide. Only requested parameters are updated.</p>
    /// </dd>
    /// </dl>
    pub fn set_deployment_type(mut self, input: ::std::option::Option<crate::types::ComponentDeploymentUpdateType>) -> Self {
        self.inner = self.inner.set_deployment_type(input);
        self
    }
    /// <p>The deployment type. It defines the mode for updating a component, as follows:</p>
    /// <dl>
    /// <dt></dt>
    /// <dd>
    /// <p><code>NONE</code></p>
    /// <p>In this mode, a deployment <i>doesn't</i> occur. Only the requested metadata parameters are updated. You can only specify <code>description</code> in this mode.</p>
    /// </dd>
    /// <dt></dt>
    /// <dd>
    /// <p><code>CURRENT_VERSION</code></p>
    /// <p>In this mode, the component is deployed and updated with the new <code>serviceSpec</code>, <code>templateSource</code>, and/or <code>type</code> that you provide. Only requested parameters are updated.</p>
    /// </dd>
    /// </dl>
    pub fn get_deployment_type(&self) -> &::std::option::Option<crate::types::ComponentDeploymentUpdateType> {
        self.inner.get_deployment_type()
    }
    /// <p>An optional customer-provided description of the component.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An optional customer-provided description of the component.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>An optional customer-provided description of the component.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The name of the service that <code>serviceInstanceName</code> is associated with. Don't specify to keep the component's current service instance attachment. Specify an empty string to detach the component from the service instance it's attached to. Specify non-empty values for both <code>serviceInstanceName</code> and <code>serviceName</code> or for neither of them.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_name(input.into());
        self
    }
    /// <p>The name of the service that <code>serviceInstanceName</code> is associated with. Don't specify to keep the component's current service instance attachment. Specify an empty string to detach the component from the service instance it's attached to. Specify non-empty values for both <code>serviceInstanceName</code> and <code>serviceName</code> or for neither of them.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_name(input);
        self
    }
    /// <p>The name of the service that <code>serviceInstanceName</code> is associated with. Don't specify to keep the component's current service instance attachment. Specify an empty string to detach the component from the service instance it's attached to. Specify non-empty values for both <code>serviceInstanceName</code> and <code>serviceName</code> or for neither of them.</p>
    pub fn get_service_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_name()
    }
    /// <p>The name of the service instance that you want to attach this component to. Don't specify to keep the component's current service instance attachment. Specify an empty string to detach the component from the service instance it's attached to. Specify non-empty values for both <code>serviceInstanceName</code> and <code>serviceName</code> or for neither of them.</p>
    pub fn service_instance_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_instance_name(input.into());
        self
    }
    /// <p>The name of the service instance that you want to attach this component to. Don't specify to keep the component's current service instance attachment. Specify an empty string to detach the component from the service instance it's attached to. Specify non-empty values for both <code>serviceInstanceName</code> and <code>serviceName</code> or for neither of them.</p>
    pub fn set_service_instance_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_instance_name(input);
        self
    }
    /// <p>The name of the service instance that you want to attach this component to. Don't specify to keep the component's current service instance attachment. Specify an empty string to detach the component from the service instance it's attached to. Specify non-empty values for both <code>serviceInstanceName</code> and <code>serviceName</code> or for neither of them.</p>
    pub fn get_service_instance_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_instance_name()
    }
    /// <p>The service spec that you want the component to use to access service inputs. Set this only when the component is attached to a service instance.</p>
    pub fn service_spec(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_spec(input.into());
        self
    }
    /// <p>The service spec that you want the component to use to access service inputs. Set this only when the component is attached to a service instance.</p>
    pub fn set_service_spec(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_spec(input);
        self
    }
    /// <p>The service spec that you want the component to use to access service inputs. Set this only when the component is attached to a service instance.</p>
    pub fn get_service_spec(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_spec()
    }
    /// <p>A path to the Infrastructure as Code (IaC) file describing infrastructure that a custom component provisions.</p><note>
    /// <p>Components support a single IaC file, even if you use Terraform as your template language.</p>
    /// </note>
    pub fn template_file(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_file(input.into());
        self
    }
    /// <p>A path to the Infrastructure as Code (IaC) file describing infrastructure that a custom component provisions.</p><note>
    /// <p>Components support a single IaC file, even if you use Terraform as your template language.</p>
    /// </note>
    pub fn set_template_file(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_file(input);
        self
    }
    /// <p>A path to the Infrastructure as Code (IaC) file describing infrastructure that a custom component provisions.</p><note>
    /// <p>Components support a single IaC file, even if you use Terraform as your template language.</p>
    /// </note>
    pub fn get_template_file(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_file()
    }
    /// <p>The client token for the updated component.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The client token for the updated component.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The client token for the updated component.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
