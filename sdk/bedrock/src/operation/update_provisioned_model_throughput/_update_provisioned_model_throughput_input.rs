// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateProvisionedModelThroughputInput {
    /// <p>The ARN or name of the provisioned throughput to update.</p>
    pub provisioned_model_id: ::std::option::Option<::std::string::String>,
    /// <p>The new name for this provisioned throughput.</p>
    pub desired_provisioned_model_name: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the new model to associate with this provisioned throughput.</p>
    pub desired_model_id: ::std::option::Option<::std::string::String>,
}
impl UpdateProvisionedModelThroughputInput {
    /// <p>The ARN or name of the provisioned throughput to update.</p>
    pub fn provisioned_model_id(&self) -> ::std::option::Option<&str> {
        self.provisioned_model_id.as_deref()
    }
    /// <p>The new name for this provisioned throughput.</p>
    pub fn desired_provisioned_model_name(&self) -> ::std::option::Option<&str> {
        self.desired_provisioned_model_name.as_deref()
    }
    /// <p>The ARN of the new model to associate with this provisioned throughput.</p>
    pub fn desired_model_id(&self) -> ::std::option::Option<&str> {
        self.desired_model_id.as_deref()
    }
}
impl UpdateProvisionedModelThroughputInput {
    /// Creates a new builder-style object to manufacture [`UpdateProvisionedModelThroughputInput`](crate::operation::update_provisioned_model_throughput::UpdateProvisionedModelThroughputInput).
    pub fn builder() -> crate::operation::update_provisioned_model_throughput::builders::UpdateProvisionedModelThroughputInputBuilder {
        crate::operation::update_provisioned_model_throughput::builders::UpdateProvisionedModelThroughputInputBuilder::default()
    }
}

/// A builder for [`UpdateProvisionedModelThroughputInput`](crate::operation::update_provisioned_model_throughput::UpdateProvisionedModelThroughputInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateProvisionedModelThroughputInputBuilder {
    pub(crate) provisioned_model_id: ::std::option::Option<::std::string::String>,
    pub(crate) desired_provisioned_model_name: ::std::option::Option<::std::string::String>,
    pub(crate) desired_model_id: ::std::option::Option<::std::string::String>,
}
impl UpdateProvisionedModelThroughputInputBuilder {
    /// <p>The ARN or name of the provisioned throughput to update.</p>
    /// This field is required.
    pub fn provisioned_model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.provisioned_model_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN or name of the provisioned throughput to update.</p>
    pub fn set_provisioned_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.provisioned_model_id = input;
        self
    }
    /// <p>The ARN or name of the provisioned throughput to update.</p>
    pub fn get_provisioned_model_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.provisioned_model_id
    }
    /// <p>The new name for this provisioned throughput.</p>
    pub fn desired_provisioned_model_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.desired_provisioned_model_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The new name for this provisioned throughput.</p>
    pub fn set_desired_provisioned_model_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.desired_provisioned_model_name = input;
        self
    }
    /// <p>The new name for this provisioned throughput.</p>
    pub fn get_desired_provisioned_model_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.desired_provisioned_model_name
    }
    /// <p>The ARN of the new model to associate with this provisioned throughput.</p>
    pub fn desired_model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.desired_model_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the new model to associate with this provisioned throughput.</p>
    pub fn set_desired_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.desired_model_id = input;
        self
    }
    /// <p>The ARN of the new model to associate with this provisioned throughput.</p>
    pub fn get_desired_model_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.desired_model_id
    }
    /// Consumes the builder and constructs a [`UpdateProvisionedModelThroughputInput`](crate::operation::update_provisioned_model_throughput::UpdateProvisionedModelThroughputInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_provisioned_model_throughput::UpdateProvisionedModelThroughputInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_provisioned_model_throughput::UpdateProvisionedModelThroughputInput {
                provisioned_model_id: self.provisioned_model_id,
                desired_provisioned_model_name: self.desired_provisioned_model_name,
                desired_model_id: self.desired_model_id,
            },
        )
    }
}
