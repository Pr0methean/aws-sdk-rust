// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_dev_endpoint::_create_dev_endpoint_output::CreateDevEndpointOutputBuilder;

pub use crate::operation::create_dev_endpoint::_create_dev_endpoint_input::CreateDevEndpointInputBuilder;

impl CreateDevEndpointInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_dev_endpoint::CreateDevEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_dev_endpoint::CreateDevEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_dev_endpoint();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDevEndpoint`.
///
/// <p>Creates a new development endpoint.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDevEndpointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_dev_endpoint::builders::CreateDevEndpointInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_dev_endpoint::CreateDevEndpointOutput,
        crate::operation::create_dev_endpoint::CreateDevEndpointError,
    > for CreateDevEndpointFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_dev_endpoint::CreateDevEndpointOutput,
            crate::operation::create_dev_endpoint::CreateDevEndpointError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDevEndpointFluentBuilder {
    /// Creates a new `CreateDevEndpoint`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDevEndpoint as a reference.
    pub fn as_input(&self) -> &crate::operation::create_dev_endpoint::builders::CreateDevEndpointInputBuilder {
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
        crate::operation::create_dev_endpoint::CreateDevEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_dev_endpoint::CreateDevEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_dev_endpoint::CreateDevEndpoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_dev_endpoint::CreateDevEndpoint::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_dev_endpoint::CreateDevEndpointOutput,
        crate::operation::create_dev_endpoint::CreateDevEndpointError,
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
    /// <p>The name to be assigned to the new <code>DevEndpoint</code>.</p>
    pub fn endpoint_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.endpoint_name(input.into());
        self
    }
    /// <p>The name to be assigned to the new <code>DevEndpoint</code>.</p>
    pub fn set_endpoint_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_endpoint_name(input);
        self
    }
    /// <p>The name to be assigned to the new <code>DevEndpoint</code>.</p>
    pub fn get_endpoint_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_endpoint_name()
    }
    /// <p>The IAM role for the <code>DevEndpoint</code>.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The IAM role for the <code>DevEndpoint</code>.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The IAM role for the <code>DevEndpoint</code>.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// Appends an item to `SecurityGroupIds`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>Security group IDs for the security groups to be used by the new <code>DevEndpoint</code>.</p>
    pub fn security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.security_group_ids(input.into());
        self
    }
    /// <p>Security group IDs for the security groups to be used by the new <code>DevEndpoint</code>.</p>
    pub fn set_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_security_group_ids(input);
        self
    }
    /// <p>Security group IDs for the security groups to be used by the new <code>DevEndpoint</code>.</p>
    pub fn get_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_security_group_ids()
    }
    /// <p>The subnet ID for the new <code>DevEndpoint</code> to use.</p>
    pub fn subnet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subnet_id(input.into());
        self
    }
    /// <p>The subnet ID for the new <code>DevEndpoint</code> to use.</p>
    pub fn set_subnet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_subnet_id(input);
        self
    }
    /// <p>The subnet ID for the new <code>DevEndpoint</code> to use.</p>
    pub fn get_subnet_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_subnet_id()
    }
    /// <p>The public key to be used by this <code>DevEndpoint</code> for authentication. This attribute is provided for backward compatibility because the recommended attribute to use is public keys.</p>
    pub fn public_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.public_key(input.into());
        self
    }
    /// <p>The public key to be used by this <code>DevEndpoint</code> for authentication. This attribute is provided for backward compatibility because the recommended attribute to use is public keys.</p>
    pub fn set_public_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_public_key(input);
        self
    }
    /// <p>The public key to be used by this <code>DevEndpoint</code> for authentication. This attribute is provided for backward compatibility because the recommended attribute to use is public keys.</p>
    pub fn get_public_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_public_key()
    }
    /// Appends an item to `PublicKeys`.
    ///
    /// To override the contents of this collection use [`set_public_keys`](Self::set_public_keys).
    ///
    /// <p>A list of public keys to be used by the development endpoints for authentication. The use of this attribute is preferred over a single public key because the public keys allow you to have a different private key per client.</p><note>
    /// <p>If you previously created an endpoint with a public key, you must remove that key to be able to set a list of public keys. Call the <code>UpdateDevEndpoint</code> API with the public key content in the <code>deletePublicKeys</code> attribute, and the list of new keys in the <code>addPublicKeys</code> attribute.</p>
    /// </note>
    pub fn public_keys(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.public_keys(input.into());
        self
    }
    /// <p>A list of public keys to be used by the development endpoints for authentication. The use of this attribute is preferred over a single public key because the public keys allow you to have a different private key per client.</p><note>
    /// <p>If you previously created an endpoint with a public key, you must remove that key to be able to set a list of public keys. Call the <code>UpdateDevEndpoint</code> API with the public key content in the <code>deletePublicKeys</code> attribute, and the list of new keys in the <code>addPublicKeys</code> attribute.</p>
    /// </note>
    pub fn set_public_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_public_keys(input);
        self
    }
    /// <p>A list of public keys to be used by the development endpoints for authentication. The use of this attribute is preferred over a single public key because the public keys allow you to have a different private key per client.</p><note>
    /// <p>If you previously created an endpoint with a public key, you must remove that key to be able to set a list of public keys. Call the <code>UpdateDevEndpoint</code> API with the public key content in the <code>deletePublicKeys</code> attribute, and the list of new keys in the <code>addPublicKeys</code> attribute.</p>
    /// </note>
    pub fn get_public_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_public_keys()
    }
    /// <p>The number of Glue Data Processing Units (DPUs) to allocate to this <code>DevEndpoint</code>.</p>
    pub fn number_of_nodes(mut self, input: i32) -> Self {
        self.inner = self.inner.number_of_nodes(input);
        self
    }
    /// <p>The number of Glue Data Processing Units (DPUs) to allocate to this <code>DevEndpoint</code>.</p>
    pub fn set_number_of_nodes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_number_of_nodes(input);
        self
    }
    /// <p>The number of Glue Data Processing Units (DPUs) to allocate to this <code>DevEndpoint</code>.</p>
    pub fn get_number_of_nodes(&self) -> &::std::option::Option<i32> {
        self.inner.get_number_of_nodes()
    }
    /// <p>The type of predefined worker that is allocated to the development endpoint. Accepts a value of Standard, G.1X, or G.2X.</p>
    /// <ul>
    /// <li>
    /// <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p></li>
    /// <li>
    /// <p>For the <code>G.1X</code> worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p></li>
    /// <li>
    /// <p>For the <code>G.2X</code> worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p></li>
    /// </ul>
    /// <p>Known issue: when a development endpoint is created with the <code>G.2X</code> <code>WorkerType</code> configuration, the Spark drivers for the development endpoint will run on 4 vCPU, 16 GB of memory, and a 64 GB disk.</p>
    pub fn worker_type(mut self, input: crate::types::WorkerType) -> Self {
        self.inner = self.inner.worker_type(input);
        self
    }
    /// <p>The type of predefined worker that is allocated to the development endpoint. Accepts a value of Standard, G.1X, or G.2X.</p>
    /// <ul>
    /// <li>
    /// <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p></li>
    /// <li>
    /// <p>For the <code>G.1X</code> worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p></li>
    /// <li>
    /// <p>For the <code>G.2X</code> worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p></li>
    /// </ul>
    /// <p>Known issue: when a development endpoint is created with the <code>G.2X</code> <code>WorkerType</code> configuration, the Spark drivers for the development endpoint will run on 4 vCPU, 16 GB of memory, and a 64 GB disk.</p>
    pub fn set_worker_type(mut self, input: ::std::option::Option<crate::types::WorkerType>) -> Self {
        self.inner = self.inner.set_worker_type(input);
        self
    }
    /// <p>The type of predefined worker that is allocated to the development endpoint. Accepts a value of Standard, G.1X, or G.2X.</p>
    /// <ul>
    /// <li>
    /// <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p></li>
    /// <li>
    /// <p>For the <code>G.1X</code> worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p></li>
    /// <li>
    /// <p>For the <code>G.2X</code> worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p></li>
    /// </ul>
    /// <p>Known issue: when a development endpoint is created with the <code>G.2X</code> <code>WorkerType</code> configuration, the Spark drivers for the development endpoint will run on 4 vCPU, 16 GB of memory, and a 64 GB disk.</p>
    pub fn get_worker_type(&self) -> &::std::option::Option<crate::types::WorkerType> {
        self.inner.get_worker_type()
    }
    /// <p>Glue version determines the versions of Apache Spark and Python that Glue supports. The Python version indicates the version supported for running your ETL scripts on development endpoints.</p>
    /// <p>For more information about the available Glue versions and corresponding Spark and Python versions, see <a href="https://docs.aws.amazon.com/glue/latest/dg/add-job.html">Glue version</a> in the developer guide.</p>
    /// <p>Development endpoints that are created without specifying a Glue version default to Glue 0.9.</p>
    /// <p>You can specify a version of Python support for development endpoints by using the <code>Arguments</code> parameter in the <code>CreateDevEndpoint</code> or <code>UpdateDevEndpoint</code> APIs. If no arguments are provided, the version defaults to Python 2.</p>
    pub fn glue_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.glue_version(input.into());
        self
    }
    /// <p>Glue version determines the versions of Apache Spark and Python that Glue supports. The Python version indicates the version supported for running your ETL scripts on development endpoints.</p>
    /// <p>For more information about the available Glue versions and corresponding Spark and Python versions, see <a href="https://docs.aws.amazon.com/glue/latest/dg/add-job.html">Glue version</a> in the developer guide.</p>
    /// <p>Development endpoints that are created without specifying a Glue version default to Glue 0.9.</p>
    /// <p>You can specify a version of Python support for development endpoints by using the <code>Arguments</code> parameter in the <code>CreateDevEndpoint</code> or <code>UpdateDevEndpoint</code> APIs. If no arguments are provided, the version defaults to Python 2.</p>
    pub fn set_glue_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_glue_version(input);
        self
    }
    /// <p>Glue version determines the versions of Apache Spark and Python that Glue supports. The Python version indicates the version supported for running your ETL scripts on development endpoints.</p>
    /// <p>For more information about the available Glue versions and corresponding Spark and Python versions, see <a href="https://docs.aws.amazon.com/glue/latest/dg/add-job.html">Glue version</a> in the developer guide.</p>
    /// <p>Development endpoints that are created without specifying a Glue version default to Glue 0.9.</p>
    /// <p>You can specify a version of Python support for development endpoints by using the <code>Arguments</code> parameter in the <code>CreateDevEndpoint</code> or <code>UpdateDevEndpoint</code> APIs. If no arguments are provided, the version defaults to Python 2.</p>
    pub fn get_glue_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_glue_version()
    }
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated to the development endpoint.</p>
    /// <p>The maximum number of workers you can define are 299 for <code>G.1X</code>, and 149 for <code>G.2X</code>.</p>
    pub fn number_of_workers(mut self, input: i32) -> Self {
        self.inner = self.inner.number_of_workers(input);
        self
    }
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated to the development endpoint.</p>
    /// <p>The maximum number of workers you can define are 299 for <code>G.1X</code>, and 149 for <code>G.2X</code>.</p>
    pub fn set_number_of_workers(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_number_of_workers(input);
        self
    }
    /// <p>The number of workers of a defined <code>workerType</code> that are allocated to the development endpoint.</p>
    /// <p>The maximum number of workers you can define are 299 for <code>G.1X</code>, and 149 for <code>G.2X</code>.</p>
    pub fn get_number_of_workers(&self) -> &::std::option::Option<i32> {
        self.inner.get_number_of_workers()
    }
    /// <p>The paths to one or more Python libraries in an Amazon S3 bucket that should be loaded in your <code>DevEndpoint</code>. Multiple values must be complete paths separated by a comma.</p><note>
    /// <p>You can only use pure Python libraries with a <code>DevEndpoint</code>. Libraries that rely on C extensions, such as the <a href="http://pandas.pydata.org/">pandas</a> Python data analysis library, are not yet supported.</p>
    /// </note>
    pub fn extra_python_libs_s3_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.extra_python_libs_s3_path(input.into());
        self
    }
    /// <p>The paths to one or more Python libraries in an Amazon S3 bucket that should be loaded in your <code>DevEndpoint</code>. Multiple values must be complete paths separated by a comma.</p><note>
    /// <p>You can only use pure Python libraries with a <code>DevEndpoint</code>. Libraries that rely on C extensions, such as the <a href="http://pandas.pydata.org/">pandas</a> Python data analysis library, are not yet supported.</p>
    /// </note>
    pub fn set_extra_python_libs_s3_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_extra_python_libs_s3_path(input);
        self
    }
    /// <p>The paths to one or more Python libraries in an Amazon S3 bucket that should be loaded in your <code>DevEndpoint</code>. Multiple values must be complete paths separated by a comma.</p><note>
    /// <p>You can only use pure Python libraries with a <code>DevEndpoint</code>. Libraries that rely on C extensions, such as the <a href="http://pandas.pydata.org/">pandas</a> Python data analysis library, are not yet supported.</p>
    /// </note>
    pub fn get_extra_python_libs_s3_path(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_extra_python_libs_s3_path()
    }
    /// <p>The path to one or more Java <code>.jar</code> files in an S3 bucket that should be loaded in your <code>DevEndpoint</code>.</p>
    pub fn extra_jars_s3_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.extra_jars_s3_path(input.into());
        self
    }
    /// <p>The path to one or more Java <code>.jar</code> files in an S3 bucket that should be loaded in your <code>DevEndpoint</code>.</p>
    pub fn set_extra_jars_s3_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_extra_jars_s3_path(input);
        self
    }
    /// <p>The path to one or more Java <code>.jar</code> files in an S3 bucket that should be loaded in your <code>DevEndpoint</code>.</p>
    pub fn get_extra_jars_s3_path(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_extra_jars_s3_path()
    }
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used with this <code>DevEndpoint</code>.</p>
    pub fn security_configuration(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.security_configuration(input.into());
        self
    }
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used with this <code>DevEndpoint</code>.</p>
    pub fn set_security_configuration(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_security_configuration(input);
        self
    }
    /// <p>The name of the <code>SecurityConfiguration</code> structure to be used with this <code>DevEndpoint</code>.</p>
    pub fn get_security_configuration(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_security_configuration()
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to use with this DevEndpoint. You may use tags to limit access to the DevEndpoint. For more information about tags in Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">Amazon Web Services Tags in Glue</a> in the developer guide.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags to use with this DevEndpoint. You may use tags to limit access to the DevEndpoint. For more information about tags in Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">Amazon Web Services Tags in Glue</a> in the developer guide.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to use with this DevEndpoint. You may use tags to limit access to the DevEndpoint. For more information about tags in Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">Amazon Web Services Tags in Glue</a> in the developer guide.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
    /// Adds a key-value pair to `Arguments`.
    ///
    /// To override the contents of this collection use [`set_arguments`](Self::set_arguments).
    ///
    /// <p>A map of arguments used to configure the <code>DevEndpoint</code>.</p>
    pub fn arguments(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arguments(k.into(), v.into());
        self
    }
    /// <p>A map of arguments used to configure the <code>DevEndpoint</code>.</p>
    pub fn set_arguments(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_arguments(input);
        self
    }
    /// <p>A map of arguments used to configure the <code>DevEndpoint</code>.</p>
    pub fn get_arguments(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_arguments()
    }
}
