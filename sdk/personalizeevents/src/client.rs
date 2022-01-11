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

/// Client for Amazon Personalize Events
///
/// Client for invoking operations on Amazon Personalize Events. Each operation on Amazon Personalize Events is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_personalizeevents::Client::new(&shared_config);
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
///     let config = aws_sdk_personalizeevents::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_personalizeevents::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`PutEvents`](crate::client::fluent_builders::PutEvents) operation.
    ///
    /// - Takes [`PutEventsInput`](crate::input::PutEventsInput) with field(s):
    ///   - [`tracking_id(Option<String>)`](crate::input::PutEventsInput::tracking_id): <p>The tracking ID for the event. The ID is generated by a call to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
    ///   - [`user_id(Option<String>)`](crate::input::PutEventsInput::user_id): <p>The user associated with the event.</p>
    ///   - [`session_id(Option<String>)`](crate::input::PutEventsInput::session_id): <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application. Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
    ///   - [`event_list(Option<Vec<Event>>)`](crate::input::PutEventsInput::event_list): <p>A list of event data from the session.</p>
    /// - On success, responds with [`PutEventsOutput`](crate::output::PutEventsOutput)

    /// - On failure, responds with [`SdkError<PutEventsError>`](crate::error::PutEventsError)
    pub fn put_events(&self) -> fluent_builders::PutEvents<C, M, R> {
        fluent_builders::PutEvents::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`PutItems`](crate::client::fluent_builders::PutItems) operation.
    ///
    /// - Takes [`PutItemsInput`](crate::input::PutItemsInput) with field(s):
    ///   - [`dataset_arn(Option<String>)`](crate::input::PutItemsInput::dataset_arn): <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
    ///   - [`items(Option<Vec<Item>>)`](crate::input::PutItemsInput::items): <p>A list of item data.</p>
    /// - On success, responds with [`PutItemsOutput`](crate::output::PutItemsOutput)

    /// - On failure, responds with [`SdkError<PutItemsError>`](crate::error::PutItemsError)
    pub fn put_items(&self) -> fluent_builders::PutItems<C, M, R> {
        fluent_builders::PutItems::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`PutUsers`](crate::client::fluent_builders::PutUsers) operation.
    ///
    /// - Takes [`PutUsersInput`](crate::input::PutUsersInput) with field(s):
    ///   - [`dataset_arn(Option<String>)`](crate::input::PutUsersInput::dataset_arn): <p>The Amazon Resource Name (ARN) of the Users dataset you are adding the user or users to.</p>
    ///   - [`users(Option<Vec<User>>)`](crate::input::PutUsersInput::users): <p>A list of user data.</p>
    /// - On success, responds with [`PutUsersOutput`](crate::output::PutUsersOutput)

    /// - On failure, responds with [`SdkError<PutUsersError>`](crate::error::PutUsersError)
    pub fn put_users(&self) -> fluent_builders::PutUsers<C, M, R> {
        fluent_builders::PutUsers::new(self.handle.clone())
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
    /// Fluent builder constructing a request to `PutEvents`.
    ///
    /// <p>Records user interaction event data. For more information see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct PutEvents<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::put_events_input::Builder,
    }
    impl<C, M, R> PutEvents<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `PutEvents`.
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
            crate::output::PutEventsOutput,
            aws_smithy_http::result::SdkError<crate::error::PutEventsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::PutEventsInputOperationOutputAlias,
                crate::output::PutEventsOutput,
                crate::error::PutEventsError,
                crate::input::PutEventsInputOperationRetryAlias,
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
        /// <p>The tracking ID for the event. The ID is generated by a call to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
        pub fn tracking_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tracking_id(input.into());
            self
        }
        /// <p>The tracking ID for the event. The ID is generated by a call to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
        pub fn set_tracking_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_tracking_id(input);
            self
        }
        /// <p>The user associated with the event.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.user_id(input.into());
            self
        }
        /// <p>The user associated with the event.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_user_id(input);
            self
        }
        /// <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application. Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
        pub fn session_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_id(input.into());
            self
        }
        /// <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application. Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
        pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_session_id(input);
            self
        }
        /// Appends an item to `eventList`.
        ///
        /// To override the contents of this collection use [`set_event_list`](Self::set_event_list).
        ///
        /// <p>A list of event data from the session.</p>
        pub fn event_list(mut self, input: crate::model::Event) -> Self {
            self.inner = self.inner.event_list(input);
            self
        }
        /// <p>A list of event data from the session.</p>
        pub fn set_event_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Event>>,
        ) -> Self {
            self.inner = self.inner.set_event_list(input);
            self
        }
    }
    /// Fluent builder constructing a request to `PutItems`.
    ///
    /// <p>Adds one or more items to an Items dataset. For more information see <a href="https://docs.aws.amazon.com/personalize/latest/dg/importing-items.html">Importing Items Incrementally</a>. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct PutItems<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::put_items_input::Builder,
    }
    impl<C, M, R> PutItems<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `PutItems`.
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
            crate::output::PutItemsOutput,
            aws_smithy_http::result::SdkError<crate::error::PutItemsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::PutItemsInputOperationOutputAlias,
                crate::output::PutItemsOutput,
                crate::error::PutItemsError,
                crate::input::PutItemsInputOperationRetryAlias,
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
        /// <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
        pub fn dataset_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.dataset_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
        pub fn set_dataset_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_dataset_arn(input);
            self
        }
        /// Appends an item to `items`.
        ///
        /// To override the contents of this collection use [`set_items`](Self::set_items).
        ///
        /// <p>A list of item data.</p>
        pub fn items(mut self, input: crate::model::Item) -> Self {
            self.inner = self.inner.items(input);
            self
        }
        /// <p>A list of item data.</p>
        pub fn set_items(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Item>>,
        ) -> Self {
            self.inner = self.inner.set_items(input);
            self
        }
    }
    /// Fluent builder constructing a request to `PutUsers`.
    ///
    /// <p>Adds one or more users to a Users dataset. For more information see <a href="https://docs.aws.amazon.com/personalize/latest/dg/importing-users.html">Importing Users Incrementally</a>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct PutUsers<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::put_users_input::Builder,
    }
    impl<C, M, R> PutUsers<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `PutUsers`.
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
            crate::output::PutUsersOutput,
            aws_smithy_http::result::SdkError<crate::error::PutUsersError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::PutUsersInputOperationOutputAlias,
                crate::output::PutUsersOutput,
                crate::error::PutUsersError,
                crate::input::PutUsersInputOperationRetryAlias,
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
        /// <p>The Amazon Resource Name (ARN) of the Users dataset you are adding the user or users to.</p>
        pub fn dataset_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.dataset_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the Users dataset you are adding the user or users to.</p>
        pub fn set_dataset_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_dataset_arn(input);
            self
        }
        /// Appends an item to `users`.
        ///
        /// To override the contents of this collection use [`set_users`](Self::set_users).
        ///
        /// <p>A list of user data.</p>
        pub fn users(mut self, input: crate::model::User) -> Self {
            self.inner = self.inner.users(input);
            self
        }
        /// <p>A list of user data.</p>
        pub fn set_users(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::User>>,
        ) -> Self {
            self.inner = self.inner.set_users(input);
            self
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
