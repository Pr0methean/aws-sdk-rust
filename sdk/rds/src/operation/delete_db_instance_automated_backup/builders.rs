// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_db_instance_automated_backup::_delete_db_instance_automated_backup_output::DeleteDbInstanceAutomatedBackupOutputBuilder;

pub use crate::operation::delete_db_instance_automated_backup::_delete_db_instance_automated_backup_input::DeleteDbInstanceAutomatedBackupInputBuilder;

impl DeleteDbInstanceAutomatedBackupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_db_instance_automated_backup::DeleteDbInstanceAutomatedBackupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_db_instance_automated_backup::DeleteDBInstanceAutomatedBackupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_db_instance_automated_backup();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteDBInstanceAutomatedBackup`.
///
/// <p>Deletes automated backups using the <code>DbiResourceId</code> value of the source DB instance or the Amazon Resource Name (ARN) of the automated backups.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDBInstanceAutomatedBackupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_db_instance_automated_backup::builders::DeleteDbInstanceAutomatedBackupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_db_instance_automated_backup::DeleteDbInstanceAutomatedBackupOutput,
        crate::operation::delete_db_instance_automated_backup::DeleteDBInstanceAutomatedBackupError,
    > for DeleteDBInstanceAutomatedBackupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_db_instance_automated_backup::DeleteDbInstanceAutomatedBackupOutput,
            crate::operation::delete_db_instance_automated_backup::DeleteDBInstanceAutomatedBackupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteDBInstanceAutomatedBackupFluentBuilder {
    /// Creates a new `DeleteDBInstanceAutomatedBackup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteDBInstanceAutomatedBackup as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_db_instance_automated_backup::builders::DeleteDbInstanceAutomatedBackupInputBuilder {
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
        crate::operation::delete_db_instance_automated_backup::DeleteDbInstanceAutomatedBackupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_db_instance_automated_backup::DeleteDBInstanceAutomatedBackupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_db_instance_automated_backup::DeleteDBInstanceAutomatedBackup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_db_instance_automated_backup::DeleteDBInstanceAutomatedBackup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_db_instance_automated_backup::DeleteDbInstanceAutomatedBackupOutput,
        crate::operation::delete_db_instance_automated_backup::DeleteDBInstanceAutomatedBackupError,
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
    /// <p>The identifier for the source DB instance, which can't be changed and which is unique to an Amazon Web Services Region.</p>
    pub fn dbi_resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dbi_resource_id(input.into());
        self
    }
    /// <p>The identifier for the source DB instance, which can't be changed and which is unique to an Amazon Web Services Region.</p>
    pub fn set_dbi_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dbi_resource_id(input);
        self
    }
    /// <p>The identifier for the source DB instance, which can't be changed and which is unique to an Amazon Web Services Region.</p>
    pub fn get_dbi_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dbi_resource_id()
    }
    /// <p>The Amazon Resource Name (ARN) of the automated backups to delete, for example, <code>arn:aws:rds:us-east-1:123456789012:auto-backup:ab-L2IJCEXJP7XQ7HOJ4SIEXAMPLE</code>.</p>
    /// <p>This setting doesn't apply to RDS Custom.</p>
    pub fn db_instance_automated_backups_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_instance_automated_backups_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the automated backups to delete, for example, <code>arn:aws:rds:us-east-1:123456789012:auto-backup:ab-L2IJCEXJP7XQ7HOJ4SIEXAMPLE</code>.</p>
    /// <p>This setting doesn't apply to RDS Custom.</p>
    pub fn set_db_instance_automated_backups_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_instance_automated_backups_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the automated backups to delete, for example, <code>arn:aws:rds:us-east-1:123456789012:auto-backup:ab-L2IJCEXJP7XQ7HOJ4SIEXAMPLE</code>.</p>
    /// <p>This setting doesn't apply to RDS Custom.</p>
    pub fn get_db_instance_automated_backups_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_instance_automated_backups_arn()
    }
}
