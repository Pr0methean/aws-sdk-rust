// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_task_definitions::_delete_task_definitions_output::DeleteTaskDefinitionsOutputBuilder;

pub use crate::operation::delete_task_definitions::_delete_task_definitions_input::DeleteTaskDefinitionsInputBuilder;

impl DeleteTaskDefinitionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_task_definitions::DeleteTaskDefinitionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_task_definitions::DeleteTaskDefinitionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_task_definitions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteTaskDefinitions`.
///
/// <p>Deletes one or more task definitions.</p>
/// <p>You must deregister a task definition revision before you delete it. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_DeregisterTaskDefinition.html">DeregisterTaskDefinition</a>.</p>
/// <p>When you delete a task definition revision, it is immediately transitions from the <code>INACTIVE</code> to <code>DELETE_IN_PROGRESS</code>. Existing tasks and services that reference a <code>DELETE_IN_PROGRESS</code> task definition revision continue to run without disruption. Existing services that reference a <code>DELETE_IN_PROGRESS</code> task definition revision can still scale up or down by modifying the service's desired count.</p>
/// <p>You can't use a <code>DELETE_IN_PROGRESS</code> task definition revision to run new tasks or create new services. You also can't update an existing service to reference a <code>DELETE_IN_PROGRESS</code> task definition revision.</p>
/// <p>A task definition revision will stay in <code>DELETE_IN_PROGRESS</code> status until all the associated tasks and services have been terminated.</p>
/// <p>When you delete all <code>INACTIVE</code> task definition revisions, the task definition name is not displayed in the console and not returned in the API. If a task definition revisions are in the <code>DELETE_IN_PROGRESS</code> state, the task definition name is displayed in the console and returned in the API. The task definition name is retained by Amazon ECS and the revision is incremented the next time you create a task definition with that name.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteTaskDefinitionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_task_definitions::builders::DeleteTaskDefinitionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_task_definitions::DeleteTaskDefinitionsOutput,
        crate::operation::delete_task_definitions::DeleteTaskDefinitionsError,
    > for DeleteTaskDefinitionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_task_definitions::DeleteTaskDefinitionsOutput,
            crate::operation::delete_task_definitions::DeleteTaskDefinitionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteTaskDefinitionsFluentBuilder {
    /// Creates a new `DeleteTaskDefinitions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteTaskDefinitions as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_task_definitions::builders::DeleteTaskDefinitionsInputBuilder {
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
        crate::operation::delete_task_definitions::DeleteTaskDefinitionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_task_definitions::DeleteTaskDefinitionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_task_definitions::DeleteTaskDefinitions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_task_definitions::DeleteTaskDefinitions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_task_definitions::DeleteTaskDefinitionsOutput,
        crate::operation::delete_task_definitions::DeleteTaskDefinitionsError,
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
    /// Appends an item to `taskDefinitions`.
    ///
    /// To override the contents of this collection use [`set_task_definitions`](Self::set_task_definitions).
    ///
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to delete. You must specify a <code>revision</code>.</p>
    /// <p>You can specify up to 10 task definitions as a comma separated list.</p>
    pub fn task_definitions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.task_definitions(input.into());
        self
    }
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to delete. You must specify a <code>revision</code>.</p>
    /// <p>You can specify up to 10 task definitions as a comma separated list.</p>
    pub fn set_task_definitions(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_task_definitions(input);
        self
    }
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to delete. You must specify a <code>revision</code>.</p>
    /// <p>You can specify up to 10 task definitions as a comma separated list.</p>
    pub fn get_task_definitions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_task_definitions()
    }
}
