// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `StartMedicalStreamTranscription` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct StartMedicalStreamTranscriptionError {
    /// Kind of error that occurred.
    pub kind: StartMedicalStreamTranscriptionErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `StartMedicalStreamTranscription` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum StartMedicalStreamTranscriptionErrorKind {
    /// <p>Service is currently unavailable. Try your request later.</p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p>One or more arguments to the <code>StartStreamTranscription</code> or <code>StartMedicalStreamTranscription</code> operation was invalid. For example, <code>MediaEncoding</code> was not set to a valid encoding, or <code>LanguageCode</code> was not set to a valid code. Check the parameters and try your request again.</p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>A problem occurred while processing the audio. Amazon Transcribe or Amazon Transcribe Medical terminated processing. Try your request again.</p>
    InternalFailureException(crate::error::InternalFailureException),
    /// <p>A new stream started with the same session ID. The current stream has been terminated.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>You have exceeded the maximum number of concurrent transcription streams, are starting transcription streams too quickly, or the maximum audio length of 4 hours. Wait until a stream has finished processing, or break your audio stream into smaller chunks and try your request again.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for StartMedicalStreamTranscriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StartMedicalStreamTranscriptionErrorKind::ServiceUnavailableException(_inner) => {
                _inner.fmt(f)
            }
            StartMedicalStreamTranscriptionErrorKind::BadRequestException(_inner) => _inner.fmt(f),
            StartMedicalStreamTranscriptionErrorKind::InternalFailureException(_inner) => {
                _inner.fmt(f)
            }
            StartMedicalStreamTranscriptionErrorKind::ConflictException(_inner) => _inner.fmt(f),
            StartMedicalStreamTranscriptionErrorKind::LimitExceededException(_inner) => {
                _inner.fmt(f)
            }
            StartMedicalStreamTranscriptionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for StartMedicalStreamTranscriptionError {
    fn code(&self) -> Option<&str> {
        StartMedicalStreamTranscriptionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl StartMedicalStreamTranscriptionError {
    /// Creates a new `StartMedicalStreamTranscriptionError`.
    pub fn new(
        kind: StartMedicalStreamTranscriptionErrorKind,
        meta: aws_smithy_types::Error,
    ) -> Self {
        Self { kind, meta }
    }

    /// Creates the `StartMedicalStreamTranscriptionError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: StartMedicalStreamTranscriptionErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `StartMedicalStreamTranscriptionError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: StartMedicalStreamTranscriptionErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `StartMedicalStreamTranscriptionErrorKind::ServiceUnavailableException`.
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartMedicalStreamTranscriptionErrorKind::ServiceUnavailableException(_)
        )
    }
    /// Returns `true` if the error kind is `StartMedicalStreamTranscriptionErrorKind::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartMedicalStreamTranscriptionErrorKind::BadRequestException(_)
        )
    }
    /// Returns `true` if the error kind is `StartMedicalStreamTranscriptionErrorKind::InternalFailureException`.
    pub fn is_internal_failure_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartMedicalStreamTranscriptionErrorKind::InternalFailureException(_)
        )
    }
    /// Returns `true` if the error kind is `StartMedicalStreamTranscriptionErrorKind::ConflictException`.
    pub fn is_conflict_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartMedicalStreamTranscriptionErrorKind::ConflictException(_)
        )
    }
    /// Returns `true` if the error kind is `StartMedicalStreamTranscriptionErrorKind::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartMedicalStreamTranscriptionErrorKind::LimitExceededException(_)
        )
    }
}
impl std::error::Error for StartMedicalStreamTranscriptionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            StartMedicalStreamTranscriptionErrorKind::ServiceUnavailableException(_inner) => {
                Some(_inner)
            }
            StartMedicalStreamTranscriptionErrorKind::BadRequestException(_inner) => Some(_inner),
            StartMedicalStreamTranscriptionErrorKind::InternalFailureException(_inner) => {
                Some(_inner)
            }
            StartMedicalStreamTranscriptionErrorKind::ConflictException(_inner) => Some(_inner),
            StartMedicalStreamTranscriptionErrorKind::LimitExceededException(_inner) => {
                Some(_inner)
            }
            StartMedicalStreamTranscriptionErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `StartStreamTranscription` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct StartStreamTranscriptionError {
    /// Kind of error that occurred.
    pub kind: StartStreamTranscriptionErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `StartStreamTranscription` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum StartStreamTranscriptionErrorKind {
    /// <p>Service is currently unavailable. Try your request later.</p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p>One or more arguments to the <code>StartStreamTranscription</code> or <code>StartMedicalStreamTranscription</code> operation was invalid. For example, <code>MediaEncoding</code> was not set to a valid encoding, or <code>LanguageCode</code> was not set to a valid code. Check the parameters and try your request again.</p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>A problem occurred while processing the audio. Amazon Transcribe or Amazon Transcribe Medical terminated processing. Try your request again.</p>
    InternalFailureException(crate::error::InternalFailureException),
    /// <p>A new stream started with the same session ID. The current stream has been terminated.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>You have exceeded the maximum number of concurrent transcription streams, are starting transcription streams too quickly, or the maximum audio length of 4 hours. Wait until a stream has finished processing, or break your audio stream into smaller chunks and try your request again.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for StartStreamTranscriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StartStreamTranscriptionErrorKind::ServiceUnavailableException(_inner) => _inner.fmt(f),
            StartStreamTranscriptionErrorKind::BadRequestException(_inner) => _inner.fmt(f),
            StartStreamTranscriptionErrorKind::InternalFailureException(_inner) => _inner.fmt(f),
            StartStreamTranscriptionErrorKind::ConflictException(_inner) => _inner.fmt(f),
            StartStreamTranscriptionErrorKind::LimitExceededException(_inner) => _inner.fmt(f),
            StartStreamTranscriptionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for StartStreamTranscriptionError {
    fn code(&self) -> Option<&str> {
        StartStreamTranscriptionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl StartStreamTranscriptionError {
    /// Creates a new `StartStreamTranscriptionError`.
    pub fn new(kind: StartStreamTranscriptionErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `StartStreamTranscriptionError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: StartStreamTranscriptionErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `StartStreamTranscriptionError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: StartStreamTranscriptionErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `StartStreamTranscriptionErrorKind::ServiceUnavailableException`.
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartStreamTranscriptionErrorKind::ServiceUnavailableException(_)
        )
    }
    /// Returns `true` if the error kind is `StartStreamTranscriptionErrorKind::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartStreamTranscriptionErrorKind::BadRequestException(_)
        )
    }
    /// Returns `true` if the error kind is `StartStreamTranscriptionErrorKind::InternalFailureException`.
    pub fn is_internal_failure_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartStreamTranscriptionErrorKind::InternalFailureException(_)
        )
    }
    /// Returns `true` if the error kind is `StartStreamTranscriptionErrorKind::ConflictException`.
    pub fn is_conflict_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartStreamTranscriptionErrorKind::ConflictException(_)
        )
    }
    /// Returns `true` if the error kind is `StartStreamTranscriptionErrorKind::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartStreamTranscriptionErrorKind::LimitExceededException(_)
        )
    }
}
impl std::error::Error for StartStreamTranscriptionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            StartStreamTranscriptionErrorKind::ServiceUnavailableException(_inner) => Some(_inner),
            StartStreamTranscriptionErrorKind::BadRequestException(_inner) => Some(_inner),
            StartStreamTranscriptionErrorKind::InternalFailureException(_inner) => Some(_inner),
            StartStreamTranscriptionErrorKind::ConflictException(_inner) => Some(_inner),
            StartStreamTranscriptionErrorKind::LimitExceededException(_inner) => Some(_inner),
            StartStreamTranscriptionErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>You have exceeded the maximum number of concurrent transcription streams, are starting transcription streams too quickly, or the maximum audio length of 4 hours. Wait until a stream has finished processing, or break your audio stream into smaller chunks and try your request again.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct LimitExceededException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for LimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("LimitExceededException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl LimitExceededException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for LimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LimitExceededException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for LimitExceededException {}
/// See [`LimitExceededException`](crate::error::LimitExceededException)
pub mod limit_exceeded_exception {

    /// A builder for [`LimitExceededException`](crate::error::LimitExceededException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`LimitExceededException`](crate::error::LimitExceededException)
        pub fn build(self) -> crate::error::LimitExceededException {
            crate::error::LimitExceededException {
                message: self.message,
            }
        }
    }
}
impl LimitExceededException {
    /// Creates a new builder-style object to manufacture [`LimitExceededException`](crate::error::LimitExceededException)
    pub fn builder() -> crate::error::limit_exceeded_exception::Builder {
        crate::error::limit_exceeded_exception::Builder::default()
    }
}

