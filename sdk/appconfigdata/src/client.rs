// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for AWS AppConfig Data
///
/// Client for invoking operations on AWS AppConfig Data. Each operation on AWS AppConfig Data is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_appconfigdata::Client::new(&shared_config);
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
/// let config = aws_sdk_appconfigdata::config::Builder::from(&shared_config)
///   .retry_config(RetryConfig::disabled())
///   .build();
/// let client = aws_sdk_appconfigdata::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`GetLatestConfiguration`](crate::client::fluent_builders::GetLatestConfiguration) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_token(impl Into<String>)`](crate::client::fluent_builders::GetLatestConfiguration::configuration_token) / [`set_configuration_token(Option<String>)`](crate::client::fluent_builders::GetLatestConfiguration::set_configuration_token): <p>Token describing the current state of the configuration session. To obtain a token, first call the <code>StartConfigurationSession</code> API. Note that every call to <code>GetLatestConfiguration</code> will return a new <code>ConfigurationToken</code> (<code>NextPollConfigurationToken</code> in the response) and MUST be provided to subsequent <code>GetLatestConfiguration</code> API calls.</p>
    /// - On success, responds with [`GetLatestConfigurationOutput`](crate::output::GetLatestConfigurationOutput) with field(s):
    ///   - [`next_poll_configuration_token(Option<String>)`](crate::output::GetLatestConfigurationOutput::next_poll_configuration_token): <p>The latest token describing the current state of the configuration session. This MUST be provided to the next call to <code>GetLatestConfiguration.</code> </p>
    ///   - [`next_poll_interval_in_seconds(i32)`](crate::output::GetLatestConfigurationOutput::next_poll_interval_in_seconds): <p>The amount of time the client should wait before polling for configuration updates again. Use <code>RequiredMinimumPollIntervalInSeconds</code> to set the desired poll interval.</p>
    ///   - [`content_type(Option<String>)`](crate::output::GetLatestConfigurationOutput::content_type): <p>A standard MIME type describing the format of the configuration content.</p>
    ///   - [`configuration(Option<Blob>)`](crate::output::GetLatestConfigurationOutput::configuration): <p>The data of the configuration. This may be empty if the client already has the latest version of configuration.</p>
    /// - On failure, responds with [`SdkError<GetLatestConfigurationError>`](crate::error::GetLatestConfigurationError)
    pub fn get_latest_configuration(&self) -> fluent_builders::GetLatestConfiguration {
        fluent_builders::GetLatestConfiguration::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`StartConfigurationSession`](crate::client::fluent_builders::StartConfigurationSession) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_identifier(impl Into<String>)`](crate::client::fluent_builders::StartConfigurationSession::application_identifier) / [`set_application_identifier(Option<String>)`](crate::client::fluent_builders::StartConfigurationSession::set_application_identifier): <p>The application ID or the application name.</p>
    ///   - [`environment_identifier(impl Into<String>)`](crate::client::fluent_builders::StartConfigurationSession::environment_identifier) / [`set_environment_identifier(Option<String>)`](crate::client::fluent_builders::StartConfigurationSession::set_environment_identifier): <p>The environment ID or the environment name.</p>
    ///   - [`configuration_profile_identifier(impl Into<String>)`](crate::client::fluent_builders::StartConfigurationSession::configuration_profile_identifier) / [`set_configuration_profile_identifier(Option<String>)`](crate::client::fluent_builders::StartConfigurationSession::set_configuration_profile_identifier): <p>The configuration profile ID or the configuration profile name.</p>
    ///   - [`required_minimum_poll_interval_in_seconds(i32)`](crate::client::fluent_builders::StartConfigurationSession::required_minimum_poll_interval_in_seconds) / [`set_required_minimum_poll_interval_in_seconds(Option<i32>)`](crate::client::fluent_builders::StartConfigurationSession::set_required_minimum_poll_interval_in_seconds): <p>Sets a constraint on a session. If you specify a value of, for example, 60 seconds, then the client that established the session can't call <code>GetLatestConfiguration</code> more frequently then every 60 seconds.</p>
    /// - On success, responds with [`StartConfigurationSessionOutput`](crate::output::StartConfigurationSessionOutput) with field(s):
    ///   - [`initial_configuration_token(Option<String>)`](crate::output::StartConfigurationSessionOutput::initial_configuration_token): <p>Token encapsulating state about the configuration session. Provide this token to the <code>GetLatestConfiguration</code> API to retrieve configuration data.</p> <important>   <p>This token should only be used once in your first call to <code>GetLatestConfiguration</code>. You MUST use the new token in the <code>GetLatestConfiguration</code> response (<code>NextPollConfigurationToken</code>) in each subsequent call to <code>GetLatestConfiguration</code>.</p>  </important>
    /// - On failure, responds with [`SdkError<StartConfigurationSessionError>`](crate::error::StartConfigurationSessionError)
    pub fn start_configuration_session(&self) -> fluent_builders::StartConfigurationSession {
        fluent_builders::StartConfigurationSession::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `GetLatestConfiguration`.
    ///
    /// <p>Retrieves the latest deployed configuration. This API may return empty configuration data if the client already has the latest version. For more information about this API action and to view example CLI commands that show how to use it with the <code>StartConfigurationSession</code> API action, see <a href="http://docs.aws.amazon.com/appconfig/latest/userguide/appconfig-retrieving-the-configuration">Receiving the configuration</a> in the <i>AppConfig User Guide</i>. </p> <important>
    /// <p>Note the following important information.</p>
    /// <ul>
    /// <li> <p>Each configuration token is only valid for one call to <code>GetLatestConfiguration</code>. The <code>GetLatestConfiguration</code> response includes a <code>NextPollConfigurationToken</code> that should always replace the token used for the just-completed call in preparation for the next one. </p> </li>
    /// <li> <p> <code>GetLatestConfiguration</code> is a priced call. For more information, see <a href="https://aws.amazon.com/systems-manager/pricing/">Pricing</a>.</p> </li>
    /// </ul>
    /// </important>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetLatestConfiguration {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::get_latest_configuration_input::Builder,
    }
    impl GetLatestConfiguration {
        /// Creates a new `GetLatestConfiguration`.
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
            crate::output::GetLatestConfigurationOutput,
            aws_smithy_http::result::SdkError<crate::error::GetLatestConfigurationError>,
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
        /// <p>Token describing the current state of the configuration session. To obtain a token, first call the <code>StartConfigurationSession</code> API. Note that every call to <code>GetLatestConfiguration</code> will return a new <code>ConfigurationToken</code> (<code>NextPollConfigurationToken</code> in the response) and MUST be provided to subsequent <code>GetLatestConfiguration</code> API calls.</p>
        pub fn configuration_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_token(input.into());
            self
        }
        /// <p>Token describing the current state of the configuration session. To obtain a token, first call the <code>StartConfigurationSession</code> API. Note that every call to <code>GetLatestConfiguration</code> will return a new <code>ConfigurationToken</code> (<code>NextPollConfigurationToken</code> in the response) and MUST be provided to subsequent <code>GetLatestConfiguration</code> API calls.</p>
        pub fn set_configuration_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_token(input);
            self
        }
    }
    /// Fluent builder constructing a request to `StartConfigurationSession`.
    ///
    /// <p>Starts a configuration session used to retrieve a deployed configuration. For more information about this API action and to view example CLI commands that show how to use it with the <code>GetLatestConfiguration</code> API action, see <a href="http://docs.aws.amazon.com/appconfig/latest/userguide/appconfig-retrieving-the-configuration">Receiving the configuration</a> in the <i>AppConfig User Guide</i>. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct StartConfigurationSession {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::start_configuration_session_input::Builder,
    }
    impl StartConfigurationSession {
        /// Creates a new `StartConfigurationSession`.
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
            crate::output::StartConfigurationSessionOutput,
            aws_smithy_http::result::SdkError<crate::error::StartConfigurationSessionError>,
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
        /// <p>The application ID or the application name.</p>
        pub fn application_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.application_identifier(input.into());
            self
        }
        /// <p>The application ID or the application name.</p>
        pub fn set_application_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_application_identifier(input);
            self
        }
        /// <p>The environment ID or the environment name.</p>
        pub fn environment_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.environment_identifier(input.into());
            self
        }
        /// <p>The environment ID or the environment name.</p>
        pub fn set_environment_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_environment_identifier(input);
            self
        }
        /// <p>The configuration profile ID or the configuration profile name.</p>
        pub fn configuration_profile_identifier(
            mut self,
            input: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.configuration_profile_identifier(input.into());
            self
        }
        /// <p>The configuration profile ID or the configuration profile name.</p>
        pub fn set_configuration_profile_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_profile_identifier(input);
            self
        }
        /// <p>Sets a constraint on a session. If you specify a value of, for example, 60 seconds, then the client that established the session can't call <code>GetLatestConfiguration</code> more frequently then every 60 seconds.</p>
        pub fn required_minimum_poll_interval_in_seconds(mut self, input: i32) -> Self {
            self.inner = self.inner.required_minimum_poll_interval_in_seconds(input);
            self
        }
        /// <p>Sets a constraint on a session. If you specify a value of, for example, 60 seconds, then the client that established the session can't call <code>GetLatestConfiguration</code> more frequently then every 60 seconds.</p>
        pub fn set_required_minimum_poll_interval_in_seconds(
            mut self,
            input: std::option::Option<i32>,
        ) -> Self {
            self.inner = self
                .inner
                .set_required_minimum_poll_interval_in_seconds(input);
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
