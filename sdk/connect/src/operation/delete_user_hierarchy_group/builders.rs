// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_user_hierarchy_group::_delete_user_hierarchy_group_output::DeleteUserHierarchyGroupOutputBuilder;

pub use crate::operation::delete_user_hierarchy_group::_delete_user_hierarchy_group_input::DeleteUserHierarchyGroupInputBuilder;

impl DeleteUserHierarchyGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_user_hierarchy_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteUserHierarchyGroup`.
///
/// <p>Deletes an existing user hierarchy group. It must not be associated with any agents or have any active child groups.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteUserHierarchyGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_user_hierarchy_group::builders::DeleteUserHierarchyGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupOutput,
        crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupError,
    > for DeleteUserHierarchyGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupOutput,
            crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteUserHierarchyGroupFluentBuilder {
    /// Creates a new `DeleteUserHierarchyGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteUserHierarchyGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_user_hierarchy_group::builders::DeleteUserHierarchyGroupInputBuilder {
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
        crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupOutput,
        crate::operation::delete_user_hierarchy_group::DeleteUserHierarchyGroupError,
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
    /// <p>The identifier of the hierarchy group.</p>
    pub fn hierarchy_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.hierarchy_group_id(input.into());
        self
    }
    /// <p>The identifier of the hierarchy group.</p>
    pub fn set_hierarchy_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_hierarchy_group_id(input);
        self
    }
    /// <p>The identifier of the hierarchy group.</p>
    pub fn get_hierarchy_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_hierarchy_group_id()
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
}
