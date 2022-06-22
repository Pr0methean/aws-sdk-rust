// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`CreateEndpointInput`](crate::input::CreateEndpointInput)
pub mod create_endpoint_input {

    /// A builder for [`CreateEndpointInput`](crate::input::CreateEndpointInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) outpost_id: std::option::Option<std::string::String>,
        pub(crate) subnet_id: std::option::Option<std::string::String>,
        pub(crate) security_group_id: std::option::Option<std::string::String>,
        pub(crate) access_type: std::option::Option<crate::model::EndpointAccessType>,
        pub(crate) customer_owned_ipv4_pool: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the Outposts. </p>
        pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.outpost_id = Some(input.into());
            self
        }
        /// <p>The ID of the Outposts. </p>
        pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.outpost_id = input;
            self
        }
        /// <p>The ID of the subnet in the selected VPC. The endpoint subnet must belong to the Outpost that has Amazon S3 on Outposts provisioned.</p>
        pub fn subnet_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.subnet_id = Some(input.into());
            self
        }
        /// <p>The ID of the subnet in the selected VPC. The endpoint subnet must belong to the Outpost that has Amazon S3 on Outposts provisioned.</p>
        pub fn set_subnet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.subnet_id = input;
            self
        }
        /// <p>The ID of the security group to use with the endpoint.</p>
        pub fn security_group_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.security_group_id = Some(input.into());
            self
        }
        /// <p>The ID of the security group to use with the endpoint.</p>
        pub fn set_security_group_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.security_group_id = input;
            self
        }
        /// <p>The type of access for the network connectivity for the Amazon S3 on Outposts endpoint. To use the Amazon Web Services VPC, choose <code>Private</code>. To use the endpoint with an on-premises network, choose <code>CustomerOwnedIp</code>. If you choose <code>CustomerOwnedIp</code>, you must also provide the customer-owned IP address pool (CoIP pool).</p> <note>
        /// <p> <code>Private</code> is the default access type value.</p>
        /// </note>
        pub fn access_type(mut self, input: crate::model::EndpointAccessType) -> Self {
            self.access_type = Some(input);
            self
        }
        /// <p>The type of access for the network connectivity for the Amazon S3 on Outposts endpoint. To use the Amazon Web Services VPC, choose <code>Private</code>. To use the endpoint with an on-premises network, choose <code>CustomerOwnedIp</code>. If you choose <code>CustomerOwnedIp</code>, you must also provide the customer-owned IP address pool (CoIP pool).</p> <note>
        /// <p> <code>Private</code> is the default access type value.</p>
        /// </note>
        pub fn set_access_type(
            mut self,
            input: std::option::Option<crate::model::EndpointAccessType>,
        ) -> Self {
            self.access_type = input;
            self
        }
        /// <p>The ID of the customer-owned IPv4 address pool (CoIP pool) for the endpoint. IP addresses are allocated from this pool for the endpoint.</p>
        pub fn customer_owned_ipv4_pool(mut self, input: impl Into<std::string::String>) -> Self {
            self.customer_owned_ipv4_pool = Some(input.into());
            self
        }
        /// <p>The ID of the customer-owned IPv4 address pool (CoIP pool) for the endpoint. IP addresses are allocated from this pool for the endpoint.</p>
        pub fn set_customer_owned_ipv4_pool(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.customer_owned_ipv4_pool = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateEndpointInput`](crate::input::CreateEndpointInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::CreateEndpointInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::CreateEndpointInput {
                outpost_id: self.outpost_id,
                subnet_id: self.subnet_id,
                security_group_id: self.security_group_id,
                access_type: self.access_type,
                customer_owned_ipv4_pool: self.customer_owned_ipv4_pool,
            })
        }
    }
}
#[doc(hidden)]
pub type CreateEndpointInputOperationOutputAlias = crate::operation::CreateEndpoint;
#[doc(hidden)]
pub type CreateEndpointInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl CreateEndpointInput {
    /// Consumes the builder and constructs an Operation<[`CreateEndpoint`](crate::operation::CreateEndpoint)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::CreateEndpoint,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::CreateEndpointInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/S3Outposts/CreateEndpoint").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::CreateEndpointInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/json",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_create_endpoint(&self)?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::CreateEndpoint::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "CreateEndpoint",
            "s3outposts",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`CreateEndpointInput`](crate::input::CreateEndpointInput)
    pub fn builder() -> crate::input::create_endpoint_input::Builder {
        crate::input::create_endpoint_input::Builder::default()
    }
}

/// See [`DeleteEndpointInput`](crate::input::DeleteEndpointInput)
pub mod delete_endpoint_input {

    /// A builder for [`DeleteEndpointInput`](crate::input::DeleteEndpointInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) endpoint_id: std::option::Option<std::string::String>,
        pub(crate) outpost_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the endpoint.</p>
        pub fn endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.endpoint_id = Some(input.into());
            self
        }
        /// <p>The ID of the endpoint.</p>
        pub fn set_endpoint_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.endpoint_id = input;
            self
        }
        /// <p>The ID of the Outposts. </p>
        pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.outpost_id = Some(input.into());
            self
        }
        /// <p>The ID of the Outposts. </p>
        pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.outpost_id = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteEndpointInput`](crate::input::DeleteEndpointInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::DeleteEndpointInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::DeleteEndpointInput {
                endpoint_id: self.endpoint_id,
                outpost_id: self.outpost_id,
            })
        }
    }
}
#[doc(hidden)]
pub type DeleteEndpointInputOperationOutputAlias = crate::operation::DeleteEndpoint;
#[doc(hidden)]
pub type DeleteEndpointInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl DeleteEndpointInput {
    /// Consumes the builder and constructs an Operation<[`DeleteEndpoint`](crate::operation::DeleteEndpoint)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::DeleteEndpoint,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::DeleteEndpointInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/S3Outposts/DeleteEndpoint").expect("formatting should succeed");
                Ok(())
            }
            fn uri_query(
                _input: &crate::input::DeleteEndpointInput,
                mut output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                let mut query = aws_smithy_http::query::Writer::new(&mut output);
                if let Some(inner_1) = &_input.endpoint_id {
                    query.push_kv("endpointId", &aws_smithy_http::query::fmt_string(&inner_1));
                }
                if let Some(inner_2) = &_input.outpost_id {
                    query.push_kv("outpostId", &aws_smithy_http::query::fmt_string(&inner_2));
                }
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::DeleteEndpointInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                Ok(builder.method("DELETE").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::DeleteEndpoint::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "DeleteEndpoint",
            "s3outposts",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`DeleteEndpointInput`](crate::input::DeleteEndpointInput)
    pub fn builder() -> crate::input::delete_endpoint_input::Builder {
        crate::input::delete_endpoint_input::Builder::default()
    }
}

