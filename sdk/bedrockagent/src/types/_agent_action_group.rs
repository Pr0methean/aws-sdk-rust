// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Contains the information of an Agent Action Group
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AgentActionGroup {
    /// Identifier for a resource.
    pub agent_id: ::std::string::String,
    /// Agent Version.
    pub agent_version: ::std::string::String,
    /// Identifier for a resource.
    pub action_group_id: ::std::string::String,
    /// Name for a resource.
    pub action_group_name: ::std::string::String,
    /// Client specified token used for idempotency checks
    pub client_token: ::std::option::Option<::std::string::String>,
    /// Description of the Resource.
    pub description: ::std::option::Option<::std::string::String>,
    /// Time Stamp.
    pub created_at: ::aws_smithy_types::DateTime,
    /// Time Stamp.
    pub updated_at: ::aws_smithy_types::DateTime,
    /// Action Group Signature for a BuiltIn Action
    pub parent_action_signature: ::std::option::Option<crate::types::ActionGroupSignature>,
    /// Type of Executors for an Action Group
    pub action_group_executor: ::std::option::Option<crate::types::ActionGroupExecutor>,
    /// Contains information about the API Schema for the Action Group
    pub api_schema: ::std::option::Option<crate::types::ApiSchema>,
    /// State of the action group
    pub action_group_state: crate::types::ActionGroupState,
}
impl AgentActionGroup {
    /// Identifier for a resource.
    pub fn agent_id(&self) -> &str {
        use std::ops::Deref;
        self.agent_id.deref()
    }
    /// Agent Version.
    pub fn agent_version(&self) -> &str {
        use std::ops::Deref;
        self.agent_version.deref()
    }
    /// Identifier for a resource.
    pub fn action_group_id(&self) -> &str {
        use std::ops::Deref;
        self.action_group_id.deref()
    }
    /// Name for a resource.
    pub fn action_group_name(&self) -> &str {
        use std::ops::Deref;
        self.action_group_name.deref()
    }
    /// Client specified token used for idempotency checks
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// Description of the Resource.
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// Time Stamp.
    pub fn created_at(&self) -> &::aws_smithy_types::DateTime {
        &self.created_at
    }
    /// Time Stamp.
    pub fn updated_at(&self) -> &::aws_smithy_types::DateTime {
        &self.updated_at
    }
    /// Action Group Signature for a BuiltIn Action
    pub fn parent_action_signature(&self) -> ::std::option::Option<&crate::types::ActionGroupSignature> {
        self.parent_action_signature.as_ref()
    }
    /// Type of Executors for an Action Group
    pub fn action_group_executor(&self) -> ::std::option::Option<&crate::types::ActionGroupExecutor> {
        self.action_group_executor.as_ref()
    }
    /// Contains information about the API Schema for the Action Group
    pub fn api_schema(&self) -> ::std::option::Option<&crate::types::ApiSchema> {
        self.api_schema.as_ref()
    }
    /// State of the action group
    pub fn action_group_state(&self) -> &crate::types::ActionGroupState {
        &self.action_group_state
    }
}
impl AgentActionGroup {
    /// Creates a new builder-style object to manufacture [`AgentActionGroup`](crate::types::AgentActionGroup).
    pub fn builder() -> crate::types::builders::AgentActionGroupBuilder {
        crate::types::builders::AgentActionGroupBuilder::default()
    }
}

