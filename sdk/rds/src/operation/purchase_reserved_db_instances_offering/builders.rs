// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::purchase_reserved_db_instances_offering::_purchase_reserved_db_instances_offering_output::PurchaseReservedDbInstancesOfferingOutputBuilder;

pub use crate::operation::purchase_reserved_db_instances_offering::_purchase_reserved_db_instances_offering_input::PurchaseReservedDbInstancesOfferingInputBuilder;

impl PurchaseReservedDbInstancesOfferingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDbInstancesOfferingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDBInstancesOfferingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.purchase_reserved_db_instances_offering();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PurchaseReservedDBInstancesOffering`.
///
/// <p>Purchases a reserved DB instance offering.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PurchaseReservedDBInstancesOfferingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::purchase_reserved_db_instances_offering::builders::PurchaseReservedDbInstancesOfferingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDbInstancesOfferingOutput,
        crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDBInstancesOfferingError,
    > for PurchaseReservedDBInstancesOfferingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDbInstancesOfferingOutput,
            crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDBInstancesOfferingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PurchaseReservedDBInstancesOfferingFluentBuilder {
    /// Creates a new `PurchaseReservedDBInstancesOffering`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PurchaseReservedDBInstancesOffering as a reference.
    pub fn as_input(&self) -> &crate::operation::purchase_reserved_db_instances_offering::builders::PurchaseReservedDbInstancesOfferingInputBuilder {
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
        crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDbInstancesOfferingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDBInstancesOfferingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDBInstancesOffering::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDBInstancesOffering::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDbInstancesOfferingOutput,
        crate::operation::purchase_reserved_db_instances_offering::PurchaseReservedDBInstancesOfferingError,
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
    /// <p>The ID of the Reserved DB instance offering to purchase.</p>
    /// <p>Example: 438012d3-4052-4cc7-b2e3-8d3372e0e706</p>
    pub fn reserved_db_instances_offering_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reserved_db_instances_offering_id(input.into());
        self
    }
    /// <p>The ID of the Reserved DB instance offering to purchase.</p>
    /// <p>Example: 438012d3-4052-4cc7-b2e3-8d3372e0e706</p>
    pub fn set_reserved_db_instances_offering_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_reserved_db_instances_offering_id(input);
        self
    }
    /// <p>The ID of the Reserved DB instance offering to purchase.</p>
    /// <p>Example: 438012d3-4052-4cc7-b2e3-8d3372e0e706</p>
    pub fn get_reserved_db_instances_offering_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_reserved_db_instances_offering_id()
    }
    /// <p>Customer-specified identifier to track this reservation.</p>
    /// <p>Example: myreservationID</p>
    pub fn reserved_db_instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reserved_db_instance_id(input.into());
        self
    }
    /// <p>Customer-specified identifier to track this reservation.</p>
    /// <p>Example: myreservationID</p>
    pub fn set_reserved_db_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_reserved_db_instance_id(input);
        self
    }
    /// <p>Customer-specified identifier to track this reservation.</p>
    /// <p>Example: myreservationID</p>
    pub fn get_reserved_db_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_reserved_db_instance_id()
    }
    /// <p>The number of instances to reserve.</p>
    /// <p>Default: <code>1</code></p>
    pub fn db_instance_count(mut self, input: i32) -> Self {
        self.inner = self.inner.db_instance_count(input);
        self
    }
    /// <p>The number of instances to reserve.</p>
    /// <p>Default: <code>1</code></p>
    pub fn set_db_instance_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_db_instance_count(input);
        self
    }
    /// <p>The number of instances to reserve.</p>
    /// <p>Default: <code>1</code></p>
    pub fn get_db_instance_count(&self) -> &::std::option::Option<i32> {
        self.inner.get_db_instance_count()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html">Tagging Amazon RDS Resources</a> in the <i>Amazon RDS User Guide.</i></p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of tags. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html">Tagging Amazon RDS Resources</a> in the <i>Amazon RDS User Guide.</i></p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of tags. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html">Tagging Amazon RDS Resources</a> in the <i>Amazon RDS User Guide.</i></p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
