//! # appbiotic-code-error
//!
//! Appbiotic Code Error is a set of types to make it easier assembling
//! services and applications with similar error reporting to the end-user.
//! It is not necessarily meant for lower level libraries such as adding
//! derived traits for serialization, or database libraries where
//! specifically-typed error handling is required.
//!
//! This component's Rust-based API is original; however, the error codes and
//! descriptions are copied directly from the
//! https://github.com/googleapis/googleapis project.

use std::fmt;

use strum_macros::IntoStaticStr;

#[cfg(feature = "safer-ffi")]
pub mod ffi;

pub mod code {
    pub const OK: i32 = 0;
    pub const CANCELLED: i32 = 1;
    pub const UNKNOWN: i32 = 2;
    pub const INVALID_ARGUMENT: i32 = 3;
    pub const DEADLINE_EXCEEDED: i32 = 4;
    pub const NOT_FOUND: i32 = 5;
    pub const ALREADY_EXISTS: i32 = 6;
    pub const PERMISSION_DENIED: i32 = 7;
    pub const UNAUTHENTICATED: i32 = 16;
    pub const RESOURCE_EXHAUSTED: i32 = 8;
    pub const FAILED_PRECONDITION: i32 = 9;
    pub const ABORTED: i32 = 10;
    pub const OUT_OF_RANGE: i32 = 11;
    pub const UNIMPLEMENTED: i32 = 12;
    pub const INTERNAL: i32 = 13;
    pub const UNAVAILABLE: i32 = 14;
    pub const DATA_LOSS: i32 = 15;
}

// TODO: Find or create library for format and flow markdown comments.

// TODO: Add serde deserializer for Error and related types.

