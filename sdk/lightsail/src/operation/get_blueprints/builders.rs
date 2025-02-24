// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_blueprints::_get_blueprints_output::GetBlueprintsOutputBuilder;

pub use crate::operation::get_blueprints::_get_blueprints_input::GetBlueprintsInputBuilder;

impl GetBlueprintsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_blueprints::GetBlueprintsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_blueprints::GetBlueprintsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_blueprints();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetBlueprints`.
///
/// <p>Returns the list of available instance images, or <i>blueprints</i>. You can use a blueprint to create a new instance already running a specific operating system, as well as a preinstalled app or development stack. The software each instance is running depends on the blueprint image you choose.</p><note>
/// <p>Use active blueprints when creating new instances. Inactive blueprints are listed to support customers with existing instances and are not necessarily available to create new instances. Blueprints are marked inactive when they become outdated due to operating system updates or new application releases.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetBlueprintsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_blueprints::builders::GetBlueprintsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_blueprints::GetBlueprintsOutput,
        crate::operation::get_blueprints::GetBlueprintsError,
    > for GetBlueprintsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_blueprints::GetBlueprintsOutput,
            crate::operation::get_blueprints::GetBlueprintsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetBlueprintsFluentBuilder {
    /// Creates a new `GetBlueprints`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetBlueprints as a reference.
    pub fn as_input(&self) -> &crate::operation::get_blueprints::builders::GetBlueprintsInputBuilder {
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
        crate::operation::get_blueprints::GetBlueprintsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_blueprints::GetBlueprintsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_blueprints::GetBlueprints::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_blueprints::GetBlueprints::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_blueprints::GetBlueprintsOutput,
        crate::operation::get_blueprints::GetBlueprintsError,
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
    /// <p>A Boolean value that indicates whether to include inactive (unavailable) blueprints in the response of your request.</p>
    pub fn include_inactive(mut self, input: bool) -> Self {
        self.inner = self.inner.include_inactive(input);
        self
    }
    /// <p>A Boolean value that indicates whether to include inactive (unavailable) blueprints in the response of your request.</p>
    pub fn set_include_inactive(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_inactive(input);
        self
    }
    /// <p>A Boolean value that indicates whether to include inactive (unavailable) blueprints in the response of your request.</p>
    pub fn get_include_inactive(&self) -> &::std::option::Option<bool> {
        self.inner.get_include_inactive()
    }
    /// <p>The token to advance to the next page of results from your request.</p>
    /// <p>To get a page token, perform an initial <code>GetBlueprints</code> request. If your results are paginated, the response will return a next page token that you can specify as the page token in a subsequent request.</p>
    pub fn page_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.page_token(input.into());
        self
    }
    /// <p>The token to advance to the next page of results from your request.</p>
    /// <p>To get a page token, perform an initial <code>GetBlueprints</code> request. If your results are paginated, the response will return a next page token that you can specify as the page token in a subsequent request.</p>
    pub fn set_page_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_page_token(input);
        self
    }
    /// <p>The token to advance to the next page of results from your request.</p>
    /// <p>To get a page token, perform an initial <code>GetBlueprints</code> request. If your results are paginated, the response will return a next page token that you can specify as the page token in a subsequent request.</p>
    pub fn get_page_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_page_token()
    }
    /// <p>Returns a list of blueprints that are specific to Lightsail for Research.</p><important>
    /// <p>You must use this parameter to view Lightsail for Research blueprints.</p>
    /// </important>
    pub fn app_category(mut self, input: crate::types::AppCategory) -> Self {
        self.inner = self.inner.app_category(input);
        self
    }
    /// <p>Returns a list of blueprints that are specific to Lightsail for Research.</p><important>
    /// <p>You must use this parameter to view Lightsail for Research blueprints.</p>
    /// </important>
    pub fn set_app_category(mut self, input: ::std::option::Option<crate::types::AppCategory>) -> Self {
        self.inner = self.inner.set_app_category(input);
        self
    }
    /// <p>Returns a list of blueprints that are specific to Lightsail for Research.</p><important>
    /// <p>You must use this parameter to view Lightsail for Research blueprints.</p>
    /// </important>
    pub fn get_app_category(&self) -> &::std::option::Option<crate::types::AppCategory> {
        self.inner.get_app_category()
    }
}
