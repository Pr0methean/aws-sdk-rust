// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_job_template::_describe_job_template_output::DescribeJobTemplateOutputBuilder;

pub use crate::operation::describe_job_template::_describe_job_template_input::DescribeJobTemplateInputBuilder;

impl DescribeJobTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_job_template::DescribeJobTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_job_template::DescribeJobTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_job_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeJobTemplate`.
///
/// <p>Returns information about a job template.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeJobTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_job_template::builders::DescribeJobTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_job_template::DescribeJobTemplateOutput,
        crate::operation::describe_job_template::DescribeJobTemplateError,
    > for DescribeJobTemplateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_job_template::DescribeJobTemplateOutput,
            crate::operation::describe_job_template::DescribeJobTemplateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeJobTemplateFluentBuilder {
    /// Creates a new `DescribeJobTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeJobTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_job_template::builders::DescribeJobTemplateInputBuilder {
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
        crate::operation::describe_job_template::DescribeJobTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_job_template::DescribeJobTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_job_template::DescribeJobTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_job_template::DescribeJobTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_job_template::DescribeJobTemplateOutput,
        crate::operation::describe_job_template::DescribeJobTemplateError,
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
    /// <p>The unique identifier of the job template.</p>
    pub fn job_template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_template_id(input.into());
        self
    }
    /// <p>The unique identifier of the job template.</p>
    pub fn set_job_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_template_id(input);
        self
    }
    /// <p>The unique identifier of the job template.</p>
    pub fn get_job_template_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_job_template_id()
    }
}
