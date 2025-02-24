// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_location_azure_blob::_create_location_azure_blob_output::CreateLocationAzureBlobOutputBuilder;

pub use crate::operation::create_location_azure_blob::_create_location_azure_blob_input::CreateLocationAzureBlobInputBuilder;

impl CreateLocationAzureBlobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_location_azure_blob::CreateLocationAzureBlobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_location_azure_blob::CreateLocationAzureBlobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_location_azure_blob();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateLocationAzureBlob`.
///
/// <p>Creates a transfer <i>location</i> for a Microsoft Azure Blob Storage container. DataSync can use this location as a transfer source or destination.</p>
/// <p>Before you begin, make sure you know <a href="https://docs.aws.amazon.com/datasync/latest/userguide/creating-azure-blob-location.html#azure-blob-access">how DataSync accesses Azure Blob Storage</a> and works with <a href="https://docs.aws.amazon.com/datasync/latest/userguide/creating-azure-blob-location.html#azure-blob-access-tiers">access tiers</a> and <a href="https://docs.aws.amazon.com/datasync/latest/userguide/creating-azure-blob-location.html#blob-types">blob types</a>. You also need a <a href="https://docs.aws.amazon.com/datasync/latest/userguide/creating-azure-blob-location.html#azure-blob-creating-agent">DataSync agent</a> that can connect to your container.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLocationAzureBlobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_location_azure_blob::builders::CreateLocationAzureBlobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_location_azure_blob::CreateLocationAzureBlobOutput,
        crate::operation::create_location_azure_blob::CreateLocationAzureBlobError,
    > for CreateLocationAzureBlobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_location_azure_blob::CreateLocationAzureBlobOutput,
            crate::operation::create_location_azure_blob::CreateLocationAzureBlobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateLocationAzureBlobFluentBuilder {
    /// Creates a new `CreateLocationAzureBlob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateLocationAzureBlob as a reference.
    pub fn as_input(&self) -> &crate::operation::create_location_azure_blob::builders::CreateLocationAzureBlobInputBuilder {
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
        crate::operation::create_location_azure_blob::CreateLocationAzureBlobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_location_azure_blob::CreateLocationAzureBlobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_location_azure_blob::CreateLocationAzureBlob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_location_azure_blob::CreateLocationAzureBlob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_location_azure_blob::CreateLocationAzureBlobOutput,
        crate::operation::create_location_azure_blob::CreateLocationAzureBlobError,
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
    /// <p>Specifies the URL of the Azure Blob Storage container involved in your transfer.</p>
    pub fn container_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.container_url(input.into());
        self
    }
    /// <p>Specifies the URL of the Azure Blob Storage container involved in your transfer.</p>
    pub fn set_container_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_container_url(input);
        self
    }
    /// <p>Specifies the URL of the Azure Blob Storage container involved in your transfer.</p>
    pub fn get_container_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_container_url()
    }
    /// <p>Specifies the authentication method DataSync uses to access your Azure Blob Storage. DataSync can access blob storage using a shared access signature (SAS).</p>
    pub fn authentication_type(mut self, input: crate::types::AzureBlobAuthenticationType) -> Self {
        self.inner = self.inner.authentication_type(input);
        self
    }
    /// <p>Specifies the authentication method DataSync uses to access your Azure Blob Storage. DataSync can access blob storage using a shared access signature (SAS).</p>
    pub fn set_authentication_type(mut self, input: ::std::option::Option<crate::types::AzureBlobAuthenticationType>) -> Self {
        self.inner = self.inner.set_authentication_type(input);
        self
    }
    /// <p>Specifies the authentication method DataSync uses to access your Azure Blob Storage. DataSync can access blob storage using a shared access signature (SAS).</p>
    pub fn get_authentication_type(&self) -> &::std::option::Option<crate::types::AzureBlobAuthenticationType> {
        self.inner.get_authentication_type()
    }
    /// <p>Specifies the SAS configuration that allows DataSync to access your Azure Blob Storage.</p>
    pub fn sas_configuration(mut self, input: crate::types::AzureBlobSasConfiguration) -> Self {
        self.inner = self.inner.sas_configuration(input);
        self
    }
    /// <p>Specifies the SAS configuration that allows DataSync to access your Azure Blob Storage.</p>
    pub fn set_sas_configuration(mut self, input: ::std::option::Option<crate::types::AzureBlobSasConfiguration>) -> Self {
        self.inner = self.inner.set_sas_configuration(input);
        self
    }
    /// <p>Specifies the SAS configuration that allows DataSync to access your Azure Blob Storage.</p>
    pub fn get_sas_configuration(&self) -> &::std::option::Option<crate::types::AzureBlobSasConfiguration> {
        self.inner.get_sas_configuration()
    }
    /// <p>Specifies the type of blob that you want your objects or files to be when transferring them into Azure Blob Storage. Currently, DataSync only supports moving data into Azure Blob Storage as block blobs. For more information on blob types, see the <a href="https://learn.microsoft.com/en-us/rest/api/storageservices/understanding-block-blobs--append-blobs--and-page-blobs">Azure Blob Storage documentation</a>.</p>
    pub fn blob_type(mut self, input: crate::types::AzureBlobType) -> Self {
        self.inner = self.inner.blob_type(input);
        self
    }
    /// <p>Specifies the type of blob that you want your objects or files to be when transferring them into Azure Blob Storage. Currently, DataSync only supports moving data into Azure Blob Storage as block blobs. For more information on blob types, see the <a href="https://learn.microsoft.com/en-us/rest/api/storageservices/understanding-block-blobs--append-blobs--and-page-blobs">Azure Blob Storage documentation</a>.</p>
    pub fn set_blob_type(mut self, input: ::std::option::Option<crate::types::AzureBlobType>) -> Self {
        self.inner = self.inner.set_blob_type(input);
        self
    }
    /// <p>Specifies the type of blob that you want your objects or files to be when transferring them into Azure Blob Storage. Currently, DataSync only supports moving data into Azure Blob Storage as block blobs. For more information on blob types, see the <a href="https://learn.microsoft.com/en-us/rest/api/storageservices/understanding-block-blobs--append-blobs--and-page-blobs">Azure Blob Storage documentation</a>.</p>
    pub fn get_blob_type(&self) -> &::std::option::Option<crate::types::AzureBlobType> {
        self.inner.get_blob_type()
    }
    /// <p>Specifies the access tier that you want your objects or files transferred into. This only applies when using the location as a transfer destination. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/creating-azure-blob-location.html#azure-blob-access-tiers">Access tiers</a>.</p>
    pub fn access_tier(mut self, input: crate::types::AzureAccessTier) -> Self {
        self.inner = self.inner.access_tier(input);
        self
    }
    /// <p>Specifies the access tier that you want your objects or files transferred into. This only applies when using the location as a transfer destination. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/creating-azure-blob-location.html#azure-blob-access-tiers">Access tiers</a>.</p>
    pub fn set_access_tier(mut self, input: ::std::option::Option<crate::types::AzureAccessTier>) -> Self {
        self.inner = self.inner.set_access_tier(input);
        self
    }
    /// <p>Specifies the access tier that you want your objects or files transferred into. This only applies when using the location as a transfer destination. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/creating-azure-blob-location.html#azure-blob-access-tiers">Access tiers</a>.</p>
    pub fn get_access_tier(&self) -> &::std::option::Option<crate::types::AzureAccessTier> {
        self.inner.get_access_tier()
    }
    /// <p>Specifies path segments if you want to limit your transfer to a virtual directory in your container (for example, <code>/my/images</code>).</p>
    pub fn subdirectory(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subdirectory(input.into());
        self
    }
    /// <p>Specifies path segments if you want to limit your transfer to a virtual directory in your container (for example, <code>/my/images</code>).</p>
    pub fn set_subdirectory(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_subdirectory(input);
        self
    }
    /// <p>Specifies path segments if you want to limit your transfer to a virtual directory in your container (for example, <code>/my/images</code>).</p>
    pub fn get_subdirectory(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_subdirectory()
    }
    /// Appends an item to `AgentArns`.
    ///
    /// To override the contents of this collection use [`set_agent_arns`](Self::set_agent_arns).
    ///
    /// <p>Specifies the Amazon Resource Name (ARN) of the DataSync agent that can connect with your Azure Blob Storage container.</p>
    /// <p>You can specify more than one agent. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/multiple-agents.html">Using multiple agents for your transfer</a>.</p>
    pub fn agent_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.agent_arns(input.into());
        self
    }
    /// <p>Specifies the Amazon Resource Name (ARN) of the DataSync agent that can connect with your Azure Blob Storage container.</p>
    /// <p>You can specify more than one agent. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/multiple-agents.html">Using multiple agents for your transfer</a>.</p>
    pub fn set_agent_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_agent_arns(input);
        self
    }
    /// <p>Specifies the Amazon Resource Name (ARN) of the DataSync agent that can connect with your Azure Blob Storage container.</p>
    /// <p>You can specify more than one agent. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/multiple-agents.html">Using multiple agents for your transfer</a>.</p>
    pub fn get_agent_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_agent_arns()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your transfer location.</p>
    pub fn tags(mut self, input: crate::types::TagListEntry) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your transfer location.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagListEntry>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your transfer location.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagListEntry>> {
        self.inner.get_tags()
    }
}
