// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_schemas::_list_schemas_output::ListSchemasOutputBuilder;

pub use crate::operation::list_schemas::_list_schemas_input::ListSchemasInputBuilder;

impl ListSchemasInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_schemas::ListSchemasOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_schemas::ListSchemasError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_schemas();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListSchemas`.
///
/// <p>Lists the schemas in a database. A token is returned to page through the schema list. Depending on the authorization method, use one of the following combinations of request parameters:</p>
/// <ul>
/// <li>
/// <p>Secrets Manager - when connecting to a cluster, provide the <code>secret-arn</code> of a secret stored in Secrets Manager which has <code>username</code> and <code>password</code>. The specified secret contains credentials to connect to the <code>database</code> you specify. When you are connecting to a cluster, you also supply the database name, If you provide a cluster identifier (<code>dbClusterIdentifier</code>), it must match the cluster identifier stored in the secret. When you are connecting to a serverless workgroup, you also supply the database name.</p></li>
/// <li>
/// <p>Temporary credentials - when connecting to your data warehouse, choose one of the following options:</p>
/// <ul>
/// <li>
/// <p>When connecting to a serverless workgroup, specify the workgroup name and database name. The database user name is derived from the IAM identity. For example, <code>arn:iam::123456789012:user:foo</code> has the database user name <code>IAM:foo</code>. Also, permission to call the <code>redshift-serverless:GetCredentials</code> operation is required.</p></li>
/// <li>
/// <p>When connecting to a cluster as an IAM identity, specify the cluster identifier and the database name. The database user name is derived from the IAM identity. For example, <code>arn:iam::123456789012:user:foo</code> has the database user name <code>IAM:foo</code>. Also, permission to call the <code>redshift:GetClusterCredentialsWithIAM</code> operation is required.</p></li>
/// <li>
/// <p>When connecting to a cluster as a database user, specify the cluster identifier, the database name, and the database user name. Also, permission to call the <code>redshift:GetClusterCredentials</code> operation is required.</p></li>
/// </ul></li>
/// </ul>
/// <p>For more information about the Amazon Redshift Data API and CLI usage examples, see <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/data-api.html">Using the Amazon Redshift Data API</a> in the <i>Amazon Redshift Management Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListSchemasFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_schemas::builders::ListSchemasInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_schemas::ListSchemasOutput,
        crate::operation::list_schemas::ListSchemasError,
    > for ListSchemasFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_schemas::ListSchemasOutput,
            crate::operation::list_schemas::ListSchemasError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListSchemasFluentBuilder {
    /// Creates a new `ListSchemas`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListSchemas as a reference.
    pub fn as_input(&self) -> &crate::operation::list_schemas::builders::ListSchemasInputBuilder {
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
        crate::operation::list_schemas::ListSchemasOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_schemas::ListSchemasError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_schemas::ListSchemas::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_schemas::ListSchemas::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_schemas::ListSchemasOutput,
        crate::operation::list_schemas::ListSchemasError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_schemas::paginator::ListSchemasPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_schemas::paginator::ListSchemasPaginator {
        crate::operation::list_schemas::paginator::ListSchemasPaginator::new(self.handle, self.inner)
    }
    /// <p>The cluster identifier. This parameter is required when connecting to a cluster and authenticating using either Secrets Manager or temporary credentials.</p>
    pub fn cluster_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_identifier(input.into());
        self
    }
    /// <p>The cluster identifier. This parameter is required when connecting to a cluster and authenticating using either Secrets Manager or temporary credentials.</p>
    pub fn set_cluster_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_identifier(input);
        self
    }
    /// <p>The cluster identifier. This parameter is required when connecting to a cluster and authenticating using either Secrets Manager or temporary credentials.</p>
    pub fn get_cluster_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cluster_identifier()
    }
    /// <p>The name or ARN of the secret that enables access to the database. This parameter is required when authenticating using Secrets Manager.</p>
    pub fn secret_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.secret_arn(input.into());
        self
    }
    /// <p>The name or ARN of the secret that enables access to the database. This parameter is required when authenticating using Secrets Manager.</p>
    pub fn set_secret_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_secret_arn(input);
        self
    }
    /// <p>The name or ARN of the secret that enables access to the database. This parameter is required when authenticating using Secrets Manager.</p>
    pub fn get_secret_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_secret_arn()
    }
    /// <p>The database user name. This parameter is required when connecting to a cluster as a database user and authenticating using temporary credentials.</p>
    pub fn db_user(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_user(input.into());
        self
    }
    /// <p>The database user name. This parameter is required when connecting to a cluster as a database user and authenticating using temporary credentials.</p>
    pub fn set_db_user(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_user(input);
        self
    }
    /// <p>The database user name. This parameter is required when connecting to a cluster as a database user and authenticating using temporary credentials.</p>
    pub fn get_db_user(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_user()
    }
    /// <p>The name of the database that contains the schemas to list. If <code>ConnectedDatabase</code> is not specified, this is also the database to connect to with your authentication credentials.</p>
    pub fn database(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.database(input.into());
        self
    }
    /// <p>The name of the database that contains the schemas to list. If <code>ConnectedDatabase</code> is not specified, this is also the database to connect to with your authentication credentials.</p>
    pub fn set_database(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_database(input);
        self
    }
    /// <p>The name of the database that contains the schemas to list. If <code>ConnectedDatabase</code> is not specified, this is also the database to connect to with your authentication credentials.</p>
    pub fn get_database(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_database()
    }
    /// <p>A database name. The connected database is specified when you connect with your authentication credentials.</p>
    pub fn connected_database(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connected_database(input.into());
        self
    }
    /// <p>A database name. The connected database is specified when you connect with your authentication credentials.</p>
    pub fn set_connected_database(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connected_database(input);
        self
    }
    /// <p>A database name. The connected database is specified when you connect with your authentication credentials.</p>
    pub fn get_connected_database(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connected_database()
    }
    /// <p>A pattern to filter results by schema name. Within a schema pattern, "%" means match any substring of 0 or more characters and "_" means match any one character. Only schema name entries matching the search pattern are returned.</p>
    pub fn schema_pattern(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.schema_pattern(input.into());
        self
    }
    /// <p>A pattern to filter results by schema name. Within a schema pattern, "%" means match any substring of 0 or more characters and "_" means match any one character. Only schema name entries matching the search pattern are returned.</p>
    pub fn set_schema_pattern(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_schema_pattern(input);
        self
    }
    /// <p>A pattern to filter results by schema name. Within a schema pattern, "%" means match any substring of 0 or more characters and "_" means match any one character. Only schema name entries matching the search pattern are returned.</p>
    pub fn get_schema_pattern(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_schema_pattern()
    }
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned NextToken value in the next NextToken parameter and retrying the command. If the NextToken field is empty, all response records have been retrieved for the request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned NextToken value in the next NextToken parameter and retrying the command. If the NextToken field is empty, all response records have been retrieved for the request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A value that indicates the starting point for the next set of response records in a subsequent request. If a value is returned in a response, you can retrieve the next set of records by providing this returned NextToken value in the next NextToken parameter and retrying the command. If the NextToken field is empty, all response records have been retrieved for the request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of schemas to return in the response. If more schemas exist than fit in one response, then <code>NextToken</code> is returned to page through the results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of schemas to return in the response. If more schemas exist than fit in one response, then <code>NextToken</code> is returned to page through the results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of schemas to return in the response. If more schemas exist than fit in one response, then <code>NextToken</code> is returned to page through the results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The serverless workgroup name or Amazon Resource Name (ARN). This parameter is required when connecting to a serverless workgroup and authenticating using either Secrets Manager or temporary credentials.</p>
    pub fn workgroup_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workgroup_name(input.into());
        self
    }
    /// <p>The serverless workgroup name or Amazon Resource Name (ARN). This parameter is required when connecting to a serverless workgroup and authenticating using either Secrets Manager or temporary credentials.</p>
    pub fn set_workgroup_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workgroup_name(input);
        self
    }
    /// <p>The serverless workgroup name or Amazon Resource Name (ARN). This parameter is required when connecting to a serverless workgroup and authenticating using either Secrets Manager or temporary credentials.</p>
    pub fn get_workgroup_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workgroup_name()
    }
}
