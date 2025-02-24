// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartRun`](crate::operation::start_run::builders::StartRunFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workflow_id(impl Into<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::workflow_id) / [`set_workflow_id(Option<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_workflow_id):<br>required: **false**<br><p>The run's workflow ID.</p><br>
    ///   - [`workflow_type(WorkflowType)`](crate::operation::start_run::builders::StartRunFluentBuilder::workflow_type) / [`set_workflow_type(Option<WorkflowType>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_workflow_type):<br>required: **false**<br><p>The run's workflow type.</p><br>
    ///   - [`run_id(impl Into<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::run_id) / [`set_run_id(Option<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_run_id):<br>required: **false**<br><p>The ID of a run to duplicate.</p><br>
    ///   - [`role_arn(impl Into<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_role_arn):<br>required: **true**<br><p>A service role for the run.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_name):<br>required: **false**<br><p>A name for the run.</p><br>
    ///   - [`run_group_id(impl Into<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::run_group_id) / [`set_run_group_id(Option<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_run_group_id):<br>required: **false**<br><p>The run's group ID.</p><br>
    ///   - [`priority(i32)`](crate::operation::start_run::builders::StartRunFluentBuilder::priority) / [`set_priority(Option<i32>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_priority):<br>required: **false**<br><p>A priority for the run.</p><br>
    ///   - [`parameters(Document)`](crate::operation::start_run::builders::StartRunFluentBuilder::parameters) / [`set_parameters(Option<Document>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_parameters):<br>required: **false**<br><p>Parameters for the run.</p><br>
    ///   - [`storage_capacity(i32)`](crate::operation::start_run::builders::StartRunFluentBuilder::storage_capacity) / [`set_storage_capacity(Option<i32>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_storage_capacity):<br>required: **false**<br><p>A storage capacity for the run in gigabytes.</p><br>
    ///   - [`output_uri(impl Into<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::output_uri) / [`set_output_uri(Option<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_output_uri):<br>required: **false**<br><p>An output URI for the run.</p><br>
    ///   - [`log_level(RunLogLevel)`](crate::operation::start_run::builders::StartRunFluentBuilder::log_level) / [`set_log_level(Option<RunLogLevel>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_log_level):<br>required: **false**<br><p>A log level for the run.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_tags):<br>required: **false**<br><p>Tags for the run.</p><br>
    ///   - [`request_id(impl Into<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::request_id) / [`set_request_id(Option<String>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_request_id):<br>required: **true**<br><p>To ensure that requests don't run multiple times, specify a unique ID for each request.</p><br>
    ///   - [`retention_mode(RunRetentionMode)`](crate::operation::start_run::builders::StartRunFluentBuilder::retention_mode) / [`set_retention_mode(Option<RunRetentionMode>)`](crate::operation::start_run::builders::StartRunFluentBuilder::set_retention_mode):<br>required: **false**<br><p>The retention mode for the run.</p><br>
    /// - On success, responds with [`StartRunOutput`](crate::operation::start_run::StartRunOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::start_run::StartRunOutput::arn): <p>The run's ARN.</p>
    ///   - [`id(Option<String>)`](crate::operation::start_run::StartRunOutput::id): <p>The run's ID.</p>
    ///   - [`status(Option<RunStatus>)`](crate::operation::start_run::StartRunOutput::status): <p>The run's status.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::start_run::StartRunOutput::tags): <p>The run's tags.</p>
    ///   - [`uuid(Option<String>)`](crate::operation::start_run::StartRunOutput::uuid): <p>The universally unique identifier for a run.</p>
    ///   - [`run_output_uri(Option<String>)`](crate::operation::start_run::StartRunOutput::run_output_uri): <p>The destination for workflow outputs.</p>
    /// - On failure, responds with [`SdkError<StartRunError>`](crate::operation::start_run::StartRunError)
    pub fn start_run(&self) -> crate::operation::start_run::builders::StartRunFluentBuilder {
        crate::operation::start_run::builders::StartRunFluentBuilder::new(self.handle.clone())
    }
}
