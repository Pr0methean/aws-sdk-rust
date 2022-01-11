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

/// Client for Amazon Personalize Runtime
///
/// Client for invoking operations on Amazon Personalize Runtime. Each operation on Amazon Personalize Runtime is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_personalizeruntime::Client::new(&shared_config);
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
///     let config = aws_sdk_personalizeruntime::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_personalizeruntime::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`GetPersonalizedRanking`](crate::client::fluent_builders::GetPersonalizedRanking) operation.
    ///
    /// - Takes [`GetPersonalizedRankingInput`](crate::input::GetPersonalizedRankingInput) with field(s):
    ///   - [`campaign_arn(Option<String>)`](crate::input::GetPersonalizedRankingInput::campaign_arn): <p>The Amazon Resource Name (ARN) of the campaign to use for generating the personalized ranking.</p>
    ///   - [`input_list(Option<Vec<String>>)`](crate::input::GetPersonalizedRankingInput::input_list): <p>A list of items (by <code>itemId</code>) to rank. If an item was not included in the training dataset, the item is appended to the end of the reranked list. The maximum is 500.</p>
    ///   - [`user_id(Option<String>)`](crate::input::GetPersonalizedRankingInput::user_id): <p>The user for which you want the campaign to provide a personalized ranking.</p>
    ///   - [`context(Option<HashMap<String, String>>)`](crate::input::GetPersonalizedRankingInput::context): <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
    ///   - [`filter_arn(Option<String>)`](crate::input::GetPersonalizedRankingInput::filter_arn): <p>The Amazon Resource Name (ARN) of a filter you created to include items or exclude items from recommendations for a given user. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
    ///   - [`filter_values(Option<HashMap<String, String>>)`](crate::input::GetPersonalizedRankingInput::filter_values): <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>  <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
    /// - On success, responds with [`GetPersonalizedRankingOutput`](crate::output::GetPersonalizedRankingOutput) with field(s):
    ///   - [`personalized_ranking(Option<Vec<PredictedItem>>)`](crate::output::GetPersonalizedRankingOutput::personalized_ranking): <p>A list of items in order of most likely interest to the user. The maximum is 500.</p>
    ///   - [`recommendation_id(Option<String>)`](crate::output::GetPersonalizedRankingOutput::recommendation_id): <p>The ID of the recommendation.</p>
    /// - On failure, responds with [`SdkError<GetPersonalizedRankingError>`](crate::error::GetPersonalizedRankingError)
    pub fn get_personalized_ranking(&self) -> fluent_builders::GetPersonalizedRanking<C, M, R> {
        fluent_builders::GetPersonalizedRanking::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`GetRecommendations`](crate::client::fluent_builders::GetRecommendations) operation.
    ///
    /// - Takes [`GetRecommendationsInput`](crate::input::GetRecommendationsInput) with field(s):
    ///   - [`campaign_arn(Option<String>)`](crate::input::GetRecommendationsInput::campaign_arn): <p>The Amazon Resource Name (ARN) of the campaign to use for getting recommendations.</p>
    ///   - [`item_id(Option<String>)`](crate::input::GetRecommendationsInput::item_id): <p>The item ID to provide recommendations for.</p>  <p>Required for <code>RELATED_ITEMS</code> recipe type.</p>
    ///   - [`user_id(Option<String>)`](crate::input::GetRecommendationsInput::user_id): <p>The user ID to provide recommendations for.</p>  <p>Required for <code>USER_PERSONALIZATION</code> recipe type.</p>
    ///   - [`num_results(i32)`](crate::input::GetRecommendationsInput::num_results): <p>The number of results to return. The default is 25. The maximum is 500.</p>
    ///   - [`context(Option<HashMap<String, String>>)`](crate::input::GetRecommendationsInput::context): <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
    ///   - [`filter_arn(Option<String>)`](crate::input::GetRecommendationsInput::filter_arn): <p>The ARN of the filter to apply to the returned recommendations. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>  <p>When using this parameter, be sure the filter resource is <code>ACTIVE</code>.</p>
    ///   - [`filter_values(Option<HashMap<String, String>>)`](crate::input::GetRecommendationsInput::filter_values): <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>  <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
    ///   - [`recommender_arn(Option<String>)`](crate::input::GetRecommendationsInput::recommender_arn): <p>The Amazon Resource Name (ARN) of the recommender to use to get recommendations. Provide a recommender ARN if you created a Domain dataset group with a recommender for a domain use case.</p>
    /// - On success, responds with [`GetRecommendationsOutput`](crate::output::GetRecommendationsOutput) with field(s):
    ///   - [`item_list(Option<Vec<PredictedItem>>)`](crate::output::GetRecommendationsOutput::item_list): <p>A list of recommendations sorted in descending order by prediction score. There can be a maximum of 500 items in the list.</p>
    ///   - [`recommendation_id(Option<String>)`](crate::output::GetRecommendationsOutput::recommendation_id): <p>The ID of the recommendation.</p>
    /// - On failure, responds with [`SdkError<GetRecommendationsError>`](crate::error::GetRecommendationsError)
    pub fn get_recommendations(&self) -> fluent_builders::GetRecommendations<C, M, R> {
        fluent_builders::GetRecommendations::new(self.handle.clone())
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
    /// Fluent builder constructing a request to `GetPersonalizedRanking`.
    ///
    /// <p>Re-ranks a list of recommended items for the given user. The first item in the list is deemed the most likely item to be of interest to the user.</p> <note>
    /// <p>The solution backing the campaign must have been created using a recipe of type PERSONALIZED_RANKING.</p>
    /// </note>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetPersonalizedRanking<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_personalized_ranking_input::Builder,
    }
    impl<C, M, R> GetPersonalizedRanking<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetPersonalizedRanking`.
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
            crate::output::GetPersonalizedRankingOutput,
            aws_smithy_http::result::SdkError<crate::error::GetPersonalizedRankingError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetPersonalizedRankingInputOperationOutputAlias,
                crate::output::GetPersonalizedRankingOutput,
                crate::error::GetPersonalizedRankingError,
                crate::input::GetPersonalizedRankingInputOperationRetryAlias,
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
        /// <p>The Amazon Resource Name (ARN) of the campaign to use for generating the personalized ranking.</p>
        pub fn campaign_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.campaign_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the campaign to use for generating the personalized ranking.</p>
        pub fn set_campaign_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_campaign_arn(input);
            self
        }
        /// Appends an item to `inputList`.
        ///
        /// To override the contents of this collection use [`set_input_list`](Self::set_input_list).
        ///
        /// <p>A list of items (by <code>itemId</code>) to rank. If an item was not included in the training dataset, the item is appended to the end of the reranked list. The maximum is 500.</p>
        pub fn input_list(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.input_list(input.into());
            self
        }
        /// <p>A list of items (by <code>itemId</code>) to rank. If an item was not included in the training dataset, the item is appended to the end of the reranked list. The maximum is 500.</p>
        pub fn set_input_list(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_input_list(input);
            self
        }
        /// <p>The user for which you want the campaign to provide a personalized ranking.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.user_id(input.into());
            self
        }
        /// <p>The user for which you want the campaign to provide a personalized ranking.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_user_id(input);
            self
        }
        /// Adds a key-value pair to `context`.
        ///
        /// To override the contents of this collection use [`set_context`](Self::set_context).
        ///
        /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
        pub fn context(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.context(k.into(), v.into());
            self
        }
        /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
        pub fn set_context(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_context(input);
            self
        }
        /// <p>The Amazon Resource Name (ARN) of a filter you created to include items or exclude items from recommendations for a given user. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        pub fn filter_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.filter_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of a filter you created to include items or exclude items from recommendations for a given user. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        pub fn set_filter_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_filter_arn(input);
            self
        }
        /// Adds a key-value pair to `filterValues`.
        ///
        /// To override the contents of this collection use [`set_filter_values`](Self::set_filter_values).
        ///
        /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>
        /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>
        /// <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        pub fn filter_values(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.filter_values(k.into(), v.into());
            self
        }
        /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>
        /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>
        /// <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        pub fn set_filter_values(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_filter_values(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetRecommendations`.
    ///
    /// <p>Returns a list of recommended items. For campaigns, the campaign's Amazon Resource Name (ARN) is required and the required user and item input depends on the recipe type used to create the solution backing the campaign as follows:</p>
    /// <ul>
    /// <li> <p>USER_PERSONALIZATION - <code>userId</code> required, <code>itemId</code> not used</p> </li>
    /// <li> <p>RELATED_ITEMS - <code>itemId</code> required, <code>userId</code> not used</p> </li>
    /// </ul> <note>
    /// <p>Campaigns that are backed by a solution created using a recipe of type PERSONALIZED_RANKING use the API.</p>
    /// </note>
    /// <p> For recommenders, the recommender's ARN is required and the required item and user input depends on the use case (domain-based recipe) backing the recommender. For information on use case requirements see <a href="https://docs.aws.amazon.com/personalize/latest/dg/domain-use-cases.html">Choosing recommender use cases</a>. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetRecommendations<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_recommendations_input::Builder,
    }
    impl<C, M, R> GetRecommendations<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetRecommendations`.
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
            crate::output::GetRecommendationsOutput,
            aws_smithy_http::result::SdkError<crate::error::GetRecommendationsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetRecommendationsInputOperationOutputAlias,
                crate::output::GetRecommendationsOutput,
                crate::error::GetRecommendationsError,
                crate::input::GetRecommendationsInputOperationRetryAlias,
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
        /// <p>The Amazon Resource Name (ARN) of the campaign to use for getting recommendations.</p>
        pub fn campaign_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.campaign_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the campaign to use for getting recommendations.</p>
        pub fn set_campaign_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_campaign_arn(input);
            self
        }
        /// <p>The item ID to provide recommendations for.</p>
        /// <p>Required for <code>RELATED_ITEMS</code> recipe type.</p>
        pub fn item_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.item_id(input.into());
            self
        }
        /// <p>The item ID to provide recommendations for.</p>
        /// <p>Required for <code>RELATED_ITEMS</code> recipe type.</p>
        pub fn set_item_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_item_id(input);
            self
        }
        /// <p>The user ID to provide recommendations for.</p>
        /// <p>Required for <code>USER_PERSONALIZATION</code> recipe type.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.user_id(input.into());
            self
        }
        /// <p>The user ID to provide recommendations for.</p>
        /// <p>Required for <code>USER_PERSONALIZATION</code> recipe type.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_user_id(input);
            self
        }
        /// <p>The number of results to return. The default is 25. The maximum is 500.</p>
        pub fn num_results(mut self, input: i32) -> Self {
            self.inner = self.inner.num_results(input);
            self
        }
        /// <p>The number of results to return. The default is 25. The maximum is 500.</p>
        pub fn set_num_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_num_results(input);
            self
        }
        /// Adds a key-value pair to `context`.
        ///
        /// To override the contents of this collection use [`set_context`](Self::set_context).
        ///
        /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
        pub fn context(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.context(k.into(), v.into());
            self
        }
        /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
        pub fn set_context(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_context(input);
            self
        }
        /// <p>The ARN of the filter to apply to the returned recommendations. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        /// <p>When using this parameter, be sure the filter resource is <code>ACTIVE</code>.</p>
        pub fn filter_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.filter_arn(input.into());
            self
        }
        /// <p>The ARN of the filter to apply to the returned recommendations. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        /// <p>When using this parameter, be sure the filter resource is <code>ACTIVE</code>.</p>
        pub fn set_filter_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_filter_arn(input);
            self
        }
        /// Adds a key-value pair to `filterValues`.
        ///
        /// To override the contents of this collection use [`set_filter_values`](Self::set_filter_values).
        ///
        /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>
        /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>
        /// <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        pub fn filter_values(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.filter_values(k.into(), v.into());
            self
        }
        /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>
        /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>
        /// <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        pub fn set_filter_values(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_filter_values(input);
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the recommender to use to get recommendations. Provide a recommender ARN if you created a Domain dataset group with a recommender for a domain use case.</p>
        pub fn recommender_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.recommender_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the recommender to use to get recommendations. Provide a recommender ARN if you created a Domain dataset group with a recommender for a domain use case.</p>
        pub fn set_recommender_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_recommender_arn(input);
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