#[derive(Clone, Debug, thiserror::Error, IntoStaticStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub enum Error {
    /// The operation was cancelled, typically by the caller.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  499 | Client Closed Request                               |
    /// | gRPC    |    1 | Cancelled                                           |
    Cancelled(ErrorStatus),

    /// Unknown error. For example, this error may be returned when a [`ErrorStatus`]
    /// value received from another address space belongs to an error space
    /// that is not known in this address space. Also errors raised by APIs
    /// that do not return enough error information may be converted to this
    /// error.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  500 | Internal Server Error                               |
    /// | gRPC    |    2 | Unknown                                             |
    Unknown(ErrorStatus),

    /// The client specified an invalid argument.  Note that this differs
    /// from [`Error::FailedPrecondition`].  [`Error::InvalidArgument`] indicates arguments
    /// that are problematic regardless of the state of the system
    /// (e.g., a malformed file name).
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  400 | Bad Request                                         |
    /// | gRPC    |    3 | Invalid argument                                    |
    InvalidArgument(ErrorStatus),

    /// The deadline expired before the operation could complete. For operations
    /// that change the state of the system, this error may be returned
    /// even if the operation has completed successfully.  For example, a
    /// successful response from a server could have been delayed long
    /// enough for the deadline to expire.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  504 | Gateway Timeout                                     |
    /// | gRPC    |    4 | Deadline exceeded                                   |
    DeadlineExceeded(ErrorStatus),

    /// Some requested entity (e.g., file or directory) was not found.
    ///
    /// Note to server developers: if a request is denied for an entire class
    /// of users, such as gradual feature rollout or undocumented allowlist,
    /// [`Error::NotFound`] may be used. If a request is denied for some users
    /// within a class of users, such as user-based access control,
    /// [`Error::PermissionDenied`] must be used.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  404 | Not Found                                           |
    /// | gRPC    |    5 | Not found                                           |
    NotFound(ErrorStatus),

    /// The entity that a client attempted to create (e.g., file or directory)
    /// already exists.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  409 | Conflict                                            |
    /// | gRPC    |    6 | Already exists                                      |
    AlreadyExists(ErrorStatus),

    /// The caller does not have permission to execute the specified
    /// operation. [`Error::PermissionDenied`] must not be used for rejections
    /// caused by exhausting some resource (use [`Error::ResourceExhausted`]
    /// instead for those errors). [`Error::PermissionDenied`] must not be
    /// used if the caller can not be identified (use [`Error::Unauthenticated`]
    /// instead for those errors). This error code does not imply the
    /// request is valid or the requested entity exists or satisfies
    /// other pre-conditions.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  403 | Forbidden                                           |
    /// | gRPC    |    7 | Permission denied                                   |
    PermissionDenied(ErrorStatus),

    /// The request does not have valid authentication credentials for the
    /// operation.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  401 | Unauthorized                                        |
    /// | gRPC    |   16 | Permission denied                                   |
    Unauthenticated(ErrorStatus),

    /// Some resource has been exhausted, perhaps a per-user quota, or
    /// perhaps the entire file system is out of space.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  429 | Too Many Requests                                   |
    /// | gRPC    |    8 | Permission denied                                   |
    ResourceExhausted(ErrorStatus),

    /// The operation was rejected because the system is not in a state
    /// required for the operation's execution.  For example, the directory
    /// to be deleted is non-empty, an rmdir operation is applied to
    /// a non-directory, etc.
    ///
    /// Service implementors can use the following guidelines to decide
    /// between [`Error::FailedPrecondition`], [`Error::Aborted`], and
    /// [`Error::Unavailable`]:
    ///
    ///  - Use [`Error::Unavailable`] if the client can retry just the failing
    ///    call.
    ///  - Use [`Error::Aborted`] if the client should retry at a higher level.
    ///    For example, when a client-specified test-and-set fails, indicating
    ///    the client should restart a read-modify-write sequence.
    ///  - Use [`Error::FailedPrecondition`] if the client should not retry
    ///    until the system state has been explicitly fixed. For example, if an
    ///    "rmdir" fails because the directory is non-empty,
    ///    [`Error::FailedPrecondition`] should be returned since the client
    ///    should not retry unless the files are deleted from the directory.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  400 | Bad Request                                         |
    /// | gRPC    |    9 | Failed precondition                                 |
    FailedPrecondition(ErrorStatus),

    /// The operation was aborted, typically due to a concurrency issue such as
    /// a sequencer check failure or transaction abort.
    ///
    /// See the guidelines above for deciding between
    /// [`Error::FailedPrecondition`], [`Error::Aborted`], and
    /// [`Error::Unavailable`].
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  409 | Conflict                                            |
    /// | gRPC    |   10 | Aborted                                             |
    Aborted(ErrorStatus),

    /// The operation was attempted past the valid range.  E.g., seeking or
    /// reading past end-of-file.
    ///
    /// Unlike [`Error::InvalidArgument`], this error indicates a problem that
    /// may be fixed if the system state changes. For example, a 32-bit file
    /// system will generate [`Error::InvalidArgument`] if asked to read at an
    /// offset that is not in the range [0,2^32-1], but it will generate
    /// [`Error::OutOfRange`] if asked to read from an offset past the current
    /// file size.
    ///
    /// There is a fair bit of overlap between [`Error::FailedPrecondition`] and
    /// [`Error::OutOfRange`].  We recommend using [`Error::OutOfRange`] (the
    /// more specific error) when it applies so that callers who are iterating
    /// through a space can easily look for an [`Error::OutOfRange`] error to
    /// detect when they are done.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  400 | Bad Request                                         |
    /// | gRPC    |   11 | Out of range                                        |
    OutOfRange(ErrorStatus),

    /// The operation is not implemented or is not supported/enabled in this
    /// service.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  501 | Not implemented                                     |
    /// | gRPC    |   12 | Unimplemented                                       |
    Unimplemented(ErrorStatus),

    /// Internal errors.  This means that some invariants expected by the
    /// underlying system have been broken.  This error code is reserved for
    /// serious errors.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  500 | Internal Server Error                               |
    /// | gRPC    |   13 | Internal                                            |
    Internal(ErrorStatus),

    /// The service is currently unavailable.  This is most likely a transient
    /// condition, which can be corrected by retrying with
    /// a backoff. Note that it is not always safe to retry
    /// non-idempotent operations.
    ///
    /// See the guidelines above for deciding between
    /// [`Error::FailedPrecondition`], [`Error::Aborted`], and
    /// [`Error::Unavailable`].
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  503 | Service Unavailable                                 |
    /// | gRPC    |   14 | Unavailable                                         |
    Unavailable(ErrorStatus),

    /// Unrecoverable data loss or corruption.
    ///
    /// | Mapping | Code | Description                                         |
    /// | :------ | ---: | :-------------------------------------------------- |
    /// | HTTP    |  500 | Internal Server Error                               |
    /// | gRPC    |   15 | Data loss                                           |
    DataLoss(ErrorStatus),
}