/// <p>A new stream started with the same session ID. The current stream has been terminated.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ConflictException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ConflictException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ConflictException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ConflictException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ConflictException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConflictException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for ConflictException {}
/// See [`ConflictException`](crate::error::ConflictException)
pub mod conflict_exception {

    /// A builder for [`ConflictException`](crate::error::ConflictException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ConflictException`](crate::error::ConflictException)
        pub fn build(self) -> crate::error::ConflictException {
            crate::error::ConflictException {
                message: self.message,
            }
        }
    }
}
impl ConflictException {
    /// Creates a new builder-style object to manufacture [`ConflictException`](crate::error::ConflictException)
    pub fn builder() -> crate::error::conflict_exception::Builder {
        crate::error::conflict_exception::Builder::default()
    }
}

/// <p>A problem occurred while processing the audio. Amazon Transcribe or Amazon Transcribe Medical terminated processing. Try your request again.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalFailureException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalFailureException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalFailureException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalFailureException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalFailureException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalFailureException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalFailureException {}
/// See [`InternalFailureException`](crate::error::InternalFailureException)
pub mod internal_failure_exception {

    /// A builder for [`InternalFailureException`](crate::error::InternalFailureException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalFailureException`](crate::error::InternalFailureException)
        pub fn build(self) -> crate::error::InternalFailureException {
            crate::error::InternalFailureException {
                message: self.message,
            }
        }
    }
}
impl InternalFailureException {
    /// Creates a new builder-style object to manufacture [`InternalFailureException`](crate::error::InternalFailureException)
    pub fn builder() -> crate::error::internal_failure_exception::Builder {
        crate::error::internal_failure_exception::Builder::default()
    }
}

