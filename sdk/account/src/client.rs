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

/// Client for AWS Account
///
/// Client for invoking operations on AWS Account. Each operation on AWS Account is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_account::Client::new(&shared_config);
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
///     let config = aws_sdk_account::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_account::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`DeleteAlternateContact`](crate::client::fluent_builders::DeleteAlternateContact) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`alternate_contact_type(AlternateContactType)`](crate::client::fluent_builders::DeleteAlternateContact::alternate_contact_type) / [`set_alternate_contact_type(Option<AlternateContactType>)`](crate::client::fluent_builders::DeleteAlternateContact::set_alternate_contact_type): <p>Specifies which of the alternate contacts to delete. </p>
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::DeleteAlternateContact::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::DeleteAlternateContact::set_account_id): <p>Specifies the 12 digit account ID number of the Amazon Web Services account that you want to access or modify with this operation.</p>  <p>If you do not specify this parameter, it defaults to the Amazon Web Services account of the identity used to call the operation.</p>  <p>To use this parameter, the caller must be an identity in the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">organization's management account</a> or a delegated administrator account, and the specified account ID must be a member account in the same organization. The organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features enabled</a>, and the organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-trusted-access.html">trusted access</a> enabled for the Account Management service, and optionally a <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-delegated-admin.html">delegated admin</a> account assigned.</p> <note>   <p>The management account can't specify its own <code>AccountId</code>; it must call the operation in standalone context by not including the <code>AccountId</code> parameter.</p>  </note>  <p>To call this operation on an account that is not a member of an organization, then don't specify this parameter, and call the operation using an identity belonging to the account whose contacts you wish to retrieve or modify.</p>
    /// - On success, responds with [`DeleteAlternateContactOutput`](crate::output::DeleteAlternateContactOutput)

    /// - On failure, responds with [`SdkError<DeleteAlternateContactError>`](crate::error::DeleteAlternateContactError)
    pub fn delete_alternate_contact(&self) -> fluent_builders::DeleteAlternateContact<C, M, R> {
        fluent_builders::DeleteAlternateContact::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`GetAlternateContact`](crate::client::fluent_builders::GetAlternateContact) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`alternate_contact_type(AlternateContactType)`](crate::client::fluent_builders::GetAlternateContact::alternate_contact_type) / [`set_alternate_contact_type(Option<AlternateContactType>)`](crate::client::fluent_builders::GetAlternateContact::set_alternate_contact_type): <p>Specifies which alternate contact you want to retrieve.</p>
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::GetAlternateContact::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::GetAlternateContact::set_account_id): <p>Specifies the 12 digit account ID number of the Amazon Web Services account that you want to access or modify with this operation.</p>  <p>If you do not specify this parameter, it defaults to the Amazon Web Services account of the identity used to call the operation.</p>  <p>To use this parameter, the caller must be an identity in the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">organization's management account</a> or a delegated administrator account, and the specified account ID must be a member account in the same organization. The organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features enabled</a>, and the organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-trusted-access.html">trusted access</a> enabled for the Account Management service, and optionally a <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-delegated-admin.html">delegated admin</a> account assigned.</p> <note>   <p>The management account can't specify its own <code>AccountId</code>; it must call the operation in standalone context by not including the <code>AccountId</code> parameter.</p>  </note>  <p>To call this operation on an account that is not a member of an organization, then don't specify this parameter, and call the operation using an identity belonging to the account whose contacts you wish to retrieve or modify.</p>
    /// - On success, responds with [`GetAlternateContactOutput`](crate::output::GetAlternateContactOutput) with field(s):
    ///   - [`alternate_contact(Option<AlternateContact>)`](crate::output::GetAlternateContactOutput::alternate_contact): <p>A structure that contains the details for the specified alternate contact.</p>
    /// - On failure, responds with [`SdkError<GetAlternateContactError>`](crate::error::GetAlternateContactError)
    pub fn get_alternate_contact(&self) -> fluent_builders::GetAlternateContact<C, M, R> {
        fluent_builders::GetAlternateContact::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`PutAlternateContact`](crate::client::fluent_builders::PutAlternateContact) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::PutAlternateContact::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::PutAlternateContact::set_name): <p>Specifies a name for the alternate contact.</p>
    ///   - [`title(impl Into<String>)`](crate::client::fluent_builders::PutAlternateContact::title) / [`set_title(Option<String>)`](crate::client::fluent_builders::PutAlternateContact::set_title): <p>Specifies a title for the alternate contact.</p>
    ///   - [`email_address(impl Into<String>)`](crate::client::fluent_builders::PutAlternateContact::email_address) / [`set_email_address(Option<String>)`](crate::client::fluent_builders::PutAlternateContact::set_email_address): <p>Specifies an email address for the alternate contact. </p>
    ///   - [`phone_number(impl Into<String>)`](crate::client::fluent_builders::PutAlternateContact::phone_number) / [`set_phone_number(Option<String>)`](crate::client::fluent_builders::PutAlternateContact::set_phone_number): <p>Specifies a phone number for the alternate contact.</p>
    ///   - [`alternate_contact_type(AlternateContactType)`](crate::client::fluent_builders::PutAlternateContact::alternate_contact_type) / [`set_alternate_contact_type(Option<AlternateContactType>)`](crate::client::fluent_builders::PutAlternateContact::set_alternate_contact_type): <p>Specifies which alternate contact you want to create or update.</p>
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::PutAlternateContact::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::PutAlternateContact::set_account_id): <p>Specifies the 12 digit account ID number of the Amazon Web Services account that you want to access or modify with this operation.</p>  <p>If you do not specify this parameter, it defaults to the Amazon Web Services account of the identity used to call the operation.</p>  <p>To use this parameter, the caller must be an identity in the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">organization's management account</a> or a delegated administrator account, and the specified account ID must be a member account in the same organization. The organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features enabled</a>, and the organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-trusted-access.html">trusted access</a> enabled for the Account Management service, and optionally a <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-delegated-admin.html">delegated admin</a> account assigned.</p> <note>   <p>The management account can't specify its own <code>AccountId</code>; it must call the operation in standalone context by not including the <code>AccountId</code> parameter.</p>  </note>  <p>To call this operation on an account that is not a member of an organization, then don't specify this parameter, and call the operation using an identity belonging to the account whose contacts you wish to retrieve or modify.</p>
    /// - On success, responds with [`PutAlternateContactOutput`](crate::output::PutAlternateContactOutput)

    /// - On failure, responds with [`SdkError<PutAlternateContactError>`](crate::error::PutAlternateContactError)
    pub fn put_alternate_contact(&self) -> fluent_builders::PutAlternateContact<C, M, R> {
        fluent_builders::PutAlternateContact::new(self.handle.clone())
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
    /// Fluent builder constructing a request to `DeleteAlternateContact`.
    ///
    /// <p>Deletes the specified alternate contact from an Amazon Web Services account.</p>
    /// <p>For complete details about how to use the alternate contact operations, see <a href="https://docs.aws.amazon.com/accounts/latest/reference/manage-acct-update-contact.html">Access or updating the alternate contacts</a>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DeleteAlternateContact<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_alternate_contact_input::Builder,
    }
    impl<C, M, R> DeleteAlternateContact<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DeleteAlternateContact`.
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
            crate::output::DeleteAlternateContactOutput,
            aws_smithy_http::result::SdkError<crate::error::DeleteAlternateContactError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteAlternateContactInputOperationOutputAlias,
                crate::output::DeleteAlternateContactOutput,
                crate::error::DeleteAlternateContactError,
                crate::input::DeleteAlternateContactInputOperationRetryAlias,
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
        /// <p>Specifies which of the alternate contacts to delete. </p>
        pub fn alternate_contact_type(mut self, input: crate::model::AlternateContactType) -> Self {
            self.inner = self.inner.alternate_contact_type(input);
            self
        }
        /// <p>Specifies which of the alternate contacts to delete. </p>
        pub fn set_alternate_contact_type(
            mut self,
            input: std::option::Option<crate::model::AlternateContactType>,
        ) -> Self {
            self.inner = self.inner.set_alternate_contact_type(input);
            self
        }
        /// <p>Specifies the 12 digit account ID number of the Amazon Web Services account that you want to access or modify with this operation.</p>
        /// <p>If you do not specify this parameter, it defaults to the Amazon Web Services account of the identity used to call the operation.</p>
        /// <p>To use this parameter, the caller must be an identity in the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">organization's management account</a> or a delegated administrator account, and the specified account ID must be a member account in the same organization. The organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features enabled</a>, and the organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-trusted-access.html">trusted access</a> enabled for the Account Management service, and optionally a <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-delegated-admin.html">delegated admin</a> account assigned.</p> <note>
        /// <p>The management account can't specify its own <code>AccountId</code>; it must call the operation in standalone context by not including the <code>AccountId</code> parameter.</p>
        /// </note>
        /// <p>To call this operation on an account that is not a member of an organization, then don't specify this parameter, and call the operation using an identity belonging to the account whose contacts you wish to retrieve or modify.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.account_id(input.into());
            self
        }
        /// <p>Specifies the 12 digit account ID number of the Amazon Web Services account that you want to access or modify with this operation.</p>
        /// <p>If you do not specify this parameter, it defaults to the Amazon Web Services account of the identity used to call the operation.</p>
        /// <p>To use this parameter, the caller must be an identity in the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">organization's management account</a> or a delegated administrator account, and the specified account ID must be a member account in the same organization. The organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features enabled</a>, and the organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-trusted-access.html">trusted access</a> enabled for the Account Management service, and optionally a <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-delegated-admin.html">delegated admin</a> account assigned.</p> <note>
        /// <p>The management account can't specify its own <code>AccountId</code>; it must call the operation in standalone context by not including the <code>AccountId</code> parameter.</p>
        /// </note>
        /// <p>To call this operation on an account that is not a member of an organization, then don't specify this parameter, and call the operation using an identity belonging to the account whose contacts you wish to retrieve or modify.</p>
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_account_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetAlternateContact`.
    ///
    /// <p>Retrieves the specified alternate contact attached to an Amazon Web Services account.</p>
    /// <p>For complete details about how to use the alternate contact operations, see <a href="https://docs.aws.amazon.com/accounts/latest/reference/manage-acct-update-contact.html">Access or updating the alternate contacts</a>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetAlternateContact<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_alternate_contact_input::Builder,
    }
    impl<C, M, R> GetAlternateContact<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetAlternateContact`.
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
            crate::output::GetAlternateContactOutput,
            aws_smithy_http::result::SdkError<crate::error::GetAlternateContactError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetAlternateContactInputOperationOutputAlias,
                crate::output::GetAlternateContactOutput,
                crate::error::GetAlternateContactError,
                crate::input::GetAlternateContactInputOperationRetryAlias,
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
        /// <p>Specifies which alternate contact you want to retrieve.</p>
        pub fn alternate_contact_type(mut self, input: crate::model::AlternateContactType) -> Self {
            self.inner = self.inner.alternate_contact_type(input);
            self
        }
        /// <p>Specifies which alternate contact you want to retrieve.</p>
        pub fn set_alternate_contact_type(
            mut self,
            input: std::option::Option<crate::model::AlternateContactType>,
        ) -> Self {
            self.inner = self.inner.set_alternate_contact_type(input);
            self
        }
        /// <p>Specifies the 12 digit account ID number of the Amazon Web Services account that you want to access or modify with this operation.</p>
        /// <p>If you do not specify this parameter, it defaults to the Amazon Web Services account of the identity used to call the operation.</p>
        /// <p>To use this parameter, the caller must be an identity in the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">organization's management account</a> or a delegated administrator account, and the specified account ID must be a member account in the same organization. The organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features enabled</a>, and the organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-trusted-access.html">trusted access</a> enabled for the Account Management service, and optionally a <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-delegated-admin.html">delegated admin</a> account assigned.</p> <note>
        /// <p>The management account can't specify its own <code>AccountId</code>; it must call the operation in standalone context by not including the <code>AccountId</code> parameter.</p>
        /// </note>
        /// <p>To call this operation on an account that is not a member of an organization, then don't specify this parameter, and call the operation using an identity belonging to the account whose contacts you wish to retrieve or modify.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.account_id(input.into());
            self
        }
        /// <p>Specifies the 12 digit account ID number of the Amazon Web Services account that you want to access or modify with this operation.</p>
        /// <p>If you do not specify this parameter, it defaults to the Amazon Web Services account of the identity used to call the operation.</p>
        /// <p>To use this parameter, the caller must be an identity in the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">organization's management account</a> or a delegated administrator account, and the specified account ID must be a member account in the same organization. The organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features enabled</a>, and the organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-trusted-access.html">trusted access</a> enabled for the Account Management service, and optionally a <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-delegated-admin.html">delegated admin</a> account assigned.</p> <note>
        /// <p>The management account can't specify its own <code>AccountId</code>; it must call the operation in standalone context by not including the <code>AccountId</code> parameter.</p>
        /// </note>
        /// <p>To call this operation on an account that is not a member of an organization, then don't specify this parameter, and call the operation using an identity belonging to the account whose contacts you wish to retrieve or modify.</p>
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_account_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `PutAlternateContact`.
    ///
    /// <p>Modifies the specified alternate contact attached to an Amazon Web Services account.</p>
    /// <p>For complete details about how to use the alternate contact operations, see <a href="https://docs.aws.amazon.com/accounts/latest/reference/manage-acct-update-contact.html">Access or updating the alternate contacts</a>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct PutAlternateContact<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::put_alternate_contact_input::Builder,
    }
    impl<C, M, R> PutAlternateContact<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `PutAlternateContact`.
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
            crate::output::PutAlternateContactOutput,
            aws_smithy_http::result::SdkError<crate::error::PutAlternateContactError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::PutAlternateContactInputOperationOutputAlias,
                crate::output::PutAlternateContactOutput,
                crate::error::PutAlternateContactError,
                crate::input::PutAlternateContactInputOperationRetryAlias,
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
        /// <p>Specifies a name for the alternate contact.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(input.into());
            self
        }
        /// <p>Specifies a name for the alternate contact.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_name(input);
            self
        }
        /// <p>Specifies a title for the alternate contact.</p>
        pub fn title(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.title(input.into());
            self
        }
        /// <p>Specifies a title for the alternate contact.</p>
        pub fn set_title(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_title(input);
            self
        }
        /// <p>Specifies an email address for the alternate contact. </p>
        pub fn email_address(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.email_address(input.into());
            self
        }
        /// <p>Specifies an email address for the alternate contact. </p>
        pub fn set_email_address(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_email_address(input);
            self
        }
        /// <p>Specifies a phone number for the alternate contact.</p>
        pub fn phone_number(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.phone_number(input.into());
            self
        }
        /// <p>Specifies a phone number for the alternate contact.</p>
        pub fn set_phone_number(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_phone_number(input);
            self
        }
        /// <p>Specifies which alternate contact you want to create or update.</p>
        pub fn alternate_contact_type(mut self, input: crate::model::AlternateContactType) -> Self {
            self.inner = self.inner.alternate_contact_type(input);
            self
        }
        /// <p>Specifies which alternate contact you want to create or update.</p>
        pub fn set_alternate_contact_type(
            mut self,
            input: std::option::Option<crate::model::AlternateContactType>,
        ) -> Self {
            self.inner = self.inner.set_alternate_contact_type(input);
            self
        }
        /// <p>Specifies the 12 digit account ID number of the Amazon Web Services account that you want to access or modify with this operation.</p>
        /// <p>If you do not specify this parameter, it defaults to the Amazon Web Services account of the identity used to call the operation.</p>
        /// <p>To use this parameter, the caller must be an identity in the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">organization's management account</a> or a delegated administrator account, and the specified account ID must be a member account in the same organization. The organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features enabled</a>, and the organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-trusted-access.html">trusted access</a> enabled for the Account Management service, and optionally a <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-delegated-admin.html">delegated admin</a> account assigned.</p> <note>
        /// <p>The management account can't specify its own <code>AccountId</code>; it must call the operation in standalone context by not including the <code>AccountId</code> parameter.</p>
        /// </note>
        /// <p>To call this operation on an account that is not a member of an organization, then don't specify this parameter, and call the operation using an identity belonging to the account whose contacts you wish to retrieve or modify.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.account_id(input.into());
            self
        }
        /// <p>Specifies the 12 digit account ID number of the Amazon Web Services account that you want to access or modify with this operation.</p>
        /// <p>If you do not specify this parameter, it defaults to the Amazon Web Services account of the identity used to call the operation.</p>
        /// <p>To use this parameter, the caller must be an identity in the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">organization's management account</a> or a delegated administrator account, and the specified account ID must be a member account in the same organization. The organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features enabled</a>, and the organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-trusted-access.html">trusted access</a> enabled for the Account Management service, and optionally a <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-delegated-admin.html">delegated admin</a> account assigned.</p> <note>
        /// <p>The management account can't specify its own <code>AccountId</code>; it must call the operation in standalone context by not including the <code>AccountId</code> parameter.</p>
        /// </note>
        /// <p>To call this operation on an account that is not a member of an organization, then don't specify this parameter, and call the operation using an identity belonging to the account whose contacts you wish to retrieve or modify.</p>
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_account_id(input);
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
