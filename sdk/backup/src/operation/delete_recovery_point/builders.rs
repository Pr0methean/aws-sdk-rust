// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_recovery_point::_delete_recovery_point_output::DeleteRecoveryPointOutputBuilder;

pub use crate::operation::delete_recovery_point::_delete_recovery_point_input::DeleteRecoveryPointInputBuilder;

impl DeleteRecoveryPointInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_recovery_point::DeleteRecoveryPointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_recovery_point::DeleteRecoveryPointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_recovery_point();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteRecoveryPoint`.
///
/// <p>Deletes the recovery point specified by a recovery point ID.</p>
/// <p>If the recovery point ID belongs to a continuous backup, calling this endpoint deletes the existing continuous backup and stops future continuous backup.</p>
/// <p>When an IAM role's permissions are insufficient to call this API, the service sends back an HTTP 200 response with an empty HTTP body, but the recovery point is not deleted. Instead, it enters an <code>EXPIRED</code> state.</p>
/// <p><code>EXPIRED</code> recovery points can be deleted with this API once the IAM role has the <code>iam:CreateServiceLinkedRole</code> action. To learn more about adding this role, see <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/deleting-backups.html#deleting-backups-troubleshooting"> Troubleshooting manual deletions</a>.</p>
/// <p>If the user or role is deleted or the permission within the role is removed, the deletion will not be successful and will enter an <code>EXPIRED</code> state.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteRecoveryPointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_recovery_point::builders::DeleteRecoveryPointInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_recovery_point::DeleteRecoveryPointOutput,
        crate::operation::delete_recovery_point::DeleteRecoveryPointError,
    > for DeleteRecoveryPointFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_recovery_point::DeleteRecoveryPointOutput,
            crate::operation::delete_recovery_point::DeleteRecoveryPointError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteRecoveryPointFluentBuilder {
    /// Creates a new `DeleteRecoveryPoint`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteRecoveryPoint as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_recovery_point::builders::DeleteRecoveryPointInputBuilder {
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
        crate::operation::delete_recovery_point::DeleteRecoveryPointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_recovery_point::DeleteRecoveryPointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_recovery_point::DeleteRecoveryPoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_recovery_point::DeleteRecoveryPoint::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_recovery_point::DeleteRecoveryPointOutput,
        crate::operation::delete_recovery_point::DeleteRecoveryPointError,
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
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    pub fn backup_vault_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.backup_vault_name(input.into());
        self
    }
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    pub fn set_backup_vault_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_backup_vault_name(input);
        self
    }
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    pub fn get_backup_vault_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_backup_vault_name()
    }
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn recovery_point_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.recovery_point_arn(input.into());
        self
    }
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn set_recovery_point_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_recovery_point_arn(input);
        self
    }
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn get_recovery_point_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_recovery_point_arn()
    }
}