/// <p>One or more arguments to the <code>StartStreamTranscription</code> or <code>StartMedicalStreamTranscription</code> operation was invalid. For example, <code>MediaEncoding</code> was not set to a valid encoding, or <code>LanguageCode</code> was not set to a valid code. Check the parameters and try your request again.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BadRequestException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for BadRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BadRequestException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl BadRequestException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for BadRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BadRequestException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for BadRequestException {}
/// See [`BadRequestException`](crate::error::BadRequestException)
pub mod bad_request_exception {

    /// A builder for [`BadRequestException`](crate::error::BadRequestException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`BadRequestException`](crate::error::BadRequestException)
        pub fn build(self) -> crate::error::BadRequestException {
            crate::error::BadRequestException {
                message: self.message,
            }
        }
    }
}
impl BadRequestException {
    /// Creates a new builder-style object to manufacture [`BadRequestException`](crate::error::BadRequestException)
    pub fn builder() -> crate::error::bad_request_exception::Builder {
        crate::error::bad_request_exception::Builder::default()
    }
}

/// <p>Service is currently unavailable. Try your request later.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ServiceUnavailableException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ServiceUnavailableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ServiceUnavailableException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ServiceUnavailableException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ServiceUnavailableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUnavailableException")?;
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
        }
        Ok(())
    }
}
impl std::error::Error for ServiceUnavailableException {}
/// See [`ServiceUnavailableException`](crate::error::ServiceUnavailableException)
pub mod service_unavailable_exception {

    /// A builder for [`ServiceUnavailableException`](crate::error::ServiceUnavailableException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ServiceUnavailableException`](crate::error::ServiceUnavailableException)
        pub fn build(self) -> crate::error::ServiceUnavailableException {
            crate::error::ServiceUnavailableException {
                message: self.message,
            }
        }
    }
}
impl ServiceUnavailableException {
    /// Creates a new builder-style object to manufacture [`ServiceUnavailableException`](crate::error::ServiceUnavailableException)
    pub fn builder() -> crate::error::service_unavailable_exception::Builder {
        crate::error::service_unavailable_exception::Builder::default()
    }
}
