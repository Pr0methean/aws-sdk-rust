// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for Amazon Elastic  Inference
///
/// Client for invoking operations on Amazon Elastic  Inference. Each operation on Amazon Elastic  Inference is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_elasticinference::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
/// let shared_config = aws_config::load_from_env().await;
/// let config = aws_sdk_elasticinference::config::Builder::from(&shared_config)
///   .retry_config(RetryConfig::disabled())
///   .build();
/// let client = aws_sdk_elasticinference::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client {
    handle: std::sync::Arc<Handle>,
}

impl std::clone::Clone for Client {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl
    From<
        aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    > for Client
{
    fn from(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    ) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl Client {
    /// Creates a client with the given service configuration.
    pub fn with_config(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
        conf: crate::Config,
    ) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    /// Constructs a fluent builder for the [`DescribeAcceleratorOfferings`](crate::client::fluent_builders::DescribeAcceleratorOfferings) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`location_type(LocationType)`](crate::client::fluent_builders::DescribeAcceleratorOfferings::location_type) / [`set_location_type(Option<LocationType>)`](crate::client::fluent_builders::DescribeAcceleratorOfferings::set_location_type): <p> The location type that you want to describe accelerator type offerings for. It can assume the following values: region: will return the accelerator type offering at the regional level. availability-zone: will return the accelerator type offering at the availability zone level. availability-zone-id: will return the accelerator type offering at the availability zone level returning the availability zone id. </p>
    ///   - [`accelerator_types(Vec<String>)`](crate::client::fluent_builders::DescribeAcceleratorOfferings::accelerator_types) / [`set_accelerator_types(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeAcceleratorOfferings::set_accelerator_types): <p> The list of accelerator types to describe. </p>
    /// - On success, responds with [`DescribeAcceleratorOfferingsOutput`](crate::output::DescribeAcceleratorOfferingsOutput) with field(s):
    ///   - [`accelerator_type_offerings(Option<Vec<AcceleratorTypeOffering>>)`](crate::output::DescribeAcceleratorOfferingsOutput::accelerator_type_offerings): <p> The list of accelerator type offerings for a specific location. </p>
    /// - On failure, responds with [`SdkError<DescribeAcceleratorOfferingsError>`](crate::error::DescribeAcceleratorOfferingsError)
    pub fn describe_accelerator_offerings(&self) -> fluent_builders::DescribeAcceleratorOfferings {
        fluent_builders::DescribeAcceleratorOfferings::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`DescribeAccelerators`](crate::client::fluent_builders::DescribeAccelerators) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeAccelerators::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`accelerator_ids(Vec<String>)`](crate::client::fluent_builders::DescribeAccelerators::accelerator_ids) / [`set_accelerator_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeAccelerators::set_accelerator_ids): <p> The IDs of the accelerators to describe. </p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeAccelerators::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeAccelerators::set_filters): <p> One or more filters. Filter names and values are case-sensitive. Valid filter names are: accelerator-types: can provide a list of accelerator type names to filter for. instance-id: can provide a list of EC2 instance ids to filter for. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeAccelerators::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::DescribeAccelerators::set_max_results): <p> The total number of items to return in the command's output. If the total number of items available is more than the value specified, a NextToken is provided in the command's output. To resume pagination, provide the NextToken value in the starting-token argument of a subsequent command. Do not use the NextToken response element directly outside of the AWS CLI. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeAccelerators::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeAccelerators::set_next_token): <p> A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    /// - On success, responds with [`DescribeAcceleratorsOutput`](crate::output::DescribeAcceleratorsOutput) with field(s):
    ///   - [`accelerator_set(Option<Vec<ElasticInferenceAccelerator>>)`](crate::output::DescribeAcceleratorsOutput::accelerator_set): <p> The details of the Elastic Inference Accelerators. </p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeAcceleratorsOutput::next_token): <p> A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    /// - On failure, responds with [`SdkError<DescribeAcceleratorsError>`](crate::error::DescribeAcceleratorsError)
    pub fn describe_accelerators(&self) -> fluent_builders::DescribeAccelerators {
        fluent_builders::DescribeAccelerators::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`DescribeAcceleratorTypes`](crate::client::fluent_builders::DescribeAcceleratorTypes) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DescribeAcceleratorTypes::send) it.

    /// - On success, responds with [`DescribeAcceleratorTypesOutput`](crate::output::DescribeAcceleratorTypesOutput) with field(s):
    ///   - [`accelerator_types(Option<Vec<AcceleratorType>>)`](crate::output::DescribeAcceleratorTypesOutput::accelerator_types): <p> The available accelerator types. </p>
    /// - On failure, responds with [`SdkError<DescribeAcceleratorTypesError>`](crate::error::DescribeAcceleratorTypesError)
    pub fn describe_accelerator_types(&self) -> fluent_builders::DescribeAcceleratorTypes {
        fluent_builders::DescribeAcceleratorTypes::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::client::fluent_builders::ListTagsForResource) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::ListTagsForResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::ListTagsForResource::set_resource_arn): <p> The ARN of the Elastic Inference Accelerator to list the tags for. </p>
    /// - On success, responds with [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput) with field(s):
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::ListTagsForResourceOutput::tags): <p> The tags of the Elastic Inference Accelerator. </p>
    /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::error::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> fluent_builders::ListTagsForResource {
        fluent_builders::ListTagsForResource::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`TagResource`](crate::client::fluent_builders::TagResource) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::TagResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::TagResource::set_resource_arn): <p> The ARN of the Elastic Inference Accelerator to tag. </p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::TagResource::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::TagResource::set_tags): <p> The tags to add to the Elastic Inference Accelerator. </p>
    /// - On success, responds with [`TagResourceOutput`](crate::output::TagResourceOutput)

    /// - On failure, responds with [`SdkError<TagResourceError>`](crate::error::TagResourceError)
    pub fn tag_resource(&self) -> fluent_builders::TagResource {
        fluent_builders::TagResource::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`UntagResource`](crate::client::fluent_builders::UntagResource) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::UntagResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::UntagResource::set_resource_arn): <p> The ARN of the Elastic Inference Accelerator to untag. </p>
    ///   - [`tag_keys(Vec<String>)`](crate::client::fluent_builders::UntagResource::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::client::fluent_builders::UntagResource::set_tag_keys): <p> The list of tags to remove from the Elastic Inference Accelerator. </p>
    /// - On success, responds with [`UntagResourceOutput`](crate::output::UntagResourceOutput)

    /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::error::UntagResourceError)
    pub fn untag_resource(&self) -> fluent_builders::UntagResource {
        fluent_builders::UntagResource::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `DescribeAcceleratorOfferings`.
    ///
    /// <p> Describes the locations in which a given accelerator type or set of types is present in a given region. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DescribeAcceleratorOfferings {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::describe_accelerator_offerings_input::Builder,
    }
    impl DescribeAcceleratorOfferings {
        /// Creates a new `DescribeAcceleratorOfferings`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::DescribeAcceleratorOfferingsOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeAcceleratorOfferingsError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p> The location type that you want to describe accelerator type offerings for. It can assume the following values: region: will return the accelerator type offering at the regional level. availability-zone: will return the accelerator type offering at the availability zone level. availability-zone-id: will return the accelerator type offering at the availability zone level returning the availability zone id. </p>
        pub fn location_type(mut self, input: crate::model::LocationType) -> Self {
            self.inner = self.inner.location_type(input);
            self
        }
        /// <p> The location type that you want to describe accelerator type offerings for. It can assume the following values: region: will return the accelerator type offering at the regional level. availability-zone: will return the accelerator type offering at the availability zone level. availability-zone-id: will return the accelerator type offering at the availability zone level returning the availability zone id. </p>
        pub fn set_location_type(
            mut self,
            input: std::option::Option<crate::model::LocationType>,
        ) -> Self {
            self.inner = self.inner.set_location_type(input);
            self
        }
        /// Appends an item to `acceleratorTypes`.
        ///
        /// To override the contents of this collection use [`set_accelerator_types`](Self::set_accelerator_types).
        ///
        /// <p> The list of accelerator types to describe. </p>
        pub fn accelerator_types(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.accelerator_types(input.into());
            self
        }
        /// <p> The list of accelerator types to describe. </p>
        pub fn set_accelerator_types(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_accelerator_types(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DescribeAccelerators`.
    ///
    /// <p> Describes information over a provided set of accelerators belonging to an account. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DescribeAccelerators {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::describe_accelerators_input::Builder,
    }
    impl DescribeAccelerators {
        /// Creates a new `DescribeAccelerators`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::DescribeAcceleratorsOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeAcceleratorsError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// Create a paginator for this request
        ///
        /// Paginators are used by calling [`send().await`](crate::paginator::DescribeAcceleratorsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::DescribeAcceleratorsPaginator {
            crate::paginator::DescribeAcceleratorsPaginator::new(self.handle, self.inner)
        }
        /// Appends an item to `acceleratorIds`.
        ///
        /// To override the contents of this collection use [`set_accelerator_ids`](Self::set_accelerator_ids).
        ///
        /// <p> The IDs of the accelerators to describe. </p>
        pub fn accelerator_ids(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.accelerator_ids(input.into());
            self
        }
        /// <p> The IDs of the accelerators to describe. </p>
        pub fn set_accelerator_ids(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_accelerator_ids(input);
            self
        }
        /// Appends an item to `filters`.
        ///
        /// To override the contents of this collection use [`set_filters`](Self::set_filters).
        ///
        /// <p> One or more filters. Filter names and values are case-sensitive. Valid filter names are: accelerator-types: can provide a list of accelerator type names to filter for. instance-id: can provide a list of EC2 instance ids to filter for. </p>
        pub fn filters(mut self, input: crate::model::Filter) -> Self {
            self.inner = self.inner.filters(input);
            self
        }
        /// <p> One or more filters. Filter names and values are case-sensitive. Valid filter names are: accelerator-types: can provide a list of accelerator type names to filter for. instance-id: can provide a list of EC2 instance ids to filter for. </p>
        pub fn set_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.inner = self.inner.set_filters(input);
            self
        }
        /// <p> The total number of items to return in the command's output. If the total number of items available is more than the value specified, a NextToken is provided in the command's output. To resume pagination, provide the NextToken value in the starting-token argument of a subsequent command. Do not use the NextToken response element directly outside of the AWS CLI. </p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p> The total number of items to return in the command's output. If the total number of items available is more than the value specified, a NextToken is provided in the command's output. To resume pagination, provide the NextToken value in the starting-token argument of a subsequent command. Do not use the NextToken response element directly outside of the AWS CLI. </p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p> A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p> A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DescribeAcceleratorTypes`.
    ///
    /// <p> Describes the accelerator types available in a given region, as well as their characteristics, such as memory and throughput. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DescribeAcceleratorTypes {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::describe_accelerator_types_input::Builder,
    }
    impl DescribeAcceleratorTypes {
        /// Creates a new `DescribeAcceleratorTypes`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::DescribeAcceleratorTypesOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeAcceleratorTypesError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
    }
    /// Fluent builder constructing a request to `ListTagsForResource`.
    ///
    /// <p> Returns all tags of an Elastic Inference Accelerator. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListTagsForResource {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::list_tags_for_resource_input::Builder,
    }
    impl ListTagsForResource {
        /// Creates a new `ListTagsForResource`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::ListTagsForResourceOutput,
            aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p> The ARN of the Elastic Inference Accelerator to list the tags for. </p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input.into());
            self
        }
        /// <p> The ARN of the Elastic Inference Accelerator to list the tags for. </p>
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
    }
    /// Fluent builder constructing a request to `TagResource`.
    ///
    /// <p> Adds the specified tags to an Elastic Inference Accelerator. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct TagResource {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::tag_resource_input::Builder,
    }
    impl TagResource {
        /// Creates a new `TagResource`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::TagResourceOutput,
            aws_smithy_http::result::SdkError<crate::error::TagResourceError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p> The ARN of the Elastic Inference Accelerator to tag. </p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input.into());
            self
        }
        /// <p> The ARN of the Elastic Inference Accelerator to tag. </p>
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p> The tags to add to the Elastic Inference Accelerator. </p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k.into(), v.into());
            self
        }
        /// <p> The tags to add to the Elastic Inference Accelerator. </p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    /// Fluent builder constructing a request to `UntagResource`.
    ///
    /// <p> Removes the specified tags from an Elastic Inference Accelerator. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct UntagResource {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::untag_resource_input::Builder,
    }
    impl UntagResource {
        /// Creates a new `UntagResource`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::UntagResourceOutput,
            aws_smithy_http::result::SdkError<crate::error::UntagResourceError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p> The ARN of the Elastic Inference Accelerator to untag. </p>
        pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(input.into());
            self
        }
        /// <p> The ARN of the Elastic Inference Accelerator to untag. </p>
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// Appends an item to `tagKeys`.
        ///
        /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
        ///
        /// <p> The list of tags to remove from the Elastic Inference Accelerator. </p>
        pub fn tag_keys(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tag_keys(input.into());
            self
        }
        /// <p> The list of tags to remove from the Elastic Inference Accelerator. </p>
        pub fn set_tag_keys(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_tag_keys(input);
            self
        }
    }
}

impl Client {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn<C, E>(conf: crate::Config, conn: C) -> Self
    where
        C: aws_smithy_client::bounds::SmithyConnector<Error = E> + Send + 'static,
        E: Into<aws_smithy_http::result::ConnectorError>,
    {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(aws_smithy_client::erase::DynConnector::new(conn))
            .middleware(aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ));
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https().middleware(
            aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ),
        );
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