// TODO: Replace strum with a more detailed display implementation.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl Error {
    pub fn inner(&self) -> &ErrorStatus {
        match self {
            Error::Internal(status) => status,
            Error::Unknown(status) => status,
            Error::Cancelled(status) => status,
            Error::InvalidArgument(status) => status,
            Error::DeadlineExceeded(status) => status,
            Error::NotFound(status) => status,
            Error::AlreadyExists(status) => status,
            Error::PermissionDenied(status) => status,
            Error::Unauthenticated(status) => status,
            Error::ResourceExhausted(status) => status,
            Error::FailedPrecondition(status) => status,
            Error::Aborted(status) => status,
            Error::OutOfRange(status) => status,
            Error::Unimplemented(status) => status,
            Error::Unavailable(status) => status,
            Error::DataLoss(status) => status,
        }
    }
}

impl From<Error> for ErrorStatus {
    fn from(value: Error) -> Self {
        match value {
            Error::Internal(status) => status,
            Error::Unknown(status) => status,
            Error::Cancelled(status) => status,
            Error::InvalidArgument(status) => status,
            Error::DeadlineExceeded(status) => status,
            Error::NotFound(status) => status,
            Error::AlreadyExists(status) => status,
            Error::PermissionDenied(status) => status,
            Error::Unauthenticated(status) => status,
            Error::ResourceExhausted(status) => status,
            Error::FailedPrecondition(status) => status,
            Error::Aborted(status) => status,
            Error::OutOfRange(status) => status,
            Error::Unimplemented(status) => status,
            Error::Unavailable(status) => status,
            Error::DataLoss(status) => status,
        }
    }
}

impl Error {
    /// Returns the gRPC code value.
    ///
    /// See https://github.com/googleapis/googleapis/blob/f36c65081b19e0758ef5696feca27c7dcee5475e/google/rpc/code.proto.
    pub fn code(&self) -> i32 {
        match self {
            Error::Cancelled(_) => code::CANCELLED,
            Error::Unknown(_) => code::UNKNOWN,
            Error::InvalidArgument(_) => code::INVALID_ARGUMENT,
            Error::DeadlineExceeded(_) => code::DEADLINE_EXCEEDED,
            Error::NotFound(_) => code::NOT_FOUND,
            Error::AlreadyExists(_) => code::ALREADY_EXISTS,
            Error::PermissionDenied(_) => code::PERMISSION_DENIED,
            Error::Unauthenticated(_) => code::UNAUTHENTICATED,
            Error::ResourceExhausted(_) => code::RESOURCE_EXHAUSTED,
            Error::FailedPrecondition(_) => code::FAILED_PRECONDITION,
            Error::Aborted(_) => code::ABORTED,
            Error::OutOfRange(_) => code::OUT_OF_RANGE,
            Error::Unimplemented(_) => code::UNIMPLEMENTED,
            Error::Internal(_) => code::INTERNAL,
            Error::Unavailable(_) => code::UNAVAILABLE,
            Error::DataLoss(_) => code::DATA_LOSS,
        }
    }

    // TODO: Build macros to automate building of the error helper functions.

    pub fn cancelled(message: Option<String>) -> Error {
        Error::Cancelled(ErrorStatus::default().with_message(message))
    }

