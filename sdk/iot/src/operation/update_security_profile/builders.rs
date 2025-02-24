// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_security_profile::_update_security_profile_output::UpdateSecurityProfileOutputBuilder;

pub use crate::operation::update_security_profile::_update_security_profile_input::UpdateSecurityProfileInputBuilder;

impl UpdateSecurityProfileInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_security_profile::UpdateSecurityProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_security_profile::UpdateSecurityProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_security_profile();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateSecurityProfile`.
///
/// <p>Updates a Device Defender security profile.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">UpdateSecurityProfile</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSecurityProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_security_profile::builders::UpdateSecurityProfileInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_security_profile::UpdateSecurityProfileOutput,
        crate::operation::update_security_profile::UpdateSecurityProfileError,
    > for UpdateSecurityProfileFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_security_profile::UpdateSecurityProfileOutput,
            crate::operation::update_security_profile::UpdateSecurityProfileError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateSecurityProfileFluentBuilder {
    /// Creates a new `UpdateSecurityProfile`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateSecurityProfile as a reference.
    pub fn as_input(&self) -> &crate::operation::update_security_profile::builders::UpdateSecurityProfileInputBuilder {
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
        crate::operation::update_security_profile::UpdateSecurityProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_security_profile::UpdateSecurityProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_security_profile::UpdateSecurityProfile::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_security_profile::UpdateSecurityProfile::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_security_profile::UpdateSecurityProfileOutput,
        crate::operation::update_security_profile::UpdateSecurityProfileError,
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
    /// <p>The name of the security profile you want to update.</p>
    pub fn security_profile_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.security_profile_name(input.into());
        self
    }
    /// <p>The name of the security profile you want to update.</p>
    pub fn set_security_profile_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_security_profile_name(input);
        self
    }
    /// <p>The name of the security profile you want to update.</p>
    pub fn get_security_profile_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_security_profile_name()
    }
    /// <p>A description of the security profile.</p>
    pub fn security_profile_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.security_profile_description(input.into());
        self
    }
    /// <p>A description of the security profile.</p>
    pub fn set_security_profile_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_security_profile_description(input);
        self
    }
    /// <p>A description of the security profile.</p>
    pub fn get_security_profile_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_security_profile_description()
    }
    /// Appends an item to `behaviors`.
    ///
    /// To override the contents of this collection use [`set_behaviors`](Self::set_behaviors).
    ///
    /// <p>Specifies the behaviors that, when violated by a device (thing), cause an alert.</p>
    pub fn behaviors(mut self, input: crate::types::Behavior) -> Self {
        self.inner = self.inner.behaviors(input);
        self
    }
    /// <p>Specifies the behaviors that, when violated by a device (thing), cause an alert.</p>
    pub fn set_behaviors(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Behavior>>) -> Self {
        self.inner = self.inner.set_behaviors(input);
        self
    }
    /// <p>Specifies the behaviors that, when violated by a device (thing), cause an alert.</p>
    pub fn get_behaviors(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Behavior>> {
        self.inner.get_behaviors()
    }
    /// Adds a key-value pair to `alertTargets`.
    ///
    /// To override the contents of this collection use [`set_alert_targets`](Self::set_alert_targets).
    ///
    /// <p>Where the alerts are sent. (Alerts are always sent to the console.)</p>
    pub fn alert_targets(mut self, k: crate::types::AlertTargetType, v: crate::types::AlertTarget) -> Self {
        self.inner = self.inner.alert_targets(k, v);
        self
    }
    /// <p>Where the alerts are sent. (Alerts are always sent to the console.)</p>
    pub fn set_alert_targets(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<crate::types::AlertTargetType, crate::types::AlertTarget>>,
    ) -> Self {
        self.inner = self.inner.set_alert_targets(input);
        self
    }
    /// <p>Where the alerts are sent. (Alerts are always sent to the console.)</p>
    pub fn get_alert_targets(&self) -> &::std::option::Option<::std::collections::HashMap<crate::types::AlertTargetType, crate::types::AlertTarget>> {
        self.inner.get_alert_targets()
    }
    /// Appends an item to `additionalMetricsToRetain`.
    ///
    /// To override the contents of this collection use [`set_additional_metrics_to_retain`](Self::set_additional_metrics_to_retain).
    ///
    /// <p><i>Please use <code>UpdateSecurityProfileRequest$additionalMetricsToRetainV2</code> instead.</i></p>
    /// <p>A list of metrics whose data is retained (stored). By default, data is retained for any metric used in the profile's <code>behaviors</code>, but it is also retained for any metric specified here. Can be used with custom metrics; cannot be used with dimensions.</p>
    #[deprecated(note = "Use additionalMetricsToRetainV2.")]
    pub fn additional_metrics_to_retain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.additional_metrics_to_retain(input.into());
        self
    }
    /// <p><i>Please use <code>UpdateSecurityProfileRequest$additionalMetricsToRetainV2</code> instead.</i></p>
    /// <p>A list of metrics whose data is retained (stored). By default, data is retained for any metric used in the profile's <code>behaviors</code>, but it is also retained for any metric specified here. Can be used with custom metrics; cannot be used with dimensions.</p>
    #[deprecated(note = "Use additionalMetricsToRetainV2.")]
    pub fn set_additional_metrics_to_retain(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_additional_metrics_to_retain(input);
        self
    }
    /// <p><i>Please use <code>UpdateSecurityProfileRequest$additionalMetricsToRetainV2</code> instead.</i></p>
    /// <p>A list of metrics whose data is retained (stored). By default, data is retained for any metric used in the profile's <code>behaviors</code>, but it is also retained for any metric specified here. Can be used with custom metrics; cannot be used with dimensions.</p>
    #[deprecated(note = "Use additionalMetricsToRetainV2.")]
    pub fn get_additional_metrics_to_retain(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_additional_metrics_to_retain()
    }
    /// Appends an item to `additionalMetricsToRetainV2`.
    ///
    /// To override the contents of this collection use [`set_additional_metrics_to_retain_v2`](Self::set_additional_metrics_to_retain_v2).
    ///
    /// <p>A list of metrics whose data is retained (stored). By default, data is retained for any metric used in the profile's behaviors, but it is also retained for any metric specified here. Can be used with custom metrics; cannot be used with dimensions.</p>
    pub fn additional_metrics_to_retain_v2(mut self, input: crate::types::MetricToRetain) -> Self {
        self.inner = self.inner.additional_metrics_to_retain_v2(input);
        self
    }
    /// <p>A list of metrics whose data is retained (stored). By default, data is retained for any metric used in the profile's behaviors, but it is also retained for any metric specified here. Can be used with custom metrics; cannot be used with dimensions.</p>
    pub fn set_additional_metrics_to_retain_v2(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MetricToRetain>>) -> Self {
        self.inner = self.inner.set_additional_metrics_to_retain_v2(input);
        self
    }
    /// <p>A list of metrics whose data is retained (stored). By default, data is retained for any metric used in the profile's behaviors, but it is also retained for any metric specified here. Can be used with custom metrics; cannot be used with dimensions.</p>
    pub fn get_additional_metrics_to_retain_v2(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MetricToRetain>> {
        self.inner.get_additional_metrics_to_retain_v2()
    }
    /// <p>If true, delete all <code>behaviors</code> defined for this security profile. If any <code>behaviors</code> are defined in the current invocation, an exception occurs.</p>
    pub fn delete_behaviors(mut self, input: bool) -> Self {
        self.inner = self.inner.delete_behaviors(input);
        self
    }
    /// <p>If true, delete all <code>behaviors</code> defined for this security profile. If any <code>behaviors</code> are defined in the current invocation, an exception occurs.</p>
    pub fn set_delete_behaviors(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_delete_behaviors(input);
        self
    }
    /// <p>If true, delete all <code>behaviors</code> defined for this security profile. If any <code>behaviors</code> are defined in the current invocation, an exception occurs.</p>
    pub fn get_delete_behaviors(&self) -> &::std::option::Option<bool> {
        self.inner.get_delete_behaviors()
    }
    /// <p>If true, delete all <code>alertTargets</code> defined for this security profile. If any <code>alertTargets</code> are defined in the current invocation, an exception occurs.</p>
    pub fn delete_alert_targets(mut self, input: bool) -> Self {
        self.inner = self.inner.delete_alert_targets(input);
        self
    }
    /// <p>If true, delete all <code>alertTargets</code> defined for this security profile. If any <code>alertTargets</code> are defined in the current invocation, an exception occurs.</p>
    pub fn set_delete_alert_targets(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_delete_alert_targets(input);
        self
    }
    /// <p>If true, delete all <code>alertTargets</code> defined for this security profile. If any <code>alertTargets</code> are defined in the current invocation, an exception occurs.</p>
    pub fn get_delete_alert_targets(&self) -> &::std::option::Option<bool> {
        self.inner.get_delete_alert_targets()
    }
    /// <p>If true, delete all <code>additionalMetricsToRetain</code> defined for this security profile. If any <code>additionalMetricsToRetain</code> are defined in the current invocation, an exception occurs.</p>
    pub fn delete_additional_metrics_to_retain(mut self, input: bool) -> Self {
        self.inner = self.inner.delete_additional_metrics_to_retain(input);
        self
    }
    /// <p>If true, delete all <code>additionalMetricsToRetain</code> defined for this security profile. If any <code>additionalMetricsToRetain</code> are defined in the current invocation, an exception occurs.</p>
    pub fn set_delete_additional_metrics_to_retain(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_delete_additional_metrics_to_retain(input);
        self
    }
    /// <p>If true, delete all <code>additionalMetricsToRetain</code> defined for this security profile. If any <code>additionalMetricsToRetain</code> are defined in the current invocation, an exception occurs.</p>
    pub fn get_delete_additional_metrics_to_retain(&self) -> &::std::option::Option<bool> {
        self.inner.get_delete_additional_metrics_to_retain()
    }
    /// <p>The expected version of the security profile. A new version is generated whenever the security profile is updated. If you specify a value that is different from the actual version, a <code>VersionConflictException</code> is thrown.</p>
    pub fn expected_version(mut self, input: i64) -> Self {
        self.inner = self.inner.expected_version(input);
        self
    }
    /// <p>The expected version of the security profile. A new version is generated whenever the security profile is updated. If you specify a value that is different from the actual version, a <code>VersionConflictException</code> is thrown.</p>
    pub fn set_expected_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_expected_version(input);
        self
    }
    /// <p>The expected version of the security profile. A new version is generated whenever the security profile is updated. If you specify a value that is different from the actual version, a <code>VersionConflictException</code> is thrown.</p>
    pub fn get_expected_version(&self) -> &::std::option::Option<i64> {
        self.inner.get_expected_version()
    }
    /// <p>Specifies the MQTT topic and role ARN required for metric export.</p>
    pub fn metrics_export_config(mut self, input: crate::types::MetricsExportConfig) -> Self {
        self.inner = self.inner.metrics_export_config(input);
        self
    }
    /// <p>Specifies the MQTT topic and role ARN required for metric export.</p>
    pub fn set_metrics_export_config(mut self, input: ::std::option::Option<crate::types::MetricsExportConfig>) -> Self {
        self.inner = self.inner.set_metrics_export_config(input);
        self
    }
    /// <p>Specifies the MQTT topic and role ARN required for metric export.</p>
    pub fn get_metrics_export_config(&self) -> &::std::option::Option<crate::types::MetricsExportConfig> {
        self.inner.get_metrics_export_config()
    }
    /// <p>Set the value as true to delete metrics export related configurations.</p>
    pub fn delete_metrics_export_config(mut self, input: bool) -> Self {
        self.inner = self.inner.delete_metrics_export_config(input);
        self
    }
    /// <p>Set the value as true to delete metrics export related configurations.</p>
    pub fn set_delete_metrics_export_config(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_delete_metrics_export_config(input);
        self
    }
    /// <p>Set the value as true to delete metrics export related configurations.</p>
    pub fn get_delete_metrics_export_config(&self) -> &::std::option::Option<bool> {
        self.inner.get_delete_metrics_export_config()
    }
}
