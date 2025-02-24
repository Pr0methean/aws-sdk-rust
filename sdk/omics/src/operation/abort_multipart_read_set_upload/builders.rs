// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::abort_multipart_read_set_upload::_abort_multipart_read_set_upload_output::AbortMultipartReadSetUploadOutputBuilder;

pub use crate::operation::abort_multipart_read_set_upload::_abort_multipart_read_set_upload_input::AbortMultipartReadSetUploadInputBuilder;

impl AbortMultipartReadSetUploadInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.abort_multipart_read_set_upload();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AbortMultipartReadSetUpload`.
///
/// <p>Stops a multipart upload.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AbortMultipartReadSetUploadFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::abort_multipart_read_set_upload::builders::AbortMultipartReadSetUploadInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadOutput,
        crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadError,
    > for AbortMultipartReadSetUploadFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadOutput,
            crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AbortMultipartReadSetUploadFluentBuilder {
    /// Creates a new `AbortMultipartReadSetUpload`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AbortMultipartReadSetUpload as a reference.
    pub fn as_input(&self) -> &crate::operation::abort_multipart_read_set_upload::builders::AbortMultipartReadSetUploadInputBuilder {
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
        crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUpload::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUpload::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadOutput,
        crate::operation::abort_multipart_read_set_upload::AbortMultipartReadSetUploadError,
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
    /// <p>The sequence store ID for the store involved in the multipart upload.</p>
    pub fn sequence_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sequence_store_id(input.into());
        self
    }
    /// <p>The sequence store ID for the store involved in the multipart upload.</p>
    pub fn set_sequence_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sequence_store_id(input);
        self
    }
    /// <p>The sequence store ID for the store involved in the multipart upload.</p>
    pub fn get_sequence_store_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sequence_store_id()
    }
    /// <p>The ID for the multipart upload.</p>
    pub fn upload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.upload_id(input.into());
        self
    }
    /// <p>The ID for the multipart upload.</p>
    pub fn set_upload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_upload_id(input);
        self
    }
    /// <p>The ID for the multipart upload.</p>
    pub fn get_upload_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_upload_id()
    }
}