/// See [`ListEndpointsInput`](crate::input::ListEndpointsInput)
pub mod list_endpoints_input {

    /// A builder for [`ListEndpointsInput`](crate::input::ListEndpointsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>If a previous response from this operation included a <code>NextToken</code> value, provide that value here to retrieve the next page of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>If a previous response from this operation included a <code>NextToken</code> value, provide that value here to retrieve the next page of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// <p>The maximum number of endpoints that will be returned in the response.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        /// <p>The maximum number of endpoints that will be returned in the response.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// Consumes the builder and constructs a [`ListEndpointsInput`](crate::input::ListEndpointsInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::ListEndpointsInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::ListEndpointsInput {
                next_token: self.next_token,
                max_results: self.max_results.unwrap_or_default(),
            })
        }
    }
}
#[doc(hidden)]
pub type ListEndpointsInputOperationOutputAlias = crate::operation::ListEndpoints;
#[doc(hidden)]
pub type ListEndpointsInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl ListEndpointsInput {
    /// Consumes the builder and constructs an Operation<[`ListEndpoints`](crate::operation::ListEndpoints)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::ListEndpoints,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::ListEndpointsInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/S3Outposts/ListEndpoints").expect("formatting should succeed");
                Ok(())
            }
            fn uri_query(
                _input: &crate::input::ListEndpointsInput,
                mut output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                let mut query = aws_smithy_http::query::Writer::new(&mut output);
                if let Some(inner_3) = &_input.next_token {
                    query.push_kv("nextToken", &aws_smithy_http::query::fmt_string(&inner_3));
                }
                if _input.max_results != 0 {
                    query.push_kv(
                        "maxResults",
                        aws_smithy_types::primitive::Encoder::from(_input.max_results).encode(),
                    );
                }
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::ListEndpointsInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::ListEndpoints::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "ListEndpoints",
            "s3outposts",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`ListEndpointsInput`](crate::input::ListEndpointsInput)
    pub fn builder() -> crate::input::list_endpoints_input::Builder {
        crate::input::list_endpoints_input::Builder::default()
    }
}

