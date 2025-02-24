// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_graphql_api_environment_variables::_put_graphql_api_environment_variables_output::PutGraphqlApiEnvironmentVariablesOutputBuilder;

pub use crate::operation::put_graphql_api_environment_variables::_put_graphql_api_environment_variables_input::PutGraphqlApiEnvironmentVariablesInputBuilder;

impl PutGraphqlApiEnvironmentVariablesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariablesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariablesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_graphql_api_environment_variables();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutGraphqlApiEnvironmentVariables`.
///
/// <p>Creates a list of environmental variables in an API by its ID value.</p>
/// <p>When creating an environmental variable, it must follow the constraints below:</p>
/// <ul>
/// <li>
/// <p>Both JavaScript and VTL templates support environmental variables.</p></li>
/// <li>
/// <p>Environmental variables are not evaluated before function invocation.</p></li>
/// <li>
/// <p>Environmental variables only support string values.</p></li>
/// <li>
/// <p>Any defined value in an environmental variable is considered a string literal and not expanded.</p></li>
/// <li>
/// <p>Variable evaluations should ideally be performed in the function code.</p></li>
/// </ul>
/// <p>When creating an environmental variable key-value pair, it must follow the additional constraints below:</p>
/// <ul>
/// <li>
/// <p>Keys must begin with a letter.</p></li>
/// <li>
/// <p>Keys must be at least two characters long.</p></li>
/// <li>
/// <p>Keys can only contain letters, numbers, and the underscore character (_).</p></li>
/// <li>
/// <p>Values can be up to 512 characters long.</p></li>
/// <li>
/// <p>You can configure up to 50 key-value pairs in a GraphQL API.</p></li>
/// </ul>
/// <p>You can create a list of environmental variables by adding it to the <code>environmentVariables</code> payload as a list in the format <code>{"key1":"value1","key2":"value2", …}</code>. Note that each call of the <code>PutGraphqlApiEnvironmentVariables</code> action will result in the overwriting of the existing environmental variable list of that API. This means the existing environmental variables will be lost. To avoid this, you must include all existing and new environmental variables in the list each time you call this action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutGraphqlApiEnvironmentVariablesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_graphql_api_environment_variables::builders::PutGraphqlApiEnvironmentVariablesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariablesOutput,
        crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariablesError,
    > for PutGraphqlApiEnvironmentVariablesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariablesOutput,
            crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariablesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutGraphqlApiEnvironmentVariablesFluentBuilder {
    /// Creates a new `PutGraphqlApiEnvironmentVariables`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutGraphqlApiEnvironmentVariables as a reference.
    pub fn as_input(&self) -> &crate::operation::put_graphql_api_environment_variables::builders::PutGraphqlApiEnvironmentVariablesInputBuilder {
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
        crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariablesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariablesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariables::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariables::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariablesOutput,
        crate::operation::put_graphql_api_environment_variables::PutGraphqlApiEnvironmentVariablesError,
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
    /// <p>The ID of the API to which the environmental variable list will be written.</p>
    pub fn api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.api_id(input.into());
        self
    }
    /// <p>The ID of the API to which the environmental variable list will be written.</p>
    pub fn set_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_api_id(input);
        self
    }
    /// <p>The ID of the API to which the environmental variable list will be written.</p>
    pub fn get_api_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_api_id()
    }
    /// Adds a key-value pair to `environmentVariables`.
    ///
    /// To override the contents of this collection use [`set_environment_variables`](Self::set_environment_variables).
    ///
    /// <p>The list of environmental variables to add to the API.</p>
    /// <p>When creating an environmental variable key-value pair, it must follow the additional constraints below:</p>
    /// <ul>
    /// <li>
    /// <p>Keys must begin with a letter.</p></li>
    /// <li>
    /// <p>Keys must be at least two characters long.</p></li>
    /// <li>
    /// <p>Keys can only contain letters, numbers, and the underscore character (_).</p></li>
    /// <li>
    /// <p>Values can be up to 512 characters long.</p></li>
    /// <li>
    /// <p>You can configure up to 50 key-value pairs in a GraphQL API.</p></li>
    /// </ul>
    /// <p>You can create a list of environmental variables by adding it to the <code>environmentVariables</code> payload as a list in the format <code>{"key1":"value1","key2":"value2", …}</code>. Note that each call of the <code>PutGraphqlApiEnvironmentVariables</code> action will result in the overwriting of the existing environmental variable list of that API. This means the existing environmental variables will be lost. To avoid this, you must include all existing and new environmental variables in the list each time you call this action.</p>
    pub fn environment_variables(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.environment_variables(k.into(), v.into());
        self
    }
    /// <p>The list of environmental variables to add to the API.</p>
    /// <p>When creating an environmental variable key-value pair, it must follow the additional constraints below:</p>
    /// <ul>
    /// <li>
    /// <p>Keys must begin with a letter.</p></li>
    /// <li>
    /// <p>Keys must be at least two characters long.</p></li>
    /// <li>
    /// <p>Keys can only contain letters, numbers, and the underscore character (_).</p></li>
    /// <li>
    /// <p>Values can be up to 512 characters long.</p></li>
    /// <li>
    /// <p>You can configure up to 50 key-value pairs in a GraphQL API.</p></li>
    /// </ul>
    /// <p>You can create a list of environmental variables by adding it to the <code>environmentVariables</code> payload as a list in the format <code>{"key1":"value1","key2":"value2", …}</code>. Note that each call of the <code>PutGraphqlApiEnvironmentVariables</code> action will result in the overwriting of the existing environmental variable list of that API. This means the existing environmental variables will be lost. To avoid this, you must include all existing and new environmental variables in the list each time you call this action.</p>
    pub fn set_environment_variables(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_environment_variables(input);
        self
    }
    /// <p>The list of environmental variables to add to the API.</p>
    /// <p>When creating an environmental variable key-value pair, it must follow the additional constraints below:</p>
    /// <ul>
    /// <li>
    /// <p>Keys must begin with a letter.</p></li>
    /// <li>
    /// <p>Keys must be at least two characters long.</p></li>
    /// <li>
    /// <p>Keys can only contain letters, numbers, and the underscore character (_).</p></li>
    /// <li>
    /// <p>Values can be up to 512 characters long.</p></li>
    /// <li>
    /// <p>You can configure up to 50 key-value pairs in a GraphQL API.</p></li>
    /// </ul>
    /// <p>You can create a list of environmental variables by adding it to the <code>environmentVariables</code> payload as a list in the format <code>{"key1":"value1","key2":"value2", …}</code>. Note that each call of the <code>PutGraphqlApiEnvironmentVariables</code> action will result in the overwriting of the existing environmental variable list of that API. This means the existing environmental variables will be lost. To avoid this, you must include all existing and new environmental variables in the list each time you call this action.</p>
    pub fn get_environment_variables(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_environment_variables()
    }
}
