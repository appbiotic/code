//! Types to use in FFI libraries to simplify error handling.

use std::ffi::{c_void, CString};

use safer_ffi::prelude::*;

use crate::{code, Error};

#[derive_ReprC]
#[repr(i32)]
pub enum AppbioticErrorCode {
    Ok = code::OK,
    Cancelled = code::CANCELLED,
    Unknown = code::UNKNOWN,
    InvalidArgument = code::INVALID_ARGUMENT,
    DeadlineExceeded = code::DEADLINE_EXCEEDED,
    NotFound = code::NOT_FOUND,
    AlreadyExists = code::ALREADY_EXISTS,
    PermissionDenied = code::PERMISSION_DENIED,
    Unauthenticated = code::UNAUTHENTICATED,
    ResourceExhausted = code::RESOURCE_EXHAUSTED,
    FailedPrecondition = code::FAILED_PRECONDITION,
    Aborted = code::ABORTED,
    OutOfRange = code::OUT_OF_RANGE,
    Unimplemented = code::UNIMPLEMENTED,
    Internal = code::INTERNAL,
    Unavailable = code::UNAVAILABLE,
    DataLoss = code::DATA_LOSS,
}

#[derive_ReprC]
#[repr(C)]
pub struct AppbioticStatus {
    code: AppbioticErrorCode,
    message: Option<char_p::Box>,
}

impl AppbioticStatus {
    pub fn ok() -> Self {
        Self {
            code: AppbioticErrorCode::Ok,
            message: None,
        }
    }
}

impl From<Error> for AppbioticStatus {
    fn from(value: Error) -> Self {
        let (code, message) = match value {
            Error::Cancelled(_) => (AppbioticErrorCode::Cancelled, value.to_string()),
            Error::Unknown(_) => (AppbioticErrorCode::Unknown, value.to_string()),
            Error::InvalidArgument(_) => (AppbioticErrorCode::InvalidArgument, value.to_string()),
            Error::DeadlineExceeded(_) => (AppbioticErrorCode::DeadlineExceeded, value.to_string()),
            Error::NotFound(_) => (AppbioticErrorCode::NotFound, value.to_string()),
            Error::AlreadyExists(_) => (AppbioticErrorCode::AlreadyExists, value.to_string()),
            Error::PermissionDenied(_) => (AppbioticErrorCode::PermissionDenied, value.to_string()),
            Error::Unauthenticated(_) => (AppbioticErrorCode::Unauthenticated, value.to_string()),
            Error::ResourceExhausted(_) => {
                (AppbioticErrorCode::ResourceExhausted, value.to_string())
            }
            Error::FailedPrecondition(_) => {
                (AppbioticErrorCode::FailedPrecondition, value.to_string())
            }
            Error::Aborted(_) => (AppbioticErrorCode::Aborted, value.to_string()),
            Error::OutOfRange(_) => (AppbioticErrorCode::OutOfRange, value.to_string()),
            Error::Unimplemented(_) => (AppbioticErrorCode::Unimplemented, value.to_string()),
            Error::Internal(_) => (AppbioticErrorCode::Internal, value.to_string()),
            Error::Unavailable(_) => (AppbioticErrorCode::Unavailable, value.to_string()),
            Error::DataLoss(_) => (AppbioticErrorCode::DataLoss, value.to_string()),
        };
        AppbioticStatus {
            code,
            message: CString::new(message.as_str()).map_or_else(|_| None, |v| Some(v.into())),
        }
    }
}

#[derive_ReprC]
#[repr(C)]
pub struct AppbioticStatusCallback {
    ctx: *mut c_void,
    completion: extern "C" fn(ctx: *mut c_void, status: AppbioticStatus),
}

unsafe impl Send for AppbioticStatusCallback {}

impl AppbioticStatusCallback {
    /// Wrapper for `completion` to work around moving split parameters that
    /// causes Rust to detect the un-sendable `*mut c_void` when this callback
    /// needs to be run in another thread.
    pub fn on_complete(&self, status: AppbioticStatus) {
        (self.completion)(self.ctx, status);
    }
}
