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

/// Client for Amazon Kinesis Video Streams Media
///
/// Client for invoking operations on Amazon Kinesis Video Streams Media. Each operation on Amazon Kinesis Video Streams Media is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_kinesisvideomedia::Client::new(&shared_config);
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
///     let config = aws_sdk_kinesisvideomedia::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_kinesisvideomedia::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`GetMedia`](crate::client::fluent_builders::GetMedia) operation.
    ///
    /// - Takes [`GetMediaInput`](crate::input::GetMediaInput) with field(s):
    ///   - [`stream_name(Option<String>)`](crate::input::GetMediaInput::stream_name): <p>The Kinesis video stream name from where you want to get the media content. If you don't specify the <code>streamName</code>, you must specify the <code>streamARN</code>.</p>
    ///   - [`stream_arn(Option<String>)`](crate::input::GetMediaInput::stream_arn): <p>The ARN of the stream from where you want to get the media content. If you don't specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
    ///   - [`start_selector(Option<StartSelector>)`](crate::input::GetMediaInput::start_selector): <p>Identifies the starting chunk to get from the specified stream. </p>
    /// - On success, responds with [`GetMediaOutput`](crate::output::GetMediaOutput) with field(s):
    ///   - [`content_type(Option<String>)`](crate::output::GetMediaOutput::content_type): <p>The content type of the requested media.</p>
    ///   - [`payload(byte_stream::ByteStream)`](crate::output::GetMediaOutput::payload): <p> The payload Kinesis Video Streams returns is a sequence of chunks from the specified stream. For information about the chunks, see . The chunks that Kinesis Video Streams returns in the <code>GetMedia</code> call also include the following additional Matroska (MKV) tags: </p>  <ul>   <li> <p>AWS_KINESISVIDEO_CONTINUATION_TOKEN (UTF-8 string) - In the event your <code>GetMedia</code> call terminates, you can use this continuation token in your next request to get the next chunk where the last request terminated.</p> </li>   <li> <p>AWS_KINESISVIDEO_MILLIS_BEHIND_NOW (UTF-8 string) - Client applications can use this tag value to determine how far behind the chunk returned in the response is from the latest chunk on the stream. </p> </li>   <li> <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - Fragment number returned in the chunk.</p> </li>   <li> <p>AWS_KINESISVIDEO_SERVER_TIMESTAMP - Server timestamp of the fragment.</p> </li>   <li> <p>AWS_KINESISVIDEO_PRODUCER_TIMESTAMP - Producer timestamp of the fragment.</p> </li>  </ul>  <p>The following tags will be present if an error occurs:</p>  <ul>   <li> <p>AWS_KINESISVIDEO_ERROR_CODE - String description of an error that caused GetMedia to stop.</p> </li>   <li> <p>AWS_KINESISVIDEO_ERROR_ID: Integer code of the error.</p> </li>  </ul>  <p>The error codes are as follows:</p>  <ul>   <li> <p>3002 - Error writing to the stream</p> </li>   <li> <p>4000 - Requested fragment is not found</p> </li>   <li> <p>4500 - Access denied for the stream's KMS key</p> </li>   <li> <p>4501 - Stream's KMS key is disabled</p> </li>   <li> <p>4502 - Validation error on the stream's KMS key</p> </li>   <li> <p>4503 - KMS key specified in the stream is unavailable</p> </li>   <li> <p>4504 - Invalid usage of the KMS key specified in the stream</p> </li>   <li> <p>4505 - Invalid state of the KMS key specified in the stream</p> </li>   <li> <p>4506 - Unable to find the KMS key specified in the stream</p> </li>   <li> <p>5000 - Internal error</p> </li>  </ul>
    /// - On failure, responds with [`SdkError<GetMediaError>`](crate::error::GetMediaError)
    pub fn get_media(&self) -> fluent_builders::GetMedia<C, M, R> {
        fluent_builders::GetMedia::new(self.handle.clone())
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
    /// Fluent builder constructing a request to `GetMedia`.
    ///
    /// <p> Use this API to retrieve media content from a Kinesis video stream. In the request, you identify the stream name or stream Amazon Resource Name (ARN), and the starting chunk. Kinesis Video Streams then returns a stream of chunks in order by fragment number.</p> <note>
    /// <p>You must first call the <code>GetDataEndpoint</code> API to get an endpoint. Then send the <code>GetMedia</code> requests to this endpoint using the <a href="https://docs.aws.amazon.com/cli/latest/reference/">--endpoint-url parameter</a>. </p>
    /// </note>
    /// <p>When you put media data (fragments) on a stream, Kinesis Video Streams stores each incoming fragment and related metadata in what is called a "chunk." For more information, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_dataplane_PutMedia.html">PutMedia</a>. The <code>GetMedia</code> API returns a stream of these chunks starting from the chunk that you specify in the request. </p>
    /// <p>The following limits apply when using the <code>GetMedia</code> API:</p>
    /// <ul>
    /// <li> <p>A client can call <code>GetMedia</code> up to five times per second per stream. </p> </li>
    /// <li> <p>Kinesis Video Streams sends media data at a rate of up to 25 megabytes per second (or 200 megabits per second) during a <code>GetMedia</code> session. </p> </li>
    /// </ul> <note>
    /// <p>If an error is thrown after invoking a Kinesis Video Streams media API, in addition to the HTTP status code and the response body, it includes the following pieces of information: </p>
    /// <ul>
    /// <li> <p> <code>x-amz-ErrorType</code> HTTP header – contains a more specific error type in addition to what the HTTP status code provides. </p> </li>
    /// <li> <p> <code>x-amz-RequestId</code> HTTP header – if you want to report an issue to AWS, the support team can better diagnose the problem if given the Request Id.</p> </li>
    /// </ul>
    /// <p>Both the HTTP status code and the ErrorType header can be utilized to make programmatic decisions about whether errors are retry-able and under what conditions, as well as provide information on what actions the client programmer might need to take in order to successfully try again.</p>
    /// <p>For more information, see the <b>Errors</b> section at the bottom of this topic, as well as <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/CommonErrors.html">Common Errors</a>. </p>
    /// </note>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetMedia<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_media_input::Builder,
    }
    impl<C, M, R> GetMedia<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetMedia`.
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
            crate::output::GetMediaOutput,
            aws_smithy_http::result::SdkError<crate::error::GetMediaError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetMediaInputOperationOutputAlias,
                crate::output::GetMediaOutput,
                crate::error::GetMediaError,
                crate::input::GetMediaInputOperationRetryAlias,
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
        /// <p>The Kinesis video stream name from where you want to get the media content. If you don't specify the <code>streamName</code>, you must specify the <code>streamARN</code>.</p>
        pub fn stream_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.stream_name(input.into());
            self
        }
        /// <p>The Kinesis video stream name from where you want to get the media content. If you don't specify the <code>streamName</code>, you must specify the <code>streamARN</code>.</p>
        pub fn set_stream_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_stream_name(input);
            self
        }
        /// <p>The ARN of the stream from where you want to get the media content. If you don't specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
        pub fn stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.stream_arn(input.into());
            self
        }
        /// <p>The ARN of the stream from where you want to get the media content. If you don't specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
        pub fn set_stream_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_stream_arn(input);
            self
        }
        /// <p>Identifies the starting chunk to get from the specified stream. </p>
        pub fn start_selector(mut self, input: crate::model::StartSelector) -> Self {
            self.inner = self.inner.start_selector(input);
            self
        }
        /// <p>Identifies the starting chunk to get from the specified stream. </p>
        pub fn set_start_selector(
            mut self,
            input: std::option::Option<crate::model::StartSelector>,
        ) -> Self {
            self.inner = self.inner.set_start_selector(input);
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