/// A builder for [`AgentActionGroup`](crate::types::AgentActionGroup).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AgentActionGroupBuilder {
    pub(crate) agent_id: ::std::option::Option<::std::string::String>,
    pub(crate) agent_version: ::std::option::Option<::std::string::String>,
    pub(crate) action_group_id: ::std::option::Option<::std::string::String>,
    pub(crate) action_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) parent_action_signature: ::std::option::Option<crate::types::ActionGroupSignature>,
    pub(crate) action_group_executor: ::std::option::Option<crate::types::ActionGroupExecutor>,
    pub(crate) api_schema: ::std::option::Option<crate::types::ApiSchema>,
    pub(crate) action_group_state: ::std::option::Option<crate::types::ActionGroupState>,
}
impl AgentActionGroupBuilder {
    /// Identifier for a resource.
    /// This field is required.
    pub fn agent_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier for a resource.
    pub fn set_agent_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_id = input;
        self
    }
    /// Identifier for a resource.
    pub fn get_agent_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.agent_id
    }
    /// Agent Version.
    /// This field is required.
    pub fn agent_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_version = ::std::option::Option::Some(input.into());
        self
    }
    /// Agent Version.
    pub fn set_agent_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_version = input;
        self
    }
    /// Agent Version.
    pub fn get_agent_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.agent_version
    }
    /// Identifier for a resource.
    /// This field is required.
    pub fn action_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.action_group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier for a resource.
    pub fn set_action_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.action_group_id = input;
        self
    }
    /// Identifier for a resource.
    pub fn get_action_group_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.action_group_id
    }
    /// Name for a resource.
    /// This field is required.
    pub fn action_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.action_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// Name for a resource.
    pub fn set_action_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.action_group_name = input;
        self
    }
    /// Name for a resource.
    pub fn get_action_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.action_group_name
    }
    /// Client specified token used for idempotency checks
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// Client specified token used for idempotency checks
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Client specified token used for idempotency checks
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// Description of the Resource.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// Description of the Resource.
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Description of the Resource.
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Time Stamp.
    /// This field is required.
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// Time Stamp.
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// Time Stamp.
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// Time Stamp.
    /// This field is required.
    pub fn updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_at = ::std::option::Option::Some(input);
        self
    }
    /// Time Stamp.
    pub fn set_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated_at = input;
        self
    }
    /// Time Stamp.
    pub fn get_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated_at
    }
    /// Action Group Signature for a BuiltIn Action
    pub fn parent_action_signature(mut self, input: crate::types::ActionGroupSignature) -> Self {
        self.parent_action_signature = ::std::option::Option::Some(input);
        self
    }
    /// Action Group Signature for a BuiltIn Action
    pub fn set_parent_action_signature(mut self, input: ::std::option::Option<crate::types::ActionGroupSignature>) -> Self {
        self.parent_action_signature = input;
        self
    }
    /// Action Group Signature for a BuiltIn Action
    pub fn get_parent_action_signature(&self) -> &::std::option::Option<crate::types::ActionGroupSignature> {
        &self.parent_action_signature
    }
    /// Type of Executors for an Action Group
    pub fn action_group_executor(mut self, input: crate::types::ActionGroupExecutor) -> Self {
        self.action_group_executor = ::std::option::Option::Some(input);
        self
    }
    /// Type of Executors for an Action Group
    pub fn set_action_group_executor(mut self, input: ::std::option::Option<crate::types::ActionGroupExecutor>) -> Self {
        self.action_group_executor = input;
        self
    }
    /// Type of Executors for an Action Group
    pub fn get_action_group_executor(&self) -> &::std::option::Option<crate::types::ActionGroupExecutor> {
        &self.action_group_executor
    }
    /// Contains information about the API Schema for the Action Group
    pub fn api_schema(mut self, input: crate::types::ApiSchema) -> Self {
        self.api_schema = ::std::option::Option::Some(input);
        self
    }
    /// Contains information about the API Schema for the Action Group
    pub fn set_api_schema(mut self, input: ::std::option::Option<crate::types::ApiSchema>) -> Self {
        self.api_schema = input;
        self
    }
    /// Contains information about the API Schema for the Action Group
    pub fn get_api_schema(&self) -> &::std::option::Option<crate::types::ApiSchema> {
        &self.api_schema
    }
    /// State of the action group
    /// This field is required.
    pub fn action_group_state(mut self, input: crate::types::ActionGroupState) -> Self {
        self.action_group_state = ::std::option::Option::Some(input);
        self
    }
    /// State of the action group
    pub fn set_action_group_state(mut self, input: ::std::option::Option<crate::types::ActionGroupState>) -> Self {
        self.action_group_state = input;
        self
    }
    /// State of the action group
    pub fn get_action_group_state(&self) -> &::std::option::Option<crate::types::ActionGroupState> {
        &self.action_group_state
    }
    /// Consumes the builder and constructs a [`AgentActionGroup`](crate::types::AgentActionGroup).
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](crate::types::builders::AgentActionGroupBuilder::agent_id)
    /// - [`agent_version`](crate::types::builders::AgentActionGroupBuilder::agent_version)
    /// - [`action_group_id`](crate::types::builders::AgentActionGroupBuilder::action_group_id)
    /// - [`action_group_name`](crate::types::builders::AgentActionGroupBuilder::action_group_name)
    /// - [`created_at`](crate::types::builders::AgentActionGroupBuilder::created_at)
    /// - [`updated_at`](crate::types::builders::AgentActionGroupBuilder::updated_at)
    /// - [`action_group_state`](crate::types::builders::AgentActionGroupBuilder::action_group_state)
    pub fn build(self) -> ::std::result::Result<crate::types::AgentActionGroup, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::AgentActionGroup {
            agent_id: self.agent_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_id",
                    "agent_id was not specified but it is required when building AgentActionGroup",
                )
            })?,
            agent_version: self.agent_version.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_version",
                    "agent_version was not specified but it is required when building AgentActionGroup",
                )
            })?,
            action_group_id: self.action_group_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "action_group_id",
                    "action_group_id was not specified but it is required when building AgentActionGroup",
                )
            })?,
            action_group_name: self.action_group_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "action_group_name",
                    "action_group_name was not specified but it is required when building AgentActionGroup",
                )
            })?,
            client_token: self.client_token,
            description: self.description,
            created_at: self.created_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "created_at",
                    "created_at was not specified but it is required when building AgentActionGroup",
                )
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "updated_at",
                    "updated_at was not specified but it is required when building AgentActionGroup",
                )
            })?,
            parent_action_signature: self.parent_action_signature,
            action_group_executor: self.action_group_executor,
            api_schema: self.api_schema,
            action_group_state: self.action_group_state.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "action_group_state",
                    "action_group_state was not specified but it is required when building AgentActionGroup",
                )
            })?,
        })
    }
}
