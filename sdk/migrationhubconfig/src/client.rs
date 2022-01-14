// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    pub(crate) client: aws_smithy_client::Client<C, M, R>,
    pub(crate) conf: crate::Config,
}

/// Client for AWS Migration Hub Config
///
/// Client for invoking operations on AWS Migration Hub Config. Each operation on AWS Migration Hub Config is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_migrationhubconfig::Client::new(&shared_config);
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
///     let shared_config = aws_config::load_from_env().await;
///     let config = aws_sdk_migrationhubconfig::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_migrationhubconfig::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the [`CreateHomeRegionControl`](crate::client::fluent_builders::CreateHomeRegionControl) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`home_region(impl Into<String>)`](crate::client::fluent_builders::CreateHomeRegionControl::home_region) / [`set_home_region(Option<String>)`](crate::client::fluent_builders::CreateHomeRegionControl::set_home_region): <p>The name of the home region of the calling account.</p>
    ///   - [`target(Target)`](crate::client::fluent_builders::CreateHomeRegionControl::target) / [`set_target(Option<Target>)`](crate::client::fluent_builders::CreateHomeRegionControl::set_target): <p>The account for which this command sets up a home region control. The <code>Target</code> is always of type <code>ACCOUNT</code>.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateHomeRegionControl::dry_run) / [`set_dry_run(bool)`](crate::client::fluent_builders::CreateHomeRegionControl::set_dry_run): <p>Optional Boolean flag to indicate whether any effect should take place. It tests whether the caller has permission to make the call.</p>
    /// - On success, responds with [`CreateHomeRegionControlOutput`](crate::output::CreateHomeRegionControlOutput) with field(s):
    ///   - [`home_region_control(Option<HomeRegionControl>)`](crate::output::CreateHomeRegionControlOutput::home_region_control): <p>This object is the <code>HomeRegionControl</code> object that's returned by a successful call to <code>CreateHomeRegionControl</code>.</p>
    /// - On failure, responds with [`SdkError<CreateHomeRegionControlError>`](crate::error::CreateHomeRegionControlError)
    pub fn create_home_region_control(&self) -> fluent_builders::CreateHomeRegionControl<C, M, R> {
        fluent_builders::CreateHomeRegionControl::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`DescribeHomeRegionControls`](crate::client::fluent_builders::DescribeHomeRegionControls) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeHomeRegionControls::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`control_id(impl Into<String>)`](crate::client::fluent_builders::DescribeHomeRegionControls::control_id) / [`set_control_id(Option<String>)`](crate::client::fluent_builders::DescribeHomeRegionControls::set_control_id): <p>The <code>ControlID</code> is a unique identifier string of your <code>HomeRegionControl</code> object.</p>
    ///   - [`home_region(impl Into<String>)`](crate::client::fluent_builders::DescribeHomeRegionControls::home_region) / [`set_home_region(Option<String>)`](crate::client::fluent_builders::DescribeHomeRegionControls::set_home_region): <p>The name of the home region you'd like to view.</p>
    ///   - [`target(Target)`](crate::client::fluent_builders::DescribeHomeRegionControls::target) / [`set_target(Option<Target>)`](crate::client::fluent_builders::DescribeHomeRegionControls::set_target): <p>The target parameter specifies the identifier to which the home region is applied, which is always of type <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeHomeRegionControls::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeHomeRegionControls::set_max_results): <p>The maximum number of filtering results to display per page. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeHomeRegionControls::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeHomeRegionControls::set_next_token): <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    /// - On success, responds with [`DescribeHomeRegionControlsOutput`](crate::output::DescribeHomeRegionControlsOutput) with field(s):
    ///   - [`home_region_controls(Option<Vec<HomeRegionControl>>)`](crate::output::DescribeHomeRegionControlsOutput::home_region_controls): <p>An array that contains your <code>HomeRegionControl</code> objects.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeHomeRegionControlsOutput::next_token): <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    /// - On failure, responds with [`SdkError<DescribeHomeRegionControlsError>`](crate::error::DescribeHomeRegionControlsError)
    pub fn describe_home_region_controls(
        &self,
    ) -> fluent_builders::DescribeHomeRegionControls<C, M, R> {
        fluent_builders::DescribeHomeRegionControls::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`GetHomeRegion`](crate::client::fluent_builders::GetHomeRegion) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::GetHomeRegion::send) it.

    /// - On success, responds with [`GetHomeRegionOutput`](crate::output::GetHomeRegionOutput) with field(s):
    ///   - [`home_region(Option<String>)`](crate::output::GetHomeRegionOutput::home_region): <p>The name of the home region of the calling account.</p>
    /// - On failure, responds with [`SdkError<GetHomeRegionError>`](crate::error::GetHomeRegionError)
    pub fn get_home_region(&self) -> fluent_builders::GetHomeRegion<C, M, R> {
        fluent_builders::GetHomeRegion::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `CreateHomeRegionControl`.
    ///
    /// <p>This API sets up the home region for the calling account only.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct CreateHomeRegionControl<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_home_region_control_input::Builder,
    }
    impl<C, M, R> CreateHomeRegionControl<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `CreateHomeRegionControl`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
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
            crate::output::CreateHomeRegionControlOutput,
            aws_smithy_http::result::SdkError<crate::error::CreateHomeRegionControlError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateHomeRegionControlInputOperationOutputAlias,
                crate::output::CreateHomeRegionControlOutput,
                crate::error::CreateHomeRegionControlError,
                crate::input::CreateHomeRegionControlInputOperationRetryAlias,
            >,
        {
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
        /// <p>The name of the home region of the calling account.</p>
        pub fn home_region(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.home_region(input.into());
            self
        }
        /// <p>The name of the home region of the calling account.</p>
        pub fn set_home_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_home_region(input);
            self
        }
        /// <p>The account for which this command sets up a home region control. The <code>Target</code> is always of type <code>ACCOUNT</code>.</p>
        pub fn target(mut self, input: crate::model::Target) -> Self {
            self.inner = self.inner.target(input);
            self
        }
        /// <p>The account for which this command sets up a home region control. The <code>Target</code> is always of type <code>ACCOUNT</code>.</p>
        pub fn set_target(mut self, input: std::option::Option<crate::model::Target>) -> Self {
            self.inner = self.inner.set_target(input);
            self
        }
        /// <p>Optional Boolean flag to indicate whether any effect should take place. It tests whether the caller has permission to make the call.</p>
        pub fn dry_run(mut self, input: bool) -> Self {
            self.inner = self.inner.dry_run(input);
            self
        }
        /// <p>Optional Boolean flag to indicate whether any effect should take place. It tests whether the caller has permission to make the call.</p>
        pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_dry_run(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DescribeHomeRegionControls`.
    ///
    /// <p>This API permits filtering on the <code>ControlId</code> and <code>HomeRegion</code> fields.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DescribeHomeRegionControls<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_home_region_controls_input::Builder,
    }
    impl<C, M, R> DescribeHomeRegionControls<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DescribeHomeRegionControls`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
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
            crate::output::DescribeHomeRegionControlsOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeHomeRegionControlsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeHomeRegionControlsInputOperationOutputAlias,
                crate::output::DescribeHomeRegionControlsOutput,
                crate::error::DescribeHomeRegionControlsError,
                crate::input::DescribeHomeRegionControlsInputOperationRetryAlias,
            >,
        {
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
        /// Paginators are used by calling [`send().await`](crate::paginator::DescribeHomeRegionControlsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(
            self,
        ) -> crate::paginator::DescribeHomeRegionControlsPaginator<C, M, R> {
            crate::paginator::DescribeHomeRegionControlsPaginator::new(self.handle, self.inner)
        }
        /// <p>The <code>ControlID</code> is a unique identifier string of your <code>HomeRegionControl</code> object.</p>
        pub fn control_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.control_id(input.into());
            self
        }
        /// <p>The <code>ControlID</code> is a unique identifier string of your <code>HomeRegionControl</code> object.</p>
        pub fn set_control_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_control_id(input);
            self
        }
        /// <p>The name of the home region you'd like to view.</p>
        pub fn home_region(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.home_region(input.into());
            self
        }
        /// <p>The name of the home region you'd like to view.</p>
        pub fn set_home_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_home_region(input);
            self
        }
        /// <p>The target parameter specifies the identifier to which the home region is applied, which is always of type <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
        pub fn target(mut self, input: crate::model::Target) -> Self {
            self.inner = self.inner.target(input);
            self
        }
        /// <p>The target parameter specifies the identifier to which the home region is applied, which is always of type <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
        pub fn set_target(mut self, input: std::option::Option<crate::model::Target>) -> Self {
            self.inner = self.inner.set_target(input);
            self
        }
        /// <p>The maximum number of filtering results to display per page. </p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of filtering results to display per page. </p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetHomeRegion`.
    ///
    /// <p>Returns the calling account’s home region, if configured. This API is used by other AWS services to determine the regional endpoint for calling AWS Application Discovery Service and Migration Hub. You must call <code>GetHomeRegion</code> at least once before you call any other AWS Application Discovery Service and AWS Migration Hub APIs, to obtain the account's Migration Hub home region.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetHomeRegion<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_home_region_input::Builder,
    }
    impl<C, M, R> GetHomeRegion<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetHomeRegion`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
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
            crate::output::GetHomeRegionOutput,
            aws_smithy_http::result::SdkError<crate::error::GetHomeRegionError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetHomeRegionInputOperationOutputAlias,
                crate::output::GetHomeRegionOutput,
                crate::error::GetHomeRegionError,
                crate::input::GetHomeRegionInputOperationRetryAlias,
            >,
        {
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
}

impl<C> Client<C, crate::middleware::DefaultMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(conn)
            .middleware(crate::middleware::DefaultMiddleware::new());
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
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        crate::middleware::DefaultMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https()
            .middleware(crate::middleware::DefaultMiddleware::new());
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
