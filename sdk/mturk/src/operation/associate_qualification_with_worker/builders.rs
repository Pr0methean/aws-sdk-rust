// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_qualification_with_worker::_associate_qualification_with_worker_output::AssociateQualificationWithWorkerOutputBuilder;

pub use crate::operation::associate_qualification_with_worker::_associate_qualification_with_worker_input::AssociateQualificationWithWorkerInputBuilder;

impl AssociateQualificationWithWorkerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorkerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorkerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_qualification_with_worker();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateQualificationWithWorker`.
///
/// <p>The <code>AssociateQualificationWithWorker</code> operation gives a Worker a Qualification. <code>AssociateQualificationWithWorker</code> does not require that the Worker submit a Qualification request. It gives the Qualification directly to the Worker.</p>
/// <p>You can only assign a Qualification of a Qualification type that you created (using the <code>CreateQualificationType</code> operation).</p><note>
/// <p>Note: <code>AssociateQualificationWithWorker</code> does not affect any pending Qualification requests for the Qualification by the Worker. If you assign a Qualification to a Worker, then later grant a Qualification request made by the Worker, the granting of the request may modify the Qualification score. To resolve a pending Qualification request without affecting the Qualification the Worker already has, reject the request with the <code>RejectQualificationRequest</code> operation.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateQualificationWithWorkerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_qualification_with_worker::builders::AssociateQualificationWithWorkerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorkerOutput,
        crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorkerError,
    > for AssociateQualificationWithWorkerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorkerOutput,
            crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorkerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateQualificationWithWorkerFluentBuilder {
    /// Creates a new `AssociateQualificationWithWorker`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateQualificationWithWorker as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_qualification_with_worker::builders::AssociateQualificationWithWorkerInputBuilder {
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
        crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorkerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorkerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorker::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorker::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorkerOutput,
        crate::operation::associate_qualification_with_worker::AssociateQualificationWithWorkerError,
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
    /// <p>The ID of the Qualification type to use for the assigned Qualification.</p>
    pub fn qualification_type_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.qualification_type_id(input.into());
        self
    }
    /// <p>The ID of the Qualification type to use for the assigned Qualification.</p>
    pub fn set_qualification_type_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_qualification_type_id(input);
        self
    }
    /// <p>The ID of the Qualification type to use for the assigned Qualification.</p>
    pub fn get_qualification_type_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_qualification_type_id()
    }
    /// <p>The ID of the Worker to whom the Qualification is being assigned. Worker IDs are included with submitted HIT assignments and Qualification requests.</p>
    pub fn worker_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.worker_id(input.into());
        self
    }
    /// <p>The ID of the Worker to whom the Qualification is being assigned. Worker IDs are included with submitted HIT assignments and Qualification requests.</p>
    pub fn set_worker_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_worker_id(input);
        self
    }
    /// <p>The ID of the Worker to whom the Qualification is being assigned. Worker IDs are included with submitted HIT assignments and Qualification requests.</p>
    pub fn get_worker_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_worker_id()
    }
    /// <p>The value of the Qualification to assign.</p>
    pub fn integer_value(mut self, input: i32) -> Self {
        self.inner = self.inner.integer_value(input);
        self
    }
    /// <p>The value of the Qualification to assign.</p>
    pub fn set_integer_value(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_integer_value(input);
        self
    }
    /// <p>The value of the Qualification to assign.</p>
    pub fn get_integer_value(&self) -> &::std::option::Option<i32> {
        self.inner.get_integer_value()
    }
    /// <p>Specifies whether to send a notification email message to the Worker saying that the qualification was assigned to the Worker. Note: this is true by default.</p>
    pub fn send_notification(mut self, input: bool) -> Self {
        self.inner = self.inner.send_notification(input);
        self
    }
    /// <p>Specifies whether to send a notification email message to the Worker saying that the qualification was assigned to the Worker. Note: this is true by default.</p>
    pub fn set_send_notification(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_send_notification(input);
        self
    }
    /// <p>Specifies whether to send a notification email message to the Worker saying that the qualification was assigned to the Worker. Note: this is true by default.</p>
    pub fn get_send_notification(&self) -> &::std::option::Option<bool> {
        self.inner.get_send_notification()
    }
}
