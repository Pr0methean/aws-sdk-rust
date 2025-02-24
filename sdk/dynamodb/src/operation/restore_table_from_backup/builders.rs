// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::restore_table_from_backup::_restore_table_from_backup_output::RestoreTableFromBackupOutputBuilder;

pub use crate::operation::restore_table_from_backup::_restore_table_from_backup_input::RestoreTableFromBackupInputBuilder;

impl RestoreTableFromBackupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.restore_table_from_backup();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RestoreTableFromBackup`.
///
/// <p>Creates a new table from an existing backup. Any number of users can execute up to 50 concurrent restores (any type of restore) in a given account.</p>
/// <p>You can call <code>RestoreTableFromBackup</code> at a maximum rate of 10 times per second.</p>
/// <p>You must manually set up the following on the restored table:</p>
/// <ul>
/// <li>
/// <p>Auto scaling policies</p></li>
/// <li>
/// <p>IAM policies</p></li>
/// <li>
/// <p>Amazon CloudWatch metrics and alarms</p></li>
/// <li>
/// <p>Tags</p></li>
/// <li>
/// <p>Stream settings</p></li>
/// <li>
/// <p>Time to Live (TTL) settings</p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RestoreTableFromBackupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput,
        crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
    > for RestoreTableFromBackupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput,
            crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RestoreTableFromBackupFluentBuilder {
    /// Creates a new `RestoreTableFromBackup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RestoreTableFromBackup as a reference.
    pub fn as_input(&self) -> &crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupInputBuilder {
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
        crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::restore_table_from_backup::RestoreTableFromBackup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::restore_table_from_backup::RestoreTableFromBackup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput,
        crate::operation::restore_table_from_backup::RestoreTableFromBackupError,
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
    /// <p>The name of the new table to which the backup must be restored.</p>
    pub fn target_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_table_name(input.into());
        self
    }
    /// <p>The name of the new table to which the backup must be restored.</p>
    pub fn set_target_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_table_name(input);
        self
    }
    /// <p>The name of the new table to which the backup must be restored.</p>
    pub fn get_target_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_table_name()
    }
    /// <p>The Amazon Resource Name (ARN) associated with the backup.</p>
    pub fn backup_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.backup_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the backup.</p>
    pub fn set_backup_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_backup_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the backup.</p>
    pub fn get_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_backup_arn()
    }
    /// <p>The billing mode of the restored table.</p>
    pub fn billing_mode_override(mut self, input: crate::types::BillingMode) -> Self {
        self.inner = self.inner.billing_mode_override(input);
        self
    }
    /// <p>The billing mode of the restored table.</p>
    pub fn set_billing_mode_override(mut self, input: ::std::option::Option<crate::types::BillingMode>) -> Self {
        self.inner = self.inner.set_billing_mode_override(input);
        self
    }
    /// <p>The billing mode of the restored table.</p>
    pub fn get_billing_mode_override(&self) -> &::std::option::Option<crate::types::BillingMode> {
        self.inner.get_billing_mode_override()
    }
    /// Appends an item to `GlobalSecondaryIndexOverride`.
    ///
    /// To override the contents of this collection use [`set_global_secondary_index_override`](Self::set_global_secondary_index_override).
    ///
    /// <p>List of global secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    pub fn global_secondary_index_override(mut self, input: crate::types::GlobalSecondaryIndex) -> Self {
        self.inner = self.inner.global_secondary_index_override(input);
        self
    }
    /// <p>List of global secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    pub fn set_global_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::GlobalSecondaryIndex>>) -> Self {
        self.inner = self.inner.set_global_secondary_index_override(input);
        self
    }
    /// <p>List of global secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    pub fn get_global_secondary_index_override(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::GlobalSecondaryIndex>> {
        self.inner.get_global_secondary_index_override()
    }
    /// Appends an item to `LocalSecondaryIndexOverride`.
    ///
    /// To override the contents of this collection use [`set_local_secondary_index_override`](Self::set_local_secondary_index_override).
    ///
    /// <p>List of local secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    pub fn local_secondary_index_override(mut self, input: crate::types::LocalSecondaryIndex) -> Self {
        self.inner = self.inner.local_secondary_index_override(input);
        self
    }
    /// <p>List of local secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    pub fn set_local_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LocalSecondaryIndex>>) -> Self {
        self.inner = self.inner.set_local_secondary_index_override(input);
        self
    }
    /// <p>List of local secondary indexes for the restored table. The indexes provided should match existing secondary indexes. You can choose to exclude some or all of the indexes at the time of restore.</p>
    pub fn get_local_secondary_index_override(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LocalSecondaryIndex>> {
        self.inner.get_local_secondary_index_override()
    }
    /// <p>Provisioned throughput settings for the restored table.</p>
    pub fn provisioned_throughput_override(mut self, input: crate::types::ProvisionedThroughput) -> Self {
        self.inner = self.inner.provisioned_throughput_override(input);
        self
    }
    /// <p>Provisioned throughput settings for the restored table.</p>
    pub fn set_provisioned_throughput_override(mut self, input: ::std::option::Option<crate::types::ProvisionedThroughput>) -> Self {
        self.inner = self.inner.set_provisioned_throughput_override(input);
        self
    }
    /// <p>Provisioned throughput settings for the restored table.</p>
    pub fn get_provisioned_throughput_override(&self) -> &::std::option::Option<crate::types::ProvisionedThroughput> {
        self.inner.get_provisioned_throughput_override()
    }
    /// <p>The new server-side encryption settings for the restored table.</p>
    pub fn sse_specification_override(mut self, input: crate::types::SseSpecification) -> Self {
        self.inner = self.inner.sse_specification_override(input);
        self
    }
    /// <p>The new server-side encryption settings for the restored table.</p>
    pub fn set_sse_specification_override(mut self, input: ::std::option::Option<crate::types::SseSpecification>) -> Self {
        self.inner = self.inner.set_sse_specification_override(input);
        self
    }
    /// <p>The new server-side encryption settings for the restored table.</p>
    pub fn get_sse_specification_override(&self) -> &::std::option::Option<crate::types::SseSpecification> {
        self.inner.get_sse_specification_override()
    }
}
