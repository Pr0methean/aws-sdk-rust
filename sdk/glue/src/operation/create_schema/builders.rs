// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_schema::_create_schema_output::CreateSchemaOutputBuilder;

pub use crate::operation::create_schema::_create_schema_input::CreateSchemaInputBuilder;

impl CreateSchemaInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_schema::CreateSchemaOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_schema::CreateSchemaError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_schema();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateSchema`.
///
/// <p>Creates a new schema set and registers the schema definition. Returns an error if the schema set already exists without actually registering the version.</p>
/// <p>When the schema set is created, a version checkpoint will be set to the first version. Compatibility mode "DISABLED" restricts any additional schema versions from being added after the first schema version. For all other compatibility modes, validation of compatibility settings will be applied only from the second version onwards when the <code>RegisterSchemaVersion</code> API is used.</p>
/// <p>When this API is called without a <code>RegistryId</code>, this will create an entry for a "default-registry" in the registry database tables, if it is not already present.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSchemaFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_schema::builders::CreateSchemaInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_schema::CreateSchemaOutput,
        crate::operation::create_schema::CreateSchemaError,
    > for CreateSchemaFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_schema::CreateSchemaOutput,
            crate::operation::create_schema::CreateSchemaError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateSchemaFluentBuilder {
    /// Creates a new `CreateSchema`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateSchema as a reference.
    pub fn as_input(&self) -> &crate::operation::create_schema::builders::CreateSchemaInputBuilder {
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
        crate::operation::create_schema::CreateSchemaOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_schema::CreateSchemaError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_schema::CreateSchema::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_schema::CreateSchema::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_schema::CreateSchemaOutput,
        crate::operation::create_schema::CreateSchemaError,
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
    /// <p>This is a wrapper shape to contain the registry identity fields. If this is not provided, the default registry will be used. The ARN format for the same will be: <code>arn:aws:glue:us-east-2:<customer id>
    /// :registry/default-registry:random-5-letter-id
    /// </customer></code>.</p>
    pub fn registry_id(mut self, input: crate::types::RegistryId) -> Self {
        self.inner = self.inner.registry_id(input);
        self
    }
    /// <p>This is a wrapper shape to contain the registry identity fields. If this is not provided, the default registry will be used. The ARN format for the same will be: <code>arn:aws:glue:us-east-2:<customer id>
    /// :registry/default-registry:random-5-letter-id
    /// </customer></code>.</p>
    pub fn set_registry_id(mut self, input: ::std::option::Option<crate::types::RegistryId>) -> Self {
        self.inner = self.inner.set_registry_id(input);
        self
    }
    /// <p>This is a wrapper shape to contain the registry identity fields. If this is not provided, the default registry will be used. The ARN format for the same will be: <code>arn:aws:glue:us-east-2:<customer id>
    /// :registry/default-registry:random-5-letter-id
    /// </customer></code>.</p>
    pub fn get_registry_id(&self) -> &::std::option::Option<crate::types::RegistryId> {
        self.inner.get_registry_id()
    }
    /// <p>Name of the schema to be created of max length of 255, and may only contain letters, numbers, hyphen, underscore, dollar sign, or hash mark. No whitespace.</p>
    pub fn schema_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.schema_name(input.into());
        self
    }
    /// <p>Name of the schema to be created of max length of 255, and may only contain letters, numbers, hyphen, underscore, dollar sign, or hash mark. No whitespace.</p>
    pub fn set_schema_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_schema_name(input);
        self
    }
    /// <p>Name of the schema to be created of max length of 255, and may only contain letters, numbers, hyphen, underscore, dollar sign, or hash mark. No whitespace.</p>
    pub fn get_schema_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_schema_name()
    }
    /// <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p>
    pub fn data_format(mut self, input: crate::types::DataFormat) -> Self {
        self.inner = self.inner.data_format(input);
        self
    }
    /// <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p>
    pub fn set_data_format(mut self, input: ::std::option::Option<crate::types::DataFormat>) -> Self {
        self.inner = self.inner.set_data_format(input);
        self
    }
    /// <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p>
    pub fn get_data_format(&self) -> &::std::option::Option<crate::types::DataFormat> {
        self.inner.get_data_format()
    }
    /// <p>The compatibility mode of the schema. The possible values are:</p>
    /// <ul>
    /// <li>
    /// <p><i>NONE</i>: No compatibility mode applies. You can use this choice in development scenarios or if you do not know the compatibility mode that you want to apply to schemas. Any new version added will be accepted without undergoing a compatibility check.</p></li>
    /// <li>
    /// <p><i>DISABLED</i>: This compatibility choice prevents versioning for a particular schema. You can use this choice to prevent future versioning of a schema.</p></li>
    /// <li>
    /// <p><i>BACKWARD</i>: This compatibility choice is recommended as it allows data receivers to read both the current and one previous schema version. This means that for instance, a new schema version cannot drop data fields or change the type of these fields, so they can't be read by readers using the previous version.</p></li>
    /// <li>
    /// <p><i>BACKWARD_ALL</i>: This compatibility choice allows data receivers to read both the current and all previous schema versions. You can use this choice when you need to delete fields or add optional fields, and check compatibility against all previous schema versions.</p></li>
    /// <li>
    /// <p><i>FORWARD</i>: This compatibility choice allows data receivers to read both the current and one next schema version, but not necessarily later versions. You can use this choice when you need to add fields or delete optional fields, but only check compatibility against the last schema version.</p></li>
    /// <li>
    /// <p><i>FORWARD_ALL</i>: This compatibility choice allows data receivers to read written by producers of any new registered schema. You can use this choice when you need to add fields or delete optional fields, and check compatibility against all previous schema versions.</p></li>
    /// <li>
    /// <p><i>FULL</i>: This compatibility choice allows data receivers to read data written by producers using the previous or next version of the schema, but not necessarily earlier or later versions. You can use this choice when you need to add or remove optional fields, but only check compatibility against the last schema version.</p></li>
    /// <li>
    /// <p><i>FULL_ALL</i>: This compatibility choice allows data receivers to read data written by producers using all previous schema versions. You can use this choice when you need to add or remove optional fields, and check compatibility against all previous schema versions.</p></li>
    /// </ul>
    pub fn compatibility(mut self, input: crate::types::Compatibility) -> Self {
        self.inner = self.inner.compatibility(input);
        self
    }
    /// <p>The compatibility mode of the schema. The possible values are:</p>
    /// <ul>
    /// <li>
    /// <p><i>NONE</i>: No compatibility mode applies. You can use this choice in development scenarios or if you do not know the compatibility mode that you want to apply to schemas. Any new version added will be accepted without undergoing a compatibility check.</p></li>
    /// <li>
    /// <p><i>DISABLED</i>: This compatibility choice prevents versioning for a particular schema. You can use this choice to prevent future versioning of a schema.</p></li>
    /// <li>
    /// <p><i>BACKWARD</i>: This compatibility choice is recommended as it allows data receivers to read both the current and one previous schema version. This means that for instance, a new schema version cannot drop data fields or change the type of these fields, so they can't be read by readers using the previous version.</p></li>
    /// <li>
    /// <p><i>BACKWARD_ALL</i>: This compatibility choice allows data receivers to read both the current and all previous schema versions. You can use this choice when you need to delete fields or add optional fields, and check compatibility against all previous schema versions.</p></li>
    /// <li>
    /// <p><i>FORWARD</i>: This compatibility choice allows data receivers to read both the current and one next schema version, but not necessarily later versions. You can use this choice when you need to add fields or delete optional fields, but only check compatibility against the last schema version.</p></li>
    /// <li>
    /// <p><i>FORWARD_ALL</i>: This compatibility choice allows data receivers to read written by producers of any new registered schema. You can use this choice when you need to add fields or delete optional fields, and check compatibility against all previous schema versions.</p></li>
    /// <li>
    /// <p><i>FULL</i>: This compatibility choice allows data receivers to read data written by producers using the previous or next version of the schema, but not necessarily earlier or later versions. You can use this choice when you need to add or remove optional fields, but only check compatibility against the last schema version.</p></li>
    /// <li>
    /// <p><i>FULL_ALL</i>: This compatibility choice allows data receivers to read data written by producers using all previous schema versions. You can use this choice when you need to add or remove optional fields, and check compatibility against all previous schema versions.</p></li>
    /// </ul>
    pub fn set_compatibility(mut self, input: ::std::option::Option<crate::types::Compatibility>) -> Self {
        self.inner = self.inner.set_compatibility(input);
        self
    }
    /// <p>The compatibility mode of the schema. The possible values are:</p>
    /// <ul>
    /// <li>
    /// <p><i>NONE</i>: No compatibility mode applies. You can use this choice in development scenarios or if you do not know the compatibility mode that you want to apply to schemas. Any new version added will be accepted without undergoing a compatibility check.</p></li>
    /// <li>
    /// <p><i>DISABLED</i>: This compatibility choice prevents versioning for a particular schema. You can use this choice to prevent future versioning of a schema.</p></li>
    /// <li>
    /// <p><i>BACKWARD</i>: This compatibility choice is recommended as it allows data receivers to read both the current and one previous schema version. This means that for instance, a new schema version cannot drop data fields or change the type of these fields, so they can't be read by readers using the previous version.</p></li>
    /// <li>
    /// <p><i>BACKWARD_ALL</i>: This compatibility choice allows data receivers to read both the current and all previous schema versions. You can use this choice when you need to delete fields or add optional fields, and check compatibility against all previous schema versions.</p></li>
    /// <li>
    /// <p><i>FORWARD</i>: This compatibility choice allows data receivers to read both the current and one next schema version, but not necessarily later versions. You can use this choice when you need to add fields or delete optional fields, but only check compatibility against the last schema version.</p></li>
    /// <li>
    /// <p><i>FORWARD_ALL</i>: This compatibility choice allows data receivers to read written by producers of any new registered schema. You can use this choice when you need to add fields or delete optional fields, and check compatibility against all previous schema versions.</p></li>
    /// <li>
    /// <p><i>FULL</i>: This compatibility choice allows data receivers to read data written by producers using the previous or next version of the schema, but not necessarily earlier or later versions. You can use this choice when you need to add or remove optional fields, but only check compatibility against the last schema version.</p></li>
    /// <li>
    /// <p><i>FULL_ALL</i>: This compatibility choice allows data receivers to read data written by producers using all previous schema versions. You can use this choice when you need to add or remove optional fields, and check compatibility against all previous schema versions.</p></li>
    /// </ul>
    pub fn get_compatibility(&self) -> &::std::option::Option<crate::types::Compatibility> {
        self.inner.get_compatibility()
    }
    /// <p>An optional description of the schema. If description is not provided, there will not be any automatic default value for this.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An optional description of the schema. If description is not provided, there will not be any automatic default value for this.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>An optional description of the schema. If description is not provided, there will not be any automatic default value for this.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Amazon Web Services tags that contain a key value pair and may be searched by console, command line, or API. If specified, follows the Amazon Web Services tags-on-create pattern.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Amazon Web Services tags that contain a key value pair and may be searched by console, command line, or API. If specified, follows the Amazon Web Services tags-on-create pattern.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Amazon Web Services tags that contain a key value pair and may be searched by console, command line, or API. If specified, follows the Amazon Web Services tags-on-create pattern.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
    /// <p>The schema definition using the <code>DataFormat</code> setting for <code>SchemaName</code>.</p>
    pub fn schema_definition(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.schema_definition(input.into());
        self
    }
    /// <p>The schema definition using the <code>DataFormat</code> setting for <code>SchemaName</code>.</p>
    pub fn set_schema_definition(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_schema_definition(input);
        self
    }
    /// <p>The schema definition using the <code>DataFormat</code> setting for <code>SchemaName</code>.</p>
    pub fn get_schema_definition(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_schema_definition()
    }
}
