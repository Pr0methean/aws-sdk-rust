// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have the required privileges to perform this action.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>Unexpected error during processing of request.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>Request references a resource which does not exist.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>Request would cause a service quota to be exceeded.</p>
    ServiceQuotaExceededException(crate::types::error::ServiceQuotaExceededException),
    /// <p>Request was denied due to request throttling.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>The input does not satisfy the constraints specified by an AWS service.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-Error) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(_) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl From<::aws_smithy_types::error::operation::BuildError> for Error {
    fn from(value: ::aws_smithy_types::error::operation::BuildError) -> Self {
        Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: value.into(),
            meta: ::std::default::Default::default(),
        })
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {
    fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {
        match self {
            Self::AccessDeniedException(inner) => inner.meta(),
            Self::ConflictException(inner) => inner.meta(),
            Self::InternalServerException(inner) => inner.meta(),
            Self::ResourceNotFoundException(inner) => inner.meta(),
            Self::ServiceQuotaExceededException(inner) => inner.meta(),
            Self::ThrottlingException(inner) => inner.meta(),
            Self::ValidationException(inner) => inner.meta(),
            Self::Unhandled(inner) => &inner.meta,
        }
    }
}
impl<R>
    From<
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError> for Error {
    fn from(err: crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError) -> Self {
        match err {
            crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::create_bill_of_materials_import_job::CreateBillOfMaterialsImportJobError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError> for Error {
    fn from(err: crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError) -> Self {
        match err {
            crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::get_bill_of_materials_import_job::GetBillOfMaterialsImportJobError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::AccessDeniedException(inner) => inner.source(),
            Error::ConflictException(inner) => inner.source(),
            Error::InternalServerException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::ServiceQuotaExceededException(inner) => inner.source(),
            Error::ThrottlingException(inner) => inner.source(),
            Error::ValidationException(inner) => inner.source(),
            Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
        }
    }
}
impl ::aws_types::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::ConflictException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ServiceQuotaExceededException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.meta.request_id(),
        }
    }
}
