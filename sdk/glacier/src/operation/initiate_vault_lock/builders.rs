// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::initiate_vault_lock::_initiate_vault_lock_output::InitiateVaultLockOutputBuilder;

pub use crate::operation::initiate_vault_lock::_initiate_vault_lock_input::InitiateVaultLockInputBuilder;

impl InitiateVaultLockInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::initiate_vault_lock::InitiateVaultLockOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::initiate_vault_lock::InitiateVaultLockError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.initiate_vault_lock();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `InitiateVaultLock`.
///
/// <p>This operation initiates the vault locking process by doing the following:</p>
/// <ul>
/// <li>
/// <p>Installing a vault lock policy on the specified vault.</p></li>
/// <li>
/// <p>Setting the lock state of vault lock to <code>InProgress</code>.</p></li>
/// <li>
/// <p>Returning a lock ID, which is used to complete the vault locking process.</p></li>
/// </ul>
/// <p>You can set one vault lock policy for each vault and this policy can be up to 20 KB in size. For more information about vault lock policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>.</p>
/// <p>You must complete the vault locking process within 24 hours after the vault lock enters the <code>InProgress</code> state. After the 24 hour window ends, the lock ID expires, the vault automatically exits the <code>InProgress</code> state, and the vault lock policy is removed from the vault. You call <code>CompleteVaultLock</code> to complete the vault locking process by setting the state of the vault lock to <code>Locked</code>.</p>
/// <p>After a vault lock is in the <code>Locked</code> state, you cannot initiate a new vault lock for the vault.</p>
/// <p>You can abort the vault locking process by calling <code>AbortVaultLock</code>. You can get the state of the vault lock by calling <code>GetVaultLock</code>. For more information about the vault locking process, <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>.</p>
/// <p>If this operation is called when the vault lock is in the <code>InProgress</code> state, the operation returns an <code>AccessDeniedException</code> error. When the vault lock is in the <code>InProgress</code> state you must call <code>AbortVaultLock</code> before you can initiate a new vault lock policy.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct InitiateVaultLockFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::initiate_vault_lock::builders::InitiateVaultLockInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::initiate_vault_lock::InitiateVaultLockOutput,
        crate::operation::initiate_vault_lock::InitiateVaultLockError,
    > for InitiateVaultLockFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::initiate_vault_lock::InitiateVaultLockOutput,
            crate::operation::initiate_vault_lock::InitiateVaultLockError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl InitiateVaultLockFluentBuilder {
    /// Creates a new `InitiateVaultLock`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the InitiateVaultLock as a reference.
    pub fn as_input(&self) -> &crate::operation::initiate_vault_lock::builders::InitiateVaultLockInputBuilder {
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
        crate::operation::initiate_vault_lock::InitiateVaultLockOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::initiate_vault_lock::InitiateVaultLockError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::initiate_vault_lock::InitiateVaultLock::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::initiate_vault_lock::InitiateVaultLock::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::initiate_vault_lock::InitiateVaultLockOutput,
        crate::operation::initiate_vault_lock::InitiateVaultLockError,
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
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vault_name(input.into());
        self
    }
    /// <p>The name of the vault.</p>
    pub fn set_vault_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vault_name(input);
        self
    }
    /// <p>The name of the vault.</p>
    pub fn get_vault_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vault_name()
    }
    /// <p>The vault lock policy as a JSON string, which uses "\" as an escape character.</p>
    pub fn policy(mut self, input: crate::types::VaultLockPolicy) -> Self {
        self.inner = self.inner.policy(input);
        self
    }
    /// <p>The vault lock policy as a JSON string, which uses "\" as an escape character.</p>
    pub fn set_policy(mut self, input: ::std::option::Option<crate::types::VaultLockPolicy>) -> Self {
        self.inner = self.inner.set_policy(input);
        self
    }
    /// <p>The vault lock policy as a JSON string, which uses "\" as an escape character.</p>
    pub fn get_policy(&self) -> &::std::option::Option<crate::types::VaultLockPolicy> {
        self.inner.get_policy()
    }
}
