// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_detector::_update_detector_output::UpdateDetectorOutputBuilder;

pub use crate::operation::update_detector::_update_detector_input::UpdateDetectorInputBuilder;

impl UpdateDetectorInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_detector::UpdateDetectorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_detector::UpdateDetectorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_detector();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateDetector`.
///
/// <p>Updates the GuardDuty detector specified by the detector ID.</p>
/// <p>Specifying both EKS Runtime Monitoring (<code>EKS_RUNTIME_MONITORING</code>) and Runtime Monitoring (<code>RUNTIME_MONITORING</code>) will cause an error. You can add only one of these two features because Runtime Monitoring already includes the threat detection for Amazon EKS resources. For more information, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/runtime-monitoring.html">Runtime Monitoring</a>.</p>
/// <p>There might be regional differences because some data sources might not be available in all the Amazon Web Services Regions where GuardDuty is presently supported. For more information, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_regions.html">Regions and endpoints</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDetectorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_detector::builders::UpdateDetectorInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_detector::UpdateDetectorOutput,
        crate::operation::update_detector::UpdateDetectorError,
    > for UpdateDetectorFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_detector::UpdateDetectorOutput,
            crate::operation::update_detector::UpdateDetectorError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateDetectorFluentBuilder {
    /// Creates a new `UpdateDetector`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateDetector as a reference.
    pub fn as_input(&self) -> &crate::operation::update_detector::builders::UpdateDetectorInputBuilder {
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
        crate::operation::update_detector::UpdateDetectorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_detector::UpdateDetectorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_detector::UpdateDetector::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_detector::UpdateDetector::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_detector::UpdateDetectorOutput,
        crate::operation::update_detector::UpdateDetectorError,
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
    /// <p>The unique ID of the detector to update.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The unique ID of the detector to update.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The unique ID of the detector to update.</p>
    pub fn get_detector_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_detector_id()
    }
    /// <p>Specifies whether the detector is enabled or not enabled.</p>
    pub fn enable(mut self, input: bool) -> Self {
        self.inner = self.inner.enable(input);
        self
    }
    /// <p>Specifies whether the detector is enabled or not enabled.</p>
    pub fn set_enable(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable(input);
        self
    }
    /// <p>Specifies whether the detector is enabled or not enabled.</p>
    pub fn get_enable(&self) -> &::std::option::Option<bool> {
        self.inner.get_enable()
    }
    /// <p>An enum value that specifies how frequently findings are exported, such as to CloudWatch Events.</p>
    pub fn finding_publishing_frequency(mut self, input: crate::types::FindingPublishingFrequency) -> Self {
        self.inner = self.inner.finding_publishing_frequency(input);
        self
    }
    /// <p>An enum value that specifies how frequently findings are exported, such as to CloudWatch Events.</p>
    pub fn set_finding_publishing_frequency(mut self, input: ::std::option::Option<crate::types::FindingPublishingFrequency>) -> Self {
        self.inner = self.inner.set_finding_publishing_frequency(input);
        self
    }
    /// <p>An enum value that specifies how frequently findings are exported, such as to CloudWatch Events.</p>
    pub fn get_finding_publishing_frequency(&self) -> &::std::option::Option<crate::types::FindingPublishingFrequency> {
        self.inner.get_finding_publishing_frequency()
    }
    /// <p>Describes which data sources will be updated.</p>
    /// <p>There might be regional differences because some data sources might not be available in all the Amazon Web Services Regions where GuardDuty is presently supported. For more information, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_regions.html">Regions and endpoints</a>.</p>
    #[deprecated(note = "This parameter is deprecated, use Features instead")]
    pub fn data_sources(mut self, input: crate::types::DataSourceConfigurations) -> Self {
        self.inner = self.inner.data_sources(input);
        self
    }
    /// <p>Describes which data sources will be updated.</p>
    /// <p>There might be regional differences because some data sources might not be available in all the Amazon Web Services Regions where GuardDuty is presently supported. For more information, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_regions.html">Regions and endpoints</a>.</p>
    #[deprecated(note = "This parameter is deprecated, use Features instead")]
    pub fn set_data_sources(mut self, input: ::std::option::Option<crate::types::DataSourceConfigurations>) -> Self {
        self.inner = self.inner.set_data_sources(input);
        self
    }
    /// <p>Describes which data sources will be updated.</p>
    /// <p>There might be regional differences because some data sources might not be available in all the Amazon Web Services Regions where GuardDuty is presently supported. For more information, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_regions.html">Regions and endpoints</a>.</p>
    #[deprecated(note = "This parameter is deprecated, use Features instead")]
    pub fn get_data_sources(&self) -> &::std::option::Option<crate::types::DataSourceConfigurations> {
        self.inner.get_data_sources()
    }
    /// Appends an item to `Features`.
    ///
    /// To override the contents of this collection use [`set_features`](Self::set_features).
    ///
    /// <p>Provides the features that will be updated for the detector.</p>
    pub fn features(mut self, input: crate::types::DetectorFeatureConfiguration) -> Self {
        self.inner = self.inner.features(input);
        self
    }
    /// <p>Provides the features that will be updated for the detector.</p>
    pub fn set_features(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DetectorFeatureConfiguration>>) -> Self {
        self.inner = self.inner.set_features(input);
        self
    }
    /// <p>Provides the features that will be updated for the detector.</p>
    pub fn get_features(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DetectorFeatureConfiguration>> {
        self.inner.get_features()
    }
}
