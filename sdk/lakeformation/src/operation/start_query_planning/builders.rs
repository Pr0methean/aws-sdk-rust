// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_query_planning::_start_query_planning_output::StartQueryPlanningOutputBuilder;

pub use crate::operation::start_query_planning::_start_query_planning_input::StartQueryPlanningInputBuilder;

impl StartQueryPlanningInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_query_planning::StartQueryPlanningOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_query_planning::StartQueryPlanningError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_query_planning();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartQueryPlanning`.
///
/// <p>Submits a request to process a query statement.</p>
/// <p>This operation generates work units that can be retrieved with the <code>GetWorkUnits</code> operation as soon as the query state is WORKUNITS_AVAILABLE or FINISHED.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartQueryPlanningFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_query_planning::builders::StartQueryPlanningInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_query_planning::StartQueryPlanningOutput,
        crate::operation::start_query_planning::StartQueryPlanningError,
    > for StartQueryPlanningFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_query_planning::StartQueryPlanningOutput,
            crate::operation::start_query_planning::StartQueryPlanningError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartQueryPlanningFluentBuilder {
    /// Creates a new `StartQueryPlanning`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartQueryPlanning as a reference.
    pub fn as_input(&self) -> &crate::operation::start_query_planning::builders::StartQueryPlanningInputBuilder {
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
        crate::operation::start_query_planning::StartQueryPlanningOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_query_planning::StartQueryPlanningError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_query_planning::StartQueryPlanning::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_query_planning::StartQueryPlanning::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_query_planning::StartQueryPlanningOutput,
        crate::operation::start_query_planning::StartQueryPlanningError,
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
    /// <p>A structure containing information about the query plan.</p>
    pub fn query_planning_context(mut self, input: crate::types::QueryPlanningContext) -> Self {
        self.inner = self.inner.query_planning_context(input);
        self
    }
    /// <p>A structure containing information about the query plan.</p>
    pub fn set_query_planning_context(mut self, input: ::std::option::Option<crate::types::QueryPlanningContext>) -> Self {
        self.inner = self.inner.set_query_planning_context(input);
        self
    }
    /// <p>A structure containing information about the query plan.</p>
    pub fn get_query_planning_context(&self) -> &::std::option::Option<crate::types::QueryPlanningContext> {
        self.inner.get_query_planning_context()
    }
    /// <p>A PartiQL query statement used as an input to the planner service.</p>
    pub fn query_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_string(input.into());
        self
    }
    /// <p>A PartiQL query statement used as an input to the planner service.</p>
    pub fn set_query_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_string(input);
        self
    }
    /// <p>A PartiQL query statement used as an input to the planner service.</p>
    pub fn get_query_string(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_query_string()
    }
}
