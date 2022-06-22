// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `CloseTunnel` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct CloseTunnelError {
    /// Kind of error that occurred.
    pub kind: CloseTunnelErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `CloseTunnel` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum CloseTunnelErrorKind {
    /// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for CloseTunnelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            CloseTunnelErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            CloseTunnelErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for CloseTunnelError {
    fn code(&self) -> Option<&str> {
        CloseTunnelError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl CloseTunnelError {
    /// Creates a new `CloseTunnelError`.
    pub fn new(kind: CloseTunnelErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `CloseTunnelError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: CloseTunnelErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `CloseTunnelError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: CloseTunnelErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `CloseTunnelErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            CloseTunnelErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for CloseTunnelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            CloseTunnelErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            CloseTunnelErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `DescribeTunnel` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DescribeTunnelError {
    /// Kind of error that occurred.
    pub kind: DescribeTunnelErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `DescribeTunnel` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DescribeTunnelErrorKind {
    /// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DescribeTunnelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeTunnelErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            DescribeTunnelErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DescribeTunnelError {
    fn code(&self) -> Option<&str> {
        DescribeTunnelError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeTunnelError {
    /// Creates a new `DescribeTunnelError`.
    pub fn new(kind: DescribeTunnelErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DescribeTunnelError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DescribeTunnelErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `DescribeTunnelError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DescribeTunnelErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `DescribeTunnelErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeTunnelErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for DescribeTunnelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeTunnelErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            DescribeTunnelErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `ListTagsForResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListTagsForResourceError {
    /// Kind of error that occurred.
    pub kind: ListTagsForResourceErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `ListTagsForResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListTagsForResourceErrorKind {
    /// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListTagsForResourceErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            ListTagsForResourceErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListTagsForResourceError {
    fn code(&self) -> Option<&str> {
        ListTagsForResourceError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListTagsForResourceError {
    /// Creates a new `ListTagsForResourceError`.
    pub fn new(kind: ListTagsForResourceErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `ListTagsForResourceError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ListTagsForResourceErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `ListTagsForResourceError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ListTagsForResourceErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `ListTagsForResourceErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListTagsForResourceErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for ListTagsForResourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListTagsForResourceErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            ListTagsForResourceErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `ListTunnels` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListTunnelsError {
    /// Kind of error that occurred.
    pub kind: ListTunnelsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `ListTunnels` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListTunnelsErrorKind {
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListTunnelsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListTunnelsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListTunnelsError {
    fn code(&self) -> Option<&str> {
        ListTunnelsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListTunnelsError {
    /// Creates a new `ListTunnelsError`.
    pub fn new(kind: ListTunnelsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `ListTunnelsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ListTunnelsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `ListTunnelsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ListTunnelsErrorKind::Unhandled(err.into()),
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
}
impl std::error::Error for ListTunnelsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListTunnelsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `OpenTunnel` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct OpenTunnelError {
    /// Kind of error that occurred.
    pub kind: OpenTunnelErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `OpenTunnel` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum OpenTunnelErrorKind {
    /// <p>Thrown when a tunnel limit is exceeded.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for OpenTunnelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            OpenTunnelErrorKind::LimitExceededException(_inner) => _inner.fmt(f),
            OpenTunnelErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for OpenTunnelError {
    fn code(&self) -> Option<&str> {
        OpenTunnelError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl OpenTunnelError {
    /// Creates a new `OpenTunnelError`.
    pub fn new(kind: OpenTunnelErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `OpenTunnelError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: OpenTunnelErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `OpenTunnelError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: OpenTunnelErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `OpenTunnelErrorKind::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(&self.kind, OpenTunnelErrorKind::LimitExceededException(_))
    }
}
impl std::error::Error for OpenTunnelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            OpenTunnelErrorKind::LimitExceededException(_inner) => Some(_inner),
            OpenTunnelErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `RotateTunnelAccessToken` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct RotateTunnelAccessTokenError {
    /// Kind of error that occurred.
    pub kind: RotateTunnelAccessTokenErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `RotateTunnelAccessToken` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum RotateTunnelAccessTokenErrorKind {
    /// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for RotateTunnelAccessTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            RotateTunnelAccessTokenErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            RotateTunnelAccessTokenErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for RotateTunnelAccessTokenError {
    fn code(&self) -> Option<&str> {
        RotateTunnelAccessTokenError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl RotateTunnelAccessTokenError {
    /// Creates a new `RotateTunnelAccessTokenError`.
    pub fn new(kind: RotateTunnelAccessTokenErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `RotateTunnelAccessTokenError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: RotateTunnelAccessTokenErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `RotateTunnelAccessTokenError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: RotateTunnelAccessTokenErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `RotateTunnelAccessTokenErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            RotateTunnelAccessTokenErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for RotateTunnelAccessTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            RotateTunnelAccessTokenErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            RotateTunnelAccessTokenErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `TagResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct TagResourceError {
    /// Kind of error that occurred.
    pub kind: TagResourceErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `TagResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum TagResourceErrorKind {
    /// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TagResourceErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            TagResourceErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for TagResourceError {
    fn code(&self) -> Option<&str> {
        TagResourceError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl TagResourceError {
    /// Creates a new `TagResourceError`.
    pub fn new(kind: TagResourceErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `TagResourceError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: TagResourceErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `TagResourceError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: TagResourceErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `TagResourceErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            TagResourceErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for TagResourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            TagResourceErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            TagResourceErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `UntagResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct UntagResourceError {
    /// Kind of error that occurred.
    pub kind: UntagResourceErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `UntagResource` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum UntagResourceErrorKind {
    /// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            UntagResourceErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            UntagResourceErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for UntagResourceError {
    fn code(&self) -> Option<&str> {
        UntagResourceError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl UntagResourceError {
    /// Creates a new `UntagResourceError`.
    pub fn new(kind: UntagResourceErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `UntagResourceError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: UntagResourceErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `UntagResourceError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: UntagResourceErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `UntagResourceErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            UntagResourceErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for UntagResourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            UntagResourceErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            UntagResourceErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceNotFoundException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ResourceNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
pub mod resource_not_found_exception {

    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
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
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>Thrown when a tunnel limit is exceeded.</p>
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
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
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
