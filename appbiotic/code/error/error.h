#ifndef APPBIOTIC_CODE_ERROR
#define APPBIOTIC_CODE_ERROR

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "appbiotic/code/ffi/ffi.h"

enum AppbioticErrorCode
#ifdef __cplusplus
  : int32_t
#endif // __cplusplus
 {
  /**
   * Not an error; returned on success.
   */
  Ok = 0,
  /**
   * The operation was cancelled, typically by the caller.
   */
  Cancelled = 1,
  /**
   * Unknown error.
   */
  Unknown = 2,
  /**
   * The client specified an invalid argument.
   */
  InvalidArgument = 3,
  /**
   * The deadline expired before the operation could complete.
   */
  DeadlineExceeded = 4,
  /**
   * Some requested entity (e.g., file or directory) was not found.
   */
  NotFound = 5,
  /**
   * The entity that a client attempted to create (e.g., file or directory)
   * already exists.
   */
  AlreadyExists = 6,
  /**
   * The caller does not have permission to execute the specified
   * operation.
   */
  PermissionDenied = 7,
  /**
   * Some resource has been exhausted, perhaps a per-user quota, or
   * perhaps the entire file system is out of space.
   */
  ResourceExhausted = 8,
  /**
   * The operation was rejected because the system is not in a state
   * required for the operation's execution.
   */
  FailedPrecondition = 9,
  /**
   * The operation was aborted, typically due to a concurrency issue such as
   * a sequencer check failure or transaction abort.
   */
  Aborted = 10,
  /**
   * The operation was attempted past the valid range.
   */
  OutOfRange = 11,
  /**
   * The operation is not implemented or is not supported/enabled in this
   * service.
   */
  Unimplemented = 12,
  /**
   * Internal errors.
   */
  Internal = 13,
  /**
   * The service is currently unavailable.
   */
  Unavailable = 14,
  /**
   * Unrecoverable data loss or corruption.
   */
  DataLoss = 15,
  /**
   * The request does not have valid authentication credentials for the
   * operation.
   */
  Unauthenticated = 16,
};
#ifndef __cplusplus
typedef int32_t AppbioticErrorCode;
#endif // __cplusplus

/**
 * The success details of an operation.
 */
typedef struct AppbioticCodeError_Status {
  AppbioticErrorCode code;
  AppbioticCodeFfi_String message;
} AppbioticCodeError_Status;

/**
 * The result of an operation.
 */
typedef struct AppbioticCodeError_Result {
  /**
   * The response from the operation.
   */
  AppbioticCodeFfi_Vec response;
  /**
   * The status of the result, i.e., whether successful or not.
   */
  struct AppbioticCodeError_Status status;
} AppbioticCodeError_Result;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Frees the memory of a [`AppbioticCodeError_Status`] pointer.
 *
 * # Safety
 *
 * Undefined behavior if pointer is not for the correct type.
 */
void AppbioticCodeError_Status_drop(struct AppbioticCodeError_Status *ptr);

/**
 * Frees the memory of a [`AppbioticCodeError_Result`] pointer.
 *
 * # Safety
 *
 * Undefined behavior if pointer is not for the correct type.
 */
void AppbioticCodeError_Result_drop(struct AppbioticCodeError_Result *ptr);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* APPBIOTIC_CODE_ERROR */
