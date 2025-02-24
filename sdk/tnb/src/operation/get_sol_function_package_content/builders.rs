// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_sol_function_package_content::_get_sol_function_package_content_output::GetSolFunctionPackageContentOutputBuilder;

pub use crate::operation::get_sol_function_package_content::_get_sol_function_package_content_input::GetSolFunctionPackageContentInputBuilder;

impl GetSolFunctionPackageContentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_sol_function_package_content::GetSolFunctionPackageContentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_sol_function_package_content::GetSolFunctionPackageContentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_sol_function_package_content();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetSolFunctionPackageContent`.
///
/// <p>Gets the contents of a function package.</p>
/// <p>A function package is a .zip file in CSAR (Cloud Service Archive) format that contains a network function (an ETSI standard telecommunication application) and function package descriptor that uses the TOSCA standard to describe how the network functions should run on your network.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetSolFunctionPackageContentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_sol_function_package_content::builders::GetSolFunctionPackageContentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_sol_function_package_content::GetSolFunctionPackageContentOutput,
        crate::operation::get_sol_function_package_content::GetSolFunctionPackageContentError,
    > for GetSolFunctionPackageContentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_sol_function_package_content::GetSolFunctionPackageContentOutput,
            crate::operation::get_sol_function_package_content::GetSolFunctionPackageContentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetSolFunctionPackageContentFluentBuilder {
    /// Creates a new `GetSolFunctionPackageContent`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetSolFunctionPackageContent as a reference.
    pub fn as_input(&self) -> &crate::operation::get_sol_function_package_content::builders::GetSolFunctionPackageContentInputBuilder {
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
        crate::operation::get_sol_function_package_content::GetSolFunctionPackageContentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_sol_function_package_content::GetSolFunctionPackageContentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_sol_function_package_content::GetSolFunctionPackageContent::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_sol_function_package_content::GetSolFunctionPackageContent::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_sol_function_package_content::GetSolFunctionPackageContentOutput,
        crate::operation::get_sol_function_package_content::GetSolFunctionPackageContentError,
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
    /// <p>ID of the function package.</p>
    pub fn vnf_pkg_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vnf_pkg_id(input.into());
        self
    }
    /// <p>ID of the function package.</p>
    pub fn set_vnf_pkg_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vnf_pkg_id(input);
        self
    }
    /// <p>ID of the function package.</p>
    pub fn get_vnf_pkg_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vnf_pkg_id()
    }
    /// <p>The format of the package that you want to download from the function packages.</p>
    pub fn accept(mut self, input: crate::types::PackageContentType) -> Self {
        self.inner = self.inner.accept(input);
        self
    }
    /// <p>The format of the package that you want to download from the function packages.</p>
    pub fn set_accept(mut self, input: ::std::option::Option<crate::types::PackageContentType>) -> Self {
        self.inner = self.inner.set_accept(input);
        self
    }
    /// <p>The format of the package that you want to download from the function packages.</p>
    pub fn get_accept(&self) -> &::std::option::Option<crate::types::PackageContentType> {
        self.inner.get_accept()
    }
}
