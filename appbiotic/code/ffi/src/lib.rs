#![allow(non_camel_case_types)]

use std::ffi::{c_char, CString};

use tracing::{event, Level};

/// A raw ptr byte slice for ffi that cleans up its memory when dropped.
#[repr(C)]
pub struct AppbioticCodeFfi_OwnedVec {
    pub data: *mut u8,
    pub len: usize,
}

impl Default for AppbioticCodeFfi_OwnedVec {
    fn default() -> Self {
        Self {
            data: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<u8>> for AppbioticCodeFfi_OwnedVec {
    fn from(value: Vec<u8>) -> Self {
        let mut value = value.into_boxed_slice();
        let result = Self {
            data: value.as_mut_ptr(),
            len: value.len(),
        };
        std::mem::forget(value);
        result
    }
}

impl Drop for AppbioticCodeFfi_OwnedVec {
    fn drop(&mut self) {
        event!(
            Level::TRACE,
            "data.is_null" = self.data.is_null(),
            "AppbioticCodeFfi_OwnedVec::drop"
        );
        if !self.data.is_null() {
            drop(unsafe { Box::from_raw(self.data) });
        }
    }
}

/// Frees the memory of a [`AppbioticCodeFfi_OwnedVec`] pointer.
///
/// # Safety
///
/// Undefined behavior if pointer is not for the correct type.
#[no_mangle]
pub unsafe extern "C" fn AppbioticCodeFfi_OwnedVec_Drop(ptr: *mut AppbioticCodeFfi_OwnedVec) {
    event!(
        Level::TRACE,
        "ptr.is_null" = ptr.is_null(),
        "AppbioticCodeFfi_OwnedVec_Drop"
    );
    if !ptr.is_null() {
        drop(unsafe { Box::from_raw(ptr) })
    }
}

/// A raw ptr byte slice for ffi which does not try to delete memory when
/// dropped.
#[repr(C)]
pub struct AppbioticCodeFfi_ReferencedVec {
    pub data: *mut u8,
    pub len: usize,
}

impl Default for AppbioticCodeFfi_ReferencedVec {
    fn default() -> Self {
        Self {
            data: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<u8>> for AppbioticCodeFfi_ReferencedVec {
    fn from(value: Vec<u8>) -> Self {
        let mut value = value.into_boxed_slice();
        let result = Self {
            data: value.as_mut_ptr(),
            len: value.len(),
        };
        std::mem::forget(value);
        result
    }
}

/// A raw ptr string for ffi.
#[repr(C)]
pub struct AppbioticCodeFfi_String {
    pub bytes: *mut c_char,
}

impl Default for AppbioticCodeFfi_String {
    fn default() -> Self {
        Self {
            bytes: std::ptr::null_mut(),
        }
    }
}

impl Drop for AppbioticCodeFfi_String {
    fn drop(&mut self) {
        event!(
            Level::TRACE,
            "bytes.is_null" = self.bytes.is_null(),
            "AppbioticCodeFfi_String::drop"
        );
        if !self.bytes.is_null() {
            drop(unsafe { Box::from_raw(self.bytes) })
        }
    }
}

impl From<String> for AppbioticCodeFfi_String {
    fn from(value: String) -> Self {
        Self {
            bytes: match CString::new(value) {
                Ok(value) => value.into_raw(),
                Err(error) => {
                    event!(
                        Level::ERROR,
                        error = error.to_string(),
                        "AppbioticCodeFfi_String::from(String)"
                    );
                    std::ptr::null_mut()
                }
            },
        }
    }
}

/// Frees the memory of a [`AppbioticCodeFfi_String`] pointer.
///
/// # Safety
///
/// Undefined behavior if pointer is not for the correct type.
#[no_mangle]
pub unsafe extern "C" fn AppbioticCodeFfi_String_drop(ptr: *mut AppbioticCodeFfi_String) {
    event!(
        Level::TRACE,
        "ptr.is_null" = ptr.is_null(),
        "AppbioticCodeFfi_String_drop"
    );
    if !ptr.is_null() {
        drop(unsafe { Box::from_raw(ptr) })
    }
}