    pub fn unknown(message: Option<String>) -> Error {
        Error::Unknown(ErrorStatus::default().with_message(message))
    }

    pub fn invalid_argument(message: Option<String>) -> Error {
        Error::InvalidArgument(ErrorStatus::default().with_message(message))
    }

    pub fn invalid_argument_field(
        message: Option<String>,
        name: String,
        description: Option<String>,
    ) -> Error {
        Error::InvalidArgument(ErrorStatus::default().with_message(message).with_details(
            ErrorDetails::bad_request(FieldViolation::for_member(name, description)),
        ))
    }

    pub fn invalid_argument_field_name<N: AsRef<str>, D: AsRef<str>>(
        name: N,
        description: D,
    ) -> Error {
        Error::InvalidArgument(
            ErrorStatus::default().with_details(ErrorDetails::bad_request(
                FieldViolation::for_member(
                    name.as_ref().to_owned(),
                    Some(description.as_ref().to_owned()),
                ),
            )),
        )
    }

    pub fn deadline_exceeded(message: Option<String>) -> Error {
        Error::DeadlineExceeded(ErrorStatus::default().with_message(message))
    }

    pub fn not_found(message: Option<String>) -> Error {
        Error::NotFound(ErrorStatus::default().with_message(message))
    }

    pub fn already_exists(message: Option<String>) -> Error {
        Error::AlreadyExists(ErrorStatus::default().with_message(message))
    }

    pub fn permission_denied(message: Option<String>) -> Error {
        Error::PermissionDenied(ErrorStatus::default().with_message(message))
    }

    pub fn unauthenticated(message: Option<String>) -> Error {
        Error::Unauthenticated(ErrorStatus::default().with_message(message))
    }

    pub fn resource_exhausted(message: Option<String>) -> Error {
        Error::ResourceExhausted(ErrorStatus::default().with_message(message))
    }

    pub fn failed_precondition(message: Option<String>) -> Error {
        Error::FailedPrecondition(ErrorStatus::default().with_message(message))
    }

    pub fn aborted(message: Option<String>) -> Error {
        Error::Aborted(ErrorStatus::default().with_message(message))
    }

    pub fn out_of_range(message: Option<String>) -> Error {
        Error::OutOfRange(ErrorStatus::default().with_message(message))
    }

    pub fn unimplemented(message: Option<String>) -> Error {
        Error::Unimplemented(ErrorStatus::default().with_message(message))
    }

    pub fn internal(message: Option<String>) -> Error {
        Error::Internal(ErrorStatus::default().with_message(message))
    }

    pub fn unavailable(message: Option<String>) -> Error {
        Error::Unavailable(ErrorStatus::default().with_message(message))
    }

    pub fn data_loss(message: Option<String>) -> Error {
        Error::DataLoss(ErrorStatus::default().with_message(message))
    }

    /// Appends a `ErrorDetails::DebugInfo` with info from `error`.
    pub fn with_error<E: fmt::Display>(self, error: E) -> Error {
        match self {
            Error::Cancelled(details) => Error::Cancelled(details.with_error(error)),
            Error::Unknown(details) => Error::Unknown(details.with_error(error)),
            Error::InvalidArgument(details) => Error::InvalidArgument(details.with_error(error)),
            Error::DeadlineExceeded(details) => Error::DeadlineExceeded(details.with_error(error)),
            Error::NotFound(details) => Error::NotFound(details.with_error(error)),
            Error::AlreadyExists(details) => Error::AlreadyExists(details.with_error(error)),
            Error::PermissionDenied(details) => Error::PermissionDenied(details.with_error(error)),
            Error::Unauthenticated(details) => Error::Unauthenticated(details.with_error(error)),
            Error::ResourceExhausted(details) => {
                Error::ResourceExhausted(details.with_error(error))
            }
            Error::FailedPrecondition(details) => {
                Error::FailedPrecondition(details.with_error(error))
            }
            Error::Aborted(details) => Error::Aborted(details.with_error(error)),
            Error::OutOfRange(details) => Error::OutOfRange(details.with_error(error)),
            Error::Unimplemented(details) => Error::Unimplemented(details.with_error(error)),
            Error::Internal(details) => Error::Internal(details.with_error(error)),
            Error::Unavailable(details) => Error::Unavailable(details.with_error(error)),
            Error::DataLoss(details) => Error::DataLoss(details.with_error(error)),
        }
    }
}

