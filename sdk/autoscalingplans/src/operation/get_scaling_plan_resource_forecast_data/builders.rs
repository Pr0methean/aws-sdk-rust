// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_scaling_plan_resource_forecast_data::_get_scaling_plan_resource_forecast_data_output::GetScalingPlanResourceForecastDataOutputBuilder;

pub use crate::operation::get_scaling_plan_resource_forecast_data::_get_scaling_plan_resource_forecast_data_input::GetScalingPlanResourceForecastDataInputBuilder;

impl GetScalingPlanResourceForecastDataInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastDataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastDataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_scaling_plan_resource_forecast_data();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetScalingPlanResourceForecastData`.
///
/// <p>Retrieves the forecast data for a scalable resource.</p>
/// <p>Capacity forecasts are represented as predicted values, or data points, that are calculated using historical data points from a specified CloudWatch load metric. Data points are available for up to 56 days.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetScalingPlanResourceForecastDataFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_scaling_plan_resource_forecast_data::builders::GetScalingPlanResourceForecastDataInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastDataOutput,
        crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastDataError,
    > for GetScalingPlanResourceForecastDataFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastDataOutput,
            crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastDataError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetScalingPlanResourceForecastDataFluentBuilder {
    /// Creates a new `GetScalingPlanResourceForecastData`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetScalingPlanResourceForecastData as a reference.
    pub fn as_input(&self) -> &crate::operation::get_scaling_plan_resource_forecast_data::builders::GetScalingPlanResourceForecastDataInputBuilder {
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
        crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastDataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastDataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastData::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastData::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastDataOutput,
        crate::operation::get_scaling_plan_resource_forecast_data::GetScalingPlanResourceForecastDataError,
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
    /// <p>The name of the scaling plan.</p>
    pub fn scaling_plan_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.scaling_plan_name(input.into());
        self
    }
    /// <p>The name of the scaling plan.</p>
    pub fn set_scaling_plan_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_scaling_plan_name(input);
        self
    }
    /// <p>The name of the scaling plan.</p>
    pub fn get_scaling_plan_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_scaling_plan_name()
    }
    /// <p>The version number of the scaling plan. Currently, the only valid value is <code>1</code>.</p>
    pub fn scaling_plan_version(mut self, input: i64) -> Self {
        self.inner = self.inner.scaling_plan_version(input);
        self
    }
    /// <p>The version number of the scaling plan. Currently, the only valid value is <code>1</code>.</p>
    pub fn set_scaling_plan_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_scaling_plan_version(input);
        self
    }
    /// <p>The version number of the scaling plan. Currently, the only valid value is <code>1</code>.</p>
    pub fn get_scaling_plan_version(&self) -> &::std::option::Option<i64> {
        self.inner.get_scaling_plan_version()
    }
    /// <p>The namespace of the AWS service. The only valid value is <code>autoscaling</code>.</p>
    pub fn service_namespace(mut self, input: crate::types::ServiceNamespace) -> Self {
        self.inner = self.inner.service_namespace(input);
        self
    }
    /// <p>The namespace of the AWS service. The only valid value is <code>autoscaling</code>.</p>
    pub fn set_service_namespace(mut self, input: ::std::option::Option<crate::types::ServiceNamespace>) -> Self {
        self.inner = self.inner.set_service_namespace(input);
        self
    }
    /// <p>The namespace of the AWS service. The only valid value is <code>autoscaling</code>.</p>
    pub fn get_service_namespace(&self) -> &::std::option::Option<crate::types::ServiceNamespace> {
        self.inner.get_service_namespace()
    }
    /// <p>The ID of the resource. This string consists of a prefix (<code>autoScalingGroup</code>) followed by the name of a specified Auto Scaling group (<code>my-asg</code>). Example: <code>autoScalingGroup/my-asg</code>.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The ID of the resource. This string consists of a prefix (<code>autoScalingGroup</code>) followed by the name of a specified Auto Scaling group (<code>my-asg</code>). Example: <code>autoScalingGroup/my-asg</code>.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>The ID of the resource. This string consists of a prefix (<code>autoScalingGroup</code>) followed by the name of a specified Auto Scaling group (<code>my-asg</code>). Example: <code>autoScalingGroup/my-asg</code>.</p>
    pub fn get_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_id()
    }
    /// <p>The scalable dimension for the resource. The only valid value is <code>autoscaling:autoScalingGroup:DesiredCapacity</code>.</p>
    pub fn scalable_dimension(mut self, input: crate::types::ScalableDimension) -> Self {
        self.inner = self.inner.scalable_dimension(input);
        self
    }
    /// <p>The scalable dimension for the resource. The only valid value is <code>autoscaling:autoScalingGroup:DesiredCapacity</code>.</p>
    pub fn set_scalable_dimension(mut self, input: ::std::option::Option<crate::types::ScalableDimension>) -> Self {
        self.inner = self.inner.set_scalable_dimension(input);
        self
    }
    /// <p>The scalable dimension for the resource. The only valid value is <code>autoscaling:autoScalingGroup:DesiredCapacity</code>.</p>
    pub fn get_scalable_dimension(&self) -> &::std::option::Option<crate::types::ScalableDimension> {
        self.inner.get_scalable_dimension()
    }
    /// <p>The type of forecast data to get.</p>
    /// <ul>
    /// <li>
    /// <p><code>LoadForecast</code>: The load metric forecast.</p></li>
    /// <li>
    /// <p><code>CapacityForecast</code>: The capacity forecast.</p></li>
    /// <li>
    /// <p><code>ScheduledActionMinCapacity</code>: The minimum capacity for each scheduled scaling action. This data is calculated as the larger of two values: the capacity forecast or the minimum capacity in the scaling instruction.</p></li>
    /// <li>
    /// <p><code>ScheduledActionMaxCapacity</code>: The maximum capacity for each scheduled scaling action. The calculation used is determined by the predictive scaling maximum capacity behavior setting in the scaling instruction.</p></li>
    /// </ul>
    pub fn forecast_data_type(mut self, input: crate::types::ForecastDataType) -> Self {
        self.inner = self.inner.forecast_data_type(input);
        self
    }
    /// <p>The type of forecast data to get.</p>
    /// <ul>
    /// <li>
    /// <p><code>LoadForecast</code>: The load metric forecast.</p></li>
    /// <li>
    /// <p><code>CapacityForecast</code>: The capacity forecast.</p></li>
    /// <li>
    /// <p><code>ScheduledActionMinCapacity</code>: The minimum capacity for each scheduled scaling action. This data is calculated as the larger of two values: the capacity forecast or the minimum capacity in the scaling instruction.</p></li>
    /// <li>
    /// <p><code>ScheduledActionMaxCapacity</code>: The maximum capacity for each scheduled scaling action. The calculation used is determined by the predictive scaling maximum capacity behavior setting in the scaling instruction.</p></li>
    /// </ul>
    pub fn set_forecast_data_type(mut self, input: ::std::option::Option<crate::types::ForecastDataType>) -> Self {
        self.inner = self.inner.set_forecast_data_type(input);
        self
    }
    /// <p>The type of forecast data to get.</p>
    /// <ul>
    /// <li>
    /// <p><code>LoadForecast</code>: The load metric forecast.</p></li>
    /// <li>
    /// <p><code>CapacityForecast</code>: The capacity forecast.</p></li>
    /// <li>
    /// <p><code>ScheduledActionMinCapacity</code>: The minimum capacity for each scheduled scaling action. This data is calculated as the larger of two values: the capacity forecast or the minimum capacity in the scaling instruction.</p></li>
    /// <li>
    /// <p><code>ScheduledActionMaxCapacity</code>: The maximum capacity for each scheduled scaling action. The calculation used is determined by the predictive scaling maximum capacity behavior setting in the scaling instruction.</p></li>
    /// </ul>
    pub fn get_forecast_data_type(&self) -> &::std::option::Option<crate::types::ForecastDataType> {
        self.inner.get_forecast_data_type()
    }
    /// <p>The inclusive start time of the time range for the forecast data to get. The date and time can be at most 56 days before the current date and time.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The inclusive start time of the time range for the forecast data to get. The date and time can be at most 56 days before the current date and time.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The inclusive start time of the time range for the forecast data to get. The date and time can be at most 56 days before the current date and time.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>The exclusive end time of the time range for the forecast data to get. The maximum time duration between the start and end time is seven days.</p>
    /// <p>Although this parameter can accept a date and time that is more than two days in the future, the availability of forecast data has limits. AWS Auto Scaling only issues forecasts for periods of two days in advance.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The exclusive end time of the time range for the forecast data to get. The maximum time duration between the start and end time is seven days.</p>
    /// <p>Although this parameter can accept a date and time that is more than two days in the future, the availability of forecast data has limits. AWS Auto Scaling only issues forecasts for periods of two days in advance.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The exclusive end time of the time range for the forecast data to get. The maximum time duration between the start and end time is seven days.</p>
    /// <p>Although this parameter can accept a date and time that is more than two days in the future, the availability of forecast data has limits. AWS Auto Scaling only issues forecasts for periods of two days in advance.</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
}
