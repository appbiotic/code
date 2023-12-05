#![allow(non_camel_case_types)]

use appbiotic_code_ffi::{AppbioticCodeFfi_String, AppbioticCodeFfi_Vec};
use tracing::{event, Level};

use crate::{AppbioticErrorCode, Error};

/// The success details of an operation.
#[repr(C)]
pub struct AppbioticCodeError_Status {
    pub code: AppbioticErrorCode,
    pub message: AppbioticCodeFfi_String,
}

impl AppbioticCodeError_Status {
    pub fn ok() -> Self {
        Self {
            code: AppbioticErrorCode::Ok,
            message: AppbioticCodeFfi_String::default(),
        }
    }

    pub fn with_message(self, message: impl AsRef<str>) -> Self {
        Self {
            code: self.code,
            message: message.as_ref().to_owned().into(),
        }
    }
}

impl From<Error> for AppbioticCodeError_Status {
    fn from(value: Error) -> Self {
        Self {
            code: value.code(),
            message: value.to_string().into(),
        }
    }
}

/// Frees the memory of a [`AppbioticCodeError_Status`] pointer.
///
/// # Safety
///
/// Undefined behavior if pointer is not for the correct type.
#[no_mangle]
pub unsafe extern "C" fn AppbioticCodeError_Status_drop(ptr: *mut AppbioticCodeError_Status) {
    event!(
        Level::TRACE,
        "ptr.is_null" = ptr.is_null(),
        "appbiotic_code_ffi_error_Status_drop"
    );
    if !ptr.is_null() {
        drop(unsafe { Box::from_raw(ptr) })
    }
}

/// The result of an operation.
#[repr(C)]
pub struct AppbioticCodeError_Result {
    /// The response from the operation.
    pub response: AppbioticCodeFfi_Vec,
    /// The status of the result, i.e., whether successful or not.
    pub status: AppbioticCodeError_Status,
}

impl AppbioticCodeError_Result {
    pub fn with_message(self, message: impl AsRef<str>) -> Self {
        Self {
            response: self.response,
            status: AppbioticCodeError_Status {
                code: self.status.code,
                message: message.as_ref().to_owned().into(),
            },
        }
    }
}

impl From<Vec<u8>> for AppbioticCodeError_Result {
    fn from(value: Vec<u8>) -> Self {
        let mut data = value.into_boxed_slice();
        let result = Self {
            response: AppbioticCodeFfi_Vec {
                data: data.as_mut_ptr(),
                len: data.len(),
            },
            status: AppbioticCodeError_Status::ok(),
        };
        std::mem::forget(data);
        result
    }
}

impl From<Error> for AppbioticCodeError_Result {
    fn from(value: Error) -> Self {
        Self {
            response: AppbioticCodeFfi_Vec::default(),
            status: value.into(),
        }
    }
}

/// Frees the memory of a [`AppbioticCodeError_Result`] pointer.
///
/// # Safety
///
/// Undefined behavior if pointer is not for the correct type.
#[no_mangle]
pub unsafe extern "C" fn AppbioticCodeError_Result_drop(ptr: *mut AppbioticCodeError_Result) {
    event!(
        Level::TRACE,
        "ptr.is_null" = ptr.is_null(),
        "AppbioticCodeError_Result_drop"
    );
    if !ptr.is_null() {
        drop(unsafe { Box::from_raw(ptr) })
    }
}