#[cfg(feature = "with-http")]
impl From<Error> for http::StatusCode {
    fn from(value: Error) -> Self {
        match value {
            Error::Cancelled(_) => {
                http::StatusCode::from_u16(499).unwrap_or(http::StatusCode::IM_A_TEAPOT)
            }
            Error::Unknown(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidArgument(_) => http::StatusCode::BAD_REQUEST,
            Error::DeadlineExceeded(_) => http::StatusCode::GATEWAY_TIMEOUT,
            Error::NotFound(_) => http::StatusCode::NOT_FOUND,
            Error::AlreadyExists(_) => http::StatusCode::CONFLICT,
            Error::PermissionDenied(_) => http::StatusCode::FORBIDDEN,
            Error::Unauthenticated(_) => http::StatusCode::UNAUTHORIZED,
            Error::ResourceExhausted(_) => http::StatusCode::TOO_MANY_REQUESTS,
            Error::FailedPrecondition(_) => http::StatusCode::BAD_REQUEST,
            Error::Aborted(_) => http::StatusCode::CONFLICT,
            Error::OutOfRange(_) => http::StatusCode::BAD_REQUEST,
            Error::Unimplemented(_) => http::StatusCode::NOT_IMPLEMENTED,
            Error::Internal(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unavailable(_) => http::StatusCode::SERVICE_UNAVAILABLE,
            Error::DataLoss(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// TODO: Properly map error details into a tonic Status.
#[cfg(feature = "protos")]
impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Internal(status) => {
                tonic::Status::new(tonic::Code::Internal, status.message.unwrap_or_default())
            }
            Error::Cancelled(status) => {
                tonic::Status::new(tonic::Code::Cancelled, status.message.unwrap_or_default())
            }
            Error::Unknown(status) => {
                tonic::Status::new(tonic::Code::Unknown, status.message.unwrap_or_default())
            }
            Error::InvalidArgument(status) => tonic::Status::new(
                tonic::Code::InvalidArgument,
                status.message.unwrap_or_default(),
            ),
            Error::DeadlineExceeded(status) => tonic::Status::new(
                tonic::Code::DeadlineExceeded,
                status.message.unwrap_or_default(),
            ),
            Error::NotFound(status) => {
                tonic::Status::new(tonic::Code::NotFound, status.message.unwrap_or_default())
            }
            Error::AlreadyExists(status) => tonic::Status::new(
                tonic::Code::AlreadyExists,
                status.message.unwrap_or_default(),
            ),
            Error::PermissionDenied(status) => tonic::Status::new(
                tonic::Code::PermissionDenied,
                status.message.unwrap_or_default(),
            ),
            Error::Unauthenticated(status) => tonic::Status::new(
                tonic::Code::Unauthenticated,
                status.message.unwrap_or_default(),
            ),
            Error::ResourceExhausted(status) => tonic::Status::new(
                tonic::Code::ResourceExhausted,
                status.message.unwrap_or_default(),
            ),
            Error::FailedPrecondition(status) => tonic::Status::new(
                tonic::Code::FailedPrecondition,
                status.message.unwrap_or_default(),
            ),
            Error::Aborted(status) => {
                tonic::Status::new(tonic::Code::Aborted, status.message.unwrap_or_default())
            }
            Error::OutOfRange(status) => {
                tonic::Status::new(tonic::Code::OutOfRange, status.message.unwrap_or_default())
            }
            Error::Unimplemented(status) => tonic::Status::new(
                tonic::Code::Unimplemented,
                status.message.unwrap_or_default(),
            ),
            Error::Unavailable(status) => {
                tonic::Status::new(tonic::Code::Unavailable, status.message.unwrap_or_default())
            }
            Error::DataLoss(status) => {
                tonic::Status::new(tonic::Code::DataLoss, status.message.unwrap_or_default())
            }
        }
    }
}

#[cfg(feature = "protos")]
impl TryFrom<tonic::Status> for Error {
    type Error = Error;

    fn try_from(value: tonic::Status) -> std::result::Result<Self, Self::Error> {
        match value.code() {
            tonic::Code::Ok => Err(Error::invalid_argument(Some(
                "Cannot convert OK status to Error".to_owned(),
            ))),
            tonic::Code::Cancelled => Ok(Error::cancelled(Some(value.message().to_owned()))),
            tonic::Code::Unknown => Ok(Error::unknown(Some(value.message().to_owned()))),
            tonic::Code::InvalidArgument => {
                Ok(Error::invalid_argument(Some(value.message().to_owned())))
            }
            tonic::Code::DeadlineExceeded => {
                Ok(Error::deadline_exceeded(Some(value.message().to_owned())))
            }
            tonic::Code::NotFound => Ok(Error::not_found(Some(value.message().to_owned()))),
            tonic::Code::AlreadyExists => {
                Ok(Error::already_exists(Some(value.message().to_owned())))
            }
            tonic::Code::PermissionDenied => {
                Ok(Error::permission_denied(Some(value.message().to_owned())))
            }
            tonic::Code::ResourceExhausted => {
                Ok(Error::resource_exhausted(Some(value.message().to_owned())))
            }
            tonic::Code::FailedPrecondition => {
                Ok(Error::failed_precondition(Some(value.message().to_owned())))
            }
            tonic::Code::Aborted => Ok(Error::aborted(Some(value.message().to_owned()))),
            tonic::Code::OutOfRange => Ok(Error::out_of_range(Some(value.message().to_owned()))),
            tonic::Code::Unimplemented => {
                Ok(Error::unimplemented(Some(value.message().to_owned())))
            }
            tonic::Code::Internal => Ok(Error::internal(Some(value.message().to_owned()))),
            tonic::Code::Unavailable => Ok(Error::unavailable(Some(value.message().to_owned()))),
            tonic::Code::DataLoss => Ok(Error::data_loss(Some(value.message().to_owned()))),
            tonic::Code::Unauthenticated => {
                Ok(Error::unauthenticated(Some(value.message().to_owned())))
            }
        }
    }
}

/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](https://github.com/grpc). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
///
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](https://cloud.google.com/apis/design/errors).
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct ErrorStatus {
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// `details` field in a `ErrorDetails::LocalizedMessage`.
    pub message: Option<String>,
    /// A list of messages that carry the error details.  There is a common set
    /// of message types for APIs to use.    
    pub details: Option<Vec<ErrorDetails>>,
}

impl ErrorStatus {
    pub fn with_message(self, message: Option<String>) -> Self {
        ErrorStatus {
            message,
            details: self.details,
        }
    }

    pub fn with_details(self, details: ErrorDetails) -> Self {
        let mut new_details = self.details.unwrap_or_default();
        new_details.push(details);
        Self {
            message: self.message,
            details: Some(new_details),
        }
    }

    pub fn with_error<E: fmt::Display>(self, error: E) -> Self {
        self.with_details(ErrorDetails::DebugInfo {
            stack_entries: None,
            detail: Some(error.to_string()),
        })
    }
}

/// The specific details of an error that may be optionally forwarded to an
/// end-user.
///
/// These error detail kinds and documentation have been imported from
/// https://github.com/googleapis/googleapis/blob/f36c65081b19e0758ef5696feca27c7dcee5475e/google/rpc/error_details.proto.
#[derive(Clone, Debug, IntoStaticStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub enum ErrorDetails {
    /// Describes violations in a client request. This error type focuses on the
    /// syntactic aspects of the request.    
    BadRequest {
        /// Describes all violations in a client request.
        field_violations: Vec<FieldViolation>,
    },
    /// Describes additional debugging info.
    DebugInfo {
        /// The stack trace entries indicating where the error occurred.
        stack_entries: Option<Vec<String>>,
        /// Additional debugging information provided by the server.
        detail: Option<String>,
    },
    /// Provides a localized error message that is safe to return to the user
    /// which can be attached to an RPC error.
    LocalizedMessage {
        /// The locale used following the specification defined at
        /// <https://www.rfc-editor.org/rfc/bcp/bcp47.txt>.
        /// Examples are: "en-US", "fr-CH", "es-MX"
        locale: String,
        /// The localized error message in the above locale.
        message: String,
    },
}

impl ErrorDetails {
    pub fn bad_request(field_violation: FieldViolation) -> Self {
        ErrorDetails::BadRequest {
            field_violations: vec![field_violation],
        }
    }

    pub fn debug_info<D: AsRef<str>>(detail: D) -> Self {
        ErrorDetails::DebugInfo {
            stack_entries: None,
            detail: Some(detail.as_ref().to_owned()),
        }
    }

    pub fn localized_message<L: AsRef<str>, M: AsRef<str>>(locale: L, message: M) -> Self {
        ErrorDetails::LocalizedMessage {
            locale: locale.as_ref().to_owned(),
            message: message.as_ref().to_owned(),
        }
    }
}

// TODO: Replace strum with a more detailed display implementation.
impl fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

/// A message type used to describe a single bad request field.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct FieldViolation {
    /// A path that leads to a field in the request body. The value will be a
    /// sequence of dot-separated identifiers that identify a protocol buffer
    /// field.
    ///
    /// Consider the following:
    ///
    /// ```protobuf
    ///     message CreateContactRequest {
    ///       message EmailAddress {
    ///         enum Type {
    ///           TYPE_UNSPECIFIED = 0;
    ///           HOME = 1;
    ///           WORK = 2;
    ///         }
    ///
    ///         optional string email = 1;
    ///         repeated EmailType type = 2;
    ///       }
    ///
    ///       string full_name = 1;
    ///       repeated EmailAddress email_addresses = 2;
    ///     }   
    /// ```
    /// In this example, in proto `field` could take one of the following values:
    ///
    /// * `full_name` for a violation in the `full_name` value
    /// * `email_addresses[1].email` for a violation in the `email` field of the
    ///   first `email_addresses` message
    /// * `email_addresses[3].type[2]` for a violation in the second `type`
    ///   value in the third `email_addresses` message.
    ///
    /// In JSON, the same values are represented as:
    ///
    /// * `fullName` for a violation in the `fullName` value
    /// * `emailAddresses[1].email` for a violation in the `email` field of the
    ///   first `emailAddresses` message
    /// * `emailAddresses[3].type[2]` for a violation in the second `type`
    ///   value in the third `emailAddresses` message.    
    pub field: Field,
    /// A description of why the request element is bad.
    pub description: Option<String>,
}

impl FieldViolation {
    pub fn for_member(name: String, description: Option<String>) -> Self {
        Self {
            field: Field::new(Property::Member { name }),
            description,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    path_reversed: Vec<Property>,
}

#[cfg(feature = "json")]
impl serde::Serialize for Field {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Field {
    pub fn new(property: Property) -> Self {
        Field {
            path_reversed: vec![property],
        }
    }

    pub fn member<N: AsRef<str>>(name: N) -> Self {
        Self::new(Property::Member {
            name: name.as_ref().to_string(),
        })
    }

    pub fn array_member<N: AsRef<str>>(name: N, index: usize) -> Self {
        Self::new(Property::ArrayMember {
            name: name.as_ref().to_string(),
            index,
        })
    }

    pub fn with_context(mut self, context: Property) -> Self {
        self.path_reversed.push(context);
        self
    }

    pub fn within_member<M: AsRef<str>>(self, name: M) -> Self {
        self.with_context(Property::Member {
            name: name.as_ref().to_owned(),
        })
    }

    pub fn within_array_member<M: AsRef<str>>(self, name: M, index: usize) -> Self {
        self.with_context(Property::ArrayMember {
            name: name.as_ref().to_owned(),
            index,
        })
    }

    pub fn invalid_argument<M: AsRef<str>, D: AsRef<str>>(
        self,
        message: M,
        description: D,
    ) -> Error {
        Error::InvalidArgument(ErrorStatus {
            message: Some(message.as_ref().to_owned()),
            details: Some(vec![ErrorDetails::BadRequest {
                field_violations: vec![FieldViolation {
                    field: self,
                    description: Some(description.as_ref().to_owned()),
                }],
            }]),
        })
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..self.path_reversed.len()).rev() {
            write!(f, r#"{}"#, self.path_reversed.get(i).ok_or(fmt::Error)?,)?;
            if i > 0 {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Property {
    Member { name: String },
    MapMember { name: String, key: String },
    ArrayMember { name: String, index: usize },
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Property::Member { name } => write!(f, r#"{}"#, name),
            Property::MapMember { name, key } => write!(f, r#"{}["{}"]"#, name, key),
            Property::ArrayMember { name, index } => write!(f, r#"{}[{}]"#, name, index),
        }
    }
}

/// A request for inter-module communication.
#[derive(Clone)]
pub struct Request<T>
where
    T: Send,
{
    pub message: T,
}

impl<T> Request<T>
where
    T: Send,
{
    pub fn new(message: T) -> Self {
        Self { message }
    }
}

#[cfg(feature = "protos")]
impl<T, U> TryFrom<tonic::Request<T>> for Request<U>
where
    T: Send,
    U: Send,
    U: TryFrom<T>,
{
    type Error = U::Error;

    fn try_from(value: tonic::Request<T>) -> Result<Self, Self::Error> {
        Ok(Request {
            message: value.into_inner().try_into()?,
        })
    }
}

/// A response for inter-module communication.
#[derive(Clone)]
pub struct Response<T>
where
    T: Send,
{
    pub message: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let error = Error::internal(Some("Something bad happened".to_string()))
            .with_error("Invalid operation");
        assert_eq!(error.to_string(), "INTERNAL");
        assert!(error
            .inner()
            .details
            .as_ref()
            .expect("some error details")
            .iter()
            .any(|d| &d.to_string() == "DEBUG_INFO"));
    }

    #[test]
    fn property_member_display() {
        let field = Property::Member {
            name: "nickname".to_string(),
        };
        assert_eq!(field.to_string().as_str(), "nickname");
    }

    #[test]
    fn property_map_member_display() {
        let field = Property::MapMember {
            name: "children".to_string(),
            key: "son".to_string(),
        };
        assert_eq!(field.to_string().as_str(), r#"children["son"]"#);
    }

    #[test]
    fn property_array_member_display() {
        let field = Property::ArrayMember {
            name: "children".to_string(),
            index: 3,
        };
        assert_eq!(field.to_string().as_str(), r#"children[3]"#);
    }

    #[test]
    fn property_display() {
        let argument = Field::new(Property::MapMember {
            name: "nicknames".to_string(),
            key: "joe".to_string(),
        })
        .with_context(Property::ArrayMember {
            name: "children".to_string(),
            index: 3,
        })
        .with_context(Property::Member {
            name: "family".to_string(),
        });

        assert_eq!(
            argument.to_string().as_str(),
            r#"family.children[3].nicknames["joe"]"#
        );
    }

    #[test]
    #[cfg(feature = "json")]
    fn field_serialize_json() {
        let field = Field::member("config").with_context(Property::Member {
            name: "server".to_owned(),
        });
        let value = serde_json::to_value(field).expect("json value");
        let value_as_str = value.as_str().expect("str value");
        assert_eq!(value_as_str, "server.config");
    }
}
