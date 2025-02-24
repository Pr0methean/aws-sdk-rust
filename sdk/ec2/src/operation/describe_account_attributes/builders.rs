// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_account_attributes::_describe_account_attributes_output::DescribeAccountAttributesOutputBuilder;

pub use crate::operation::describe_account_attributes::_describe_account_attributes_input::DescribeAccountAttributesInputBuilder;

impl DescribeAccountAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_account_attributes::DescribeAccountAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_account_attributes::DescribeAccountAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_account_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeAccountAttributes`.
///
/// <p>Describes attributes of your Amazon Web Services account. The following are the supported account attributes:</p>
/// <ul>
/// <li>
/// <p><code>default-vpc</code>: The ID of the default VPC for your account, or <code>none</code>.</p></li>
/// <li>
/// <p><code>max-instances</code>: This attribute is no longer supported. The returned value does not reflect your actual vCPU limit for running On-Demand Instances. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-on-demand-instances.html#ec2-on-demand-instances-limits">On-Demand Instance Limits</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p></li>
/// <li>
/// <p><code>max-elastic-ips</code>: The maximum number of Elastic IP addresses that you can allocate.</p></li>
/// <li>
/// <p><code>supported-platforms</code>: This attribute is deprecated.</p></li>
/// <li>
/// <p><code>vpc-max-elastic-ips</code>: The maximum number of Elastic IP addresses that you can allocate.</p></li>
/// <li>
/// <p><code>vpc-max-security-groups-per-interface</code>: The maximum number of security groups that you can assign to a network interface.</p></li>
/// </ul><note>
/// <p>The order of the elements in the response, including those within nested structures, might vary. Applications should not assume the elements appear in a particular order.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeAccountAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_account_attributes::builders::DescribeAccountAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_account_attributes::DescribeAccountAttributesOutput,
        crate::operation::describe_account_attributes::DescribeAccountAttributesError,
    > for DescribeAccountAttributesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_account_attributes::DescribeAccountAttributesOutput,
            crate::operation::describe_account_attributes::DescribeAccountAttributesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeAccountAttributesFluentBuilder {
    /// Creates a new `DescribeAccountAttributes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeAccountAttributes as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_account_attributes::builders::DescribeAccountAttributesInputBuilder {
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
        crate::operation::describe_account_attributes::DescribeAccountAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_account_attributes::DescribeAccountAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_account_attributes::DescribeAccountAttributes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_account_attributes::DescribeAccountAttributes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_account_attributes::DescribeAccountAttributesOutput,
        crate::operation::describe_account_attributes::DescribeAccountAttributesError,
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
    /// Appends an item to `AttributeNames`.
    ///
    /// To override the contents of this collection use [`set_attribute_names`](Self::set_attribute_names).
    ///
    /// <p>The account attribute names.</p>
    pub fn attribute_names(mut self, input: crate::types::AccountAttributeName) -> Self {
        self.inner = self.inner.attribute_names(input);
        self
    }
    /// <p>The account attribute names.</p>
    pub fn set_attribute_names(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AccountAttributeName>>) -> Self {
        self.inner = self.inner.set_attribute_names(input);
        self
    }
    /// <p>The account attribute names.</p>
    pub fn get_attribute_names(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AccountAttributeName>> {
        self.inner.get_attribute_names()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
