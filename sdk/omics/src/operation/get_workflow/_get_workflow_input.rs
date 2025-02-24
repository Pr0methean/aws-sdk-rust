// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetWorkflowInput {
    /// <p>The workflow's ID.</p>
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The workflow's type.</p>
    pub r#type: ::std::option::Option<crate::types::WorkflowType>,
    /// <p>The export format for the workflow.</p>
    pub export: ::std::option::Option<::std::vec::Vec<crate::types::WorkflowExport>>,
}
impl GetWorkflowInput {
    /// <p>The workflow's ID.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The workflow's type.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::WorkflowType> {
        self.r#type.as_ref()
    }
    /// <p>The export format for the workflow.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.export.is_none()`.
    pub fn export(&self) -> &[crate::types::WorkflowExport] {
        self.export.as_deref().unwrap_or_default()
    }
}
impl GetWorkflowInput {
    /// Creates a new builder-style object to manufacture [`GetWorkflowInput`](crate::operation::get_workflow::GetWorkflowInput).
    pub fn builder() -> crate::operation::get_workflow::builders::GetWorkflowInputBuilder {
        crate::operation::get_workflow::builders::GetWorkflowInputBuilder::default()
    }
}

/// A builder for [`GetWorkflowInput`](crate::operation::get_workflow::GetWorkflowInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetWorkflowInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::WorkflowType>,
    pub(crate) export: ::std::option::Option<::std::vec::Vec<crate::types::WorkflowExport>>,
}
impl GetWorkflowInputBuilder {
    /// <p>The workflow's ID.</p>
    /// This field is required.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The workflow's ID.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The workflow's ID.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// <p>The workflow's type.</p>
    pub fn r#type(mut self, input: crate::types::WorkflowType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The workflow's type.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::WorkflowType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The workflow's type.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::WorkflowType> {
        &self.r#type
    }
    /// Appends an item to `export`.
    ///
    /// To override the contents of this collection use [`set_export`](Self::set_export).
    ///
    /// <p>The export format for the workflow.</p>
    pub fn export(mut self, input: crate::types::WorkflowExport) -> Self {
        let mut v = self.export.unwrap_or_default();
        v.push(input);
        self.export = ::std::option::Option::Some(v);
        self
    }
    /// <p>The export format for the workflow.</p>
    pub fn set_export(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::WorkflowExport>>) -> Self {
        self.export = input;
        self
    }
    /// <p>The export format for the workflow.</p>
    pub fn get_export(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::WorkflowExport>> {
        &self.export
    }
    /// Consumes the builder and constructs a [`GetWorkflowInput`](crate::operation::get_workflow::GetWorkflowInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_workflow::GetWorkflowInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_workflow::GetWorkflowInput {
            id: self.id,
            r#type: self.r#type,
            export: self.export,
        })
    }
}