/// See [`ListSharedEndpointsInput`](crate::input::ListSharedEndpointsInput)
pub mod list_shared_endpoints_input {

    /// A builder for [`ListSharedEndpointsInput`](crate::input::ListSharedEndpointsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
        pub(crate) outpost_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>If a previous response from this operation included a <code>NextToken</code> value, you can provide that value here to retrieve the next page of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>If a previous response from this operation included a <code>NextToken</code> value, you can provide that value here to retrieve the next page of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// <p>The maximum number of endpoints that will be returned in the response.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        /// <p>The maximum number of endpoints that will be returned in the response.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// <p>The ID of the Amazon Web Services Outpost.</p>
        pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.outpost_id = Some(input.into());
            self
        }
        /// <p>The ID of the Amazon Web Services Outpost.</p>
        pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.outpost_id = input;
            self
        }
        /// Consumes the builder and constructs a [`ListSharedEndpointsInput`](crate::input::ListSharedEndpointsInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::ListSharedEndpointsInput,
            aws_smithy_http::operation::BuildError,
        > {
            Ok(crate::input::ListSharedEndpointsInput {
                next_token: self.next_token,
                max_results: self.max_results.unwrap_or_default(),
                outpost_id: self.outpost_id,
            })
        }
    }
}
#[doc(hidden)]
pub type ListSharedEndpointsInputOperationOutputAlias = crate::operation::ListSharedEndpoints;
#[doc(hidden)]
pub type ListSharedEndpointsInputOperationRetryAlias = aws_http::retry::AwsErrorRetryPolicy;
impl ListSharedEndpointsInput {
    /// Consumes the builder and constructs an Operation<[`ListSharedEndpoints`](crate::operation::ListSharedEndpoints)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::ListSharedEndpoints,
            aws_http::retry::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::ListSharedEndpointsInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/S3Outposts/ListSharedEndpoints")
                    .expect("formatting should succeed");
                Ok(())
            }
            fn uri_query(
                _input: &crate::input::ListSharedEndpointsInput,
                mut output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                let mut query = aws_smithy_http::query::Writer::new(&mut output);
                if let Some(inner_4) = &_input.next_token {
                    query.push_kv("nextToken", &aws_smithy_http::query::fmt_string(&inner_4));
                }
                if _input.max_results != 0 {
                    query.push_kv(
                        "maxResults",
                        aws_smithy_types::primitive::Encoder::from(_input.max_results).encode(),
                    );
                }
                if let Some(inner_5) = &_input.outpost_id {
                    query.push_kv("outpostId", &aws_smithy_http::query::fmt_string(&inner_5));
                }
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::ListSharedEndpointsInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::ListSharedEndpoints::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "ListSharedEndpoints",
            "s3outposts",
        ));
        let op = op.with_retry_policy(aws_http::retry::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`ListSharedEndpointsInput`](crate::input::ListSharedEndpointsInput)
    pub fn builder() -> crate::input::list_shared_endpoints_input::Builder {
        crate::input::list_shared_endpoints_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListSharedEndpointsInput {
    /// <p>If a previous response from this operation included a <code>NextToken</code> value, you can provide that value here to retrieve the next page of results.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of endpoints that will be returned in the response.</p>
    pub max_results: i32,
    /// <p>The ID of the Amazon Web Services Outpost.</p>
    pub outpost_id: std::option::Option<std::string::String>,
}
impl ListSharedEndpointsInput {
    /// <p>If a previous response from this operation included a <code>NextToken</code> value, you can provide that value here to retrieve the next page of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of endpoints that will be returned in the response.</p>
    pub fn max_results(&self) -> i32 {
        self.max_results
    }
    /// <p>The ID of the Amazon Web Services Outpost.</p>
    pub fn outpost_id(&self) -> std::option::Option<&str> {
        self.outpost_id.as_deref()
    }
}
impl std::fmt::Debug for ListSharedEndpointsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListSharedEndpointsInput");
        formatter.field("next_token", &self.next_token);
        formatter.field("max_results", &self.max_results);
        formatter.field("outpost_id", &self.outpost_id);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListEndpointsInput {
    /// <p>If a previous response from this operation included a <code>NextToken</code> value, provide that value here to retrieve the next page of results.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of endpoints that will be returned in the response.</p>
    pub max_results: i32,
}
impl ListEndpointsInput {
    /// <p>If a previous response from this operation included a <code>NextToken</code> value, provide that value here to retrieve the next page of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of endpoints that will be returned in the response.</p>
    pub fn max_results(&self) -> i32 {
        self.max_results
    }
}
impl std::fmt::Debug for ListEndpointsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListEndpointsInput");
        formatter.field("next_token", &self.next_token);
        formatter.field("max_results", &self.max_results);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteEndpointInput {
    /// <p>The ID of the endpoint.</p>
    pub endpoint_id: std::option::Option<std::string::String>,
    /// <p>The ID of the Outposts. </p>
    pub outpost_id: std::option::Option<std::string::String>,
}
impl DeleteEndpointInput {
    /// <p>The ID of the endpoint.</p>
    pub fn endpoint_id(&self) -> std::option::Option<&str> {
        self.endpoint_id.as_deref()
    }
    /// <p>The ID of the Outposts. </p>
    pub fn outpost_id(&self) -> std::option::Option<&str> {
        self.outpost_id.as_deref()
    }
}
impl std::fmt::Debug for DeleteEndpointInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteEndpointInput");
        formatter.field("endpoint_id", &self.endpoint_id);
        formatter.field("outpost_id", &self.outpost_id);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateEndpointInput {
    /// <p>The ID of the Outposts. </p>
    pub outpost_id: std::option::Option<std::string::String>,
    /// <p>The ID of the subnet in the selected VPC. The endpoint subnet must belong to the Outpost that has Amazon S3 on Outposts provisioned.</p>
    pub subnet_id: std::option::Option<std::string::String>,
    /// <p>The ID of the security group to use with the endpoint.</p>
    pub security_group_id: std::option::Option<std::string::String>,
    /// <p>The type of access for the network connectivity for the Amazon S3 on Outposts endpoint. To use the Amazon Web Services VPC, choose <code>Private</code>. To use the endpoint with an on-premises network, choose <code>CustomerOwnedIp</code>. If you choose <code>CustomerOwnedIp</code>, you must also provide the customer-owned IP address pool (CoIP pool).</p> <note>
    /// <p> <code>Private</code> is the default access type value.</p>
    /// </note>
    pub access_type: std::option::Option<crate::model::EndpointAccessType>,
    /// <p>The ID of the customer-owned IPv4 address pool (CoIP pool) for the endpoint. IP addresses are allocated from this pool for the endpoint.</p>
    pub customer_owned_ipv4_pool: std::option::Option<std::string::String>,
}
impl CreateEndpointInput {
    /// <p>The ID of the Outposts. </p>
    pub fn outpost_id(&self) -> std::option::Option<&str> {
        self.outpost_id.as_deref()
    }
    /// <p>The ID of the subnet in the selected VPC. The endpoint subnet must belong to the Outpost that has Amazon S3 on Outposts provisioned.</p>
    pub fn subnet_id(&self) -> std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>The ID of the security group to use with the endpoint.</p>
    pub fn security_group_id(&self) -> std::option::Option<&str> {
        self.security_group_id.as_deref()
    }
    /// <p>The type of access for the network connectivity for the Amazon S3 on Outposts endpoint. To use the Amazon Web Services VPC, choose <code>Private</code>. To use the endpoint with an on-premises network, choose <code>CustomerOwnedIp</code>. If you choose <code>CustomerOwnedIp</code>, you must also provide the customer-owned IP address pool (CoIP pool).</p> <note>
    /// <p> <code>Private</code> is the default access type value.</p>
    /// </note>
    pub fn access_type(&self) -> std::option::Option<&crate::model::EndpointAccessType> {
        self.access_type.as_ref()
    }
    /// <p>The ID of the customer-owned IPv4 address pool (CoIP pool) for the endpoint. IP addresses are allocated from this pool for the endpoint.</p>
    pub fn customer_owned_ipv4_pool(&self) -> std::option::Option<&str> {
        self.customer_owned_ipv4_pool.as_deref()
    }
}
impl std::fmt::Debug for CreateEndpointInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateEndpointInput");
        formatter.field("outpost_id", &self.outpost_id);
        formatter.field("subnet_id", &self.subnet_id);
        formatter.field("security_group_id", &self.security_group_id);
        formatter.field("access_type", &self.access_type);
        formatter.field("customer_owned_ipv4_pool", &self.customer_owned_ipv4_pool);
        formatter.finish()
    }
}
