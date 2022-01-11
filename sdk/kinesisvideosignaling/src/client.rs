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

/// Client for Amazon Kinesis Video Signaling Channels
///
/// Client for invoking operations on Amazon Kinesis Video Signaling Channels. Each operation on Amazon Kinesis Video Signaling Channels is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_kinesisvideosignaling::Client::new(&shared_config);
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
///     let config = aws_sdk_kinesisvideosignaling::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_kinesisvideosignaling::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`GetIceServerConfig`](crate::client::fluent_builders::GetIceServerConfig) operation.
    ///
    /// - Takes [`GetIceServerConfigInput`](crate::input::GetIceServerConfigInput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::input::GetIceServerConfigInput::channel_arn): <p>The ARN of the signaling channel to be used for the peer-to-peer connection between configured peers. </p>
    ///   - [`client_id(Option<String>)`](crate::input::GetIceServerConfigInput::client_id): <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
    ///   - [`service(Option<Service>)`](crate::input::GetIceServerConfigInput::service): <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid value.</p>
    ///   - [`username(Option<String>)`](crate::input::GetIceServerConfigInput::username): <p>An optional user ID to be associated with the credentials.</p>
    /// - On success, responds with [`GetIceServerConfigOutput`](crate::output::GetIceServerConfigOutput) with field(s):
    ///   - [`ice_server_list(Option<Vec<IceServer>>)`](crate::output::GetIceServerConfigOutput::ice_server_list): <p>The list of ICE server information objects.</p>
    /// - On failure, responds with [`SdkError<GetIceServerConfigError>`](crate::error::GetIceServerConfigError)
    pub fn get_ice_server_config(&self) -> fluent_builders::GetIceServerConfig<C, M, R> {
        fluent_builders::GetIceServerConfig::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`SendAlexaOfferToMaster`](crate::client::fluent_builders::SendAlexaOfferToMaster) operation.
    ///
    /// - Takes [`SendAlexaOfferToMasterInput`](crate::input::SendAlexaOfferToMasterInput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::input::SendAlexaOfferToMasterInput::channel_arn): <p>The ARN of the signaling channel by which Alexa and the master peer communicate.</p>
    ///   - [`sender_client_id(Option<String>)`](crate::input::SendAlexaOfferToMasterInput::sender_client_id): <p>The unique identifier for the sender client.</p>
    ///   - [`message_payload(Option<String>)`](crate::input::SendAlexaOfferToMasterInput::message_payload): <p>The base64-encoded SDP offer content.</p>
    /// - On success, responds with [`SendAlexaOfferToMasterOutput`](crate::output::SendAlexaOfferToMasterOutput) with field(s):
    ///   - [`answer(Option<String>)`](crate::output::SendAlexaOfferToMasterOutput::answer): <p>The base64-encoded SDP answer content.</p>
    /// - On failure, responds with [`SdkError<SendAlexaOfferToMasterError>`](crate::error::SendAlexaOfferToMasterError)
    pub fn send_alexa_offer_to_master(&self) -> fluent_builders::SendAlexaOfferToMaster<C, M, R> {
        fluent_builders::SendAlexaOfferToMaster::new(self.handle.clone())
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
    /// Fluent builder constructing a request to `GetIceServerConfig`.
    ///
    /// <p>Gets the Interactive Connectivity Establishment (ICE) server configuration information, including URIs, username, and password which can be used to configure the WebRTC connection. The ICE component uses this configuration information to setup the WebRTC connection, including authenticating with the Traversal Using Relays around NAT (TURN) relay server. </p>
    /// <p>TURN is a protocol that is used to improve the connectivity of peer-to-peer applications. By providing a cloud-based relay service, TURN ensures that a connection can be established even when one or more peers are incapable of a direct peer-to-peer connection. For more information, see <a href="https://tools.ietf.org/html/draft-uberti-rtcweb-turn-rest-00">A REST API For Access To TURN Services</a>.</p>
    /// <p> You can invoke this API to establish a fallback mechanism in case either of the peers is unable to establish a direct peer-to-peer connection over a signaling channel. You must specify either a signaling channel ARN or the client ID in order to invoke this API.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetIceServerConfig<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_ice_server_config_input::Builder,
    }
    impl<C, M, R> GetIceServerConfig<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetIceServerConfig`.
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
            crate::output::GetIceServerConfigOutput,
            aws_smithy_http::result::SdkError<crate::error::GetIceServerConfigError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetIceServerConfigInputOperationOutputAlias,
                crate::output::GetIceServerConfigOutput,
                crate::error::GetIceServerConfigError,
                crate::input::GetIceServerConfigInputOperationRetryAlias,
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
        /// <p>The ARN of the signaling channel to be used for the peer-to-peer connection between configured peers. </p>
        pub fn channel_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.channel_arn(input.into());
            self
        }
        /// <p>The ARN of the signaling channel to be used for the peer-to-peer connection between configured peers. </p>
        pub fn set_channel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_channel_arn(input);
            self
        }
        /// <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
        pub fn client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_id(input.into());
            self
        }
        /// <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_id(input);
            self
        }
        /// <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid value.</p>
        pub fn service(mut self, input: crate::model::Service) -> Self {
            self.inner = self.inner.service(input);
            self
        }
        /// <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid value.</p>
        pub fn set_service(mut self, input: std::option::Option<crate::model::Service>) -> Self {
            self.inner = self.inner.set_service(input);
            self
        }
        /// <p>An optional user ID to be associated with the credentials.</p>
        pub fn username(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.username(input.into());
            self
        }
        /// <p>An optional user ID to be associated with the credentials.</p>
        pub fn set_username(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_username(input);
            self
        }
    }
    /// Fluent builder constructing a request to `SendAlexaOfferToMaster`.
    ///
    /// <p>This API allows you to connect WebRTC-enabled devices with Alexa display devices. When invoked, it sends the Alexa Session Description Protocol (SDP) offer to the master peer. The offer is delivered as soon as the master is connected to the specified signaling channel. This API returns the SDP answer from the connected master. If the master is not connected to the signaling channel, redelivery requests are made until the message expires.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct SendAlexaOfferToMaster<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::send_alexa_offer_to_master_input::Builder,
    }
    impl<C, M, R> SendAlexaOfferToMaster<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `SendAlexaOfferToMaster`.
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
            crate::output::SendAlexaOfferToMasterOutput,
            aws_smithy_http::result::SdkError<crate::error::SendAlexaOfferToMasterError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::SendAlexaOfferToMasterInputOperationOutputAlias,
                crate::output::SendAlexaOfferToMasterOutput,
                crate::error::SendAlexaOfferToMasterError,
                crate::input::SendAlexaOfferToMasterInputOperationRetryAlias,
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
        /// <p>The ARN of the signaling channel by which Alexa and the master peer communicate.</p>
        pub fn channel_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.channel_arn(input.into());
            self
        }
        /// <p>The ARN of the signaling channel by which Alexa and the master peer communicate.</p>
        pub fn set_channel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_channel_arn(input);
            self
        }
        /// <p>The unique identifier for the sender client.</p>
        pub fn sender_client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.sender_client_id(input.into());
            self
        }
        /// <p>The unique identifier for the sender client.</p>
        pub fn set_sender_client_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_sender_client_id(input);
            self
        }
        /// <p>The base64-encoded SDP offer content.</p>
        pub fn message_payload(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.message_payload(input.into());
            self
        }
        /// <p>The base64-encoded SDP offer content.</p>
        pub fn set_message_payload(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_message_payload(input);
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
