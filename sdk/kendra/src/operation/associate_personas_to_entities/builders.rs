// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_personas_to_entities::_associate_personas_to_entities_output::AssociatePersonasToEntitiesOutputBuilder;

pub use crate::operation::associate_personas_to_entities::_associate_personas_to_entities_input::AssociatePersonasToEntitiesInputBuilder;

impl AssociatePersonasToEntitiesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_personas_to_entities::AssociatePersonasToEntitiesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_personas_to_entities::AssociatePersonasToEntitiesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_personas_to_entities();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociatePersonasToEntities`.
///
/// <p>Defines the specific permissions of users or groups in your IAM Identity Center identity source with access to your Amazon Kendra experience. You can create an Amazon Kendra experience such as a search application. For more information on creating a search application experience, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/deploying-search-experience-no-code.html">Building a search experience with no code</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociatePersonasToEntitiesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_personas_to_entities::builders::AssociatePersonasToEntitiesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_personas_to_entities::AssociatePersonasToEntitiesOutput,
        crate::operation::associate_personas_to_entities::AssociatePersonasToEntitiesError,
    > for AssociatePersonasToEntitiesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_personas_to_entities::AssociatePersonasToEntitiesOutput,
            crate::operation::associate_personas_to_entities::AssociatePersonasToEntitiesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociatePersonasToEntitiesFluentBuilder {
    /// Creates a new `AssociatePersonasToEntities`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociatePersonasToEntities as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_personas_to_entities::builders::AssociatePersonasToEntitiesInputBuilder {
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
        crate::operation::associate_personas_to_entities::AssociatePersonasToEntitiesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_personas_to_entities::AssociatePersonasToEntitiesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_personas_to_entities::AssociatePersonasToEntities::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_personas_to_entities::AssociatePersonasToEntities::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_personas_to_entities::AssociatePersonasToEntitiesOutput,
        crate::operation::associate_personas_to_entities::AssociatePersonasToEntitiesError,
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
    /// <p>The identifier of your Amazon Kendra experience.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The identifier of your Amazon Kendra experience.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The identifier of your Amazon Kendra experience.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The identifier of the index for your Amazon Kendra experience.</p>
    pub fn index_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.index_id(input.into());
        self
    }
    /// <p>The identifier of the index for your Amazon Kendra experience.</p>
    pub fn set_index_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_index_id(input);
        self
    }
    /// <p>The identifier of the index for your Amazon Kendra experience.</p>
    pub fn get_index_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_index_id()
    }
    /// Appends an item to `Personas`.
    ///
    /// To override the contents of this collection use [`set_personas`](Self::set_personas).
    ///
    /// <p>The personas that define the specific permissions of users or groups in your IAM Identity Center identity source. The available personas or access roles are <code>Owner</code> and <code>Viewer</code>. For more information on these personas, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/deploying-search-experience-no-code.html#access-search-experience">Providing access to your search page</a>.</p>
    pub fn personas(mut self, input: crate::types::EntityPersonaConfiguration) -> Self {
        self.inner = self.inner.personas(input);
        self
    }
    /// <p>The personas that define the specific permissions of users or groups in your IAM Identity Center identity source. The available personas or access roles are <code>Owner</code> and <code>Viewer</code>. For more information on these personas, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/deploying-search-experience-no-code.html#access-search-experience">Providing access to your search page</a>.</p>
    pub fn set_personas(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::EntityPersonaConfiguration>>) -> Self {
        self.inner = self.inner.set_personas(input);
        self
    }
    /// <p>The personas that define the specific permissions of users or groups in your IAM Identity Center identity source. The available personas or access roles are <code>Owner</code> and <code>Viewer</code>. For more information on these personas, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/deploying-search-experience-no-code.html#access-search-experience">Providing access to your search page</a>.</p>
    pub fn get_personas(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EntityPersonaConfiguration>> {
        self.inner.get_personas()
    }
}
