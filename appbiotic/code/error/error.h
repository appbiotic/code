#ifndef APPBIOTIC_CODE_ERROR
#define APPBIOTIC_CODE_ERROR

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <CoreFoundation/CoreFoundation.h>
#include "appbiotic/code/ffi/ffi.h"

typedef CF_ENUM(int32_t, AppbioticErrorCode) {
  /**
   * Not an error; returned on success.
   */
  AppbioticErrorCode_Ok = 0,
  /**
   * The operation was cancelled, typically by the caller.
   */
  AppbioticErrorCode_Cancelled = 1,
  /**
   * Unknown error.
   */
  AppbioticErrorCode_Unknown = 2,
  /**
   * The client specified an invalid argument.
   */
  AppbioticErrorCode_InvalidArgument = 3,
  /**
   * The deadline expired before the operation could complete.
   */
  AppbioticErrorCode_DeadlineExceeded = 4,
  /**
   * Some requested entity (e.g., file or directory) was not found.
   */
  AppbioticErrorCode_NotFound = 5,
  /**
   * The entity that a client attempted to create (e.g., file or directory)
   * already exists.
   */
  AppbioticErrorCode_AlreadyExists = 6,
  /**
   * The caller does not have permission to execute the specified
   * operation.
   */
  AppbioticErrorCode_PermissionDenied = 7,
  /**
   * Some resource has been exhausted, perhaps a per-user quota, or
   * perhaps the entire file system is out of space.
   */
  AppbioticErrorCode_ResourceExhausted = 8,
  /**
   * The operation was rejected because the system is not in a state
   * required for the operation's execution.
   */
  AppbioticErrorCode_FailedPrecondition = 9,
  /**
   * The operation was aborted, typically due to a concurrency issue such as
   * a sequencer check failure or transaction abort.
   */
  AppbioticErrorCode_Aborted = 10,
  /**
   * The operation was attempted past the valid range.
   */
  AppbioticErrorCode_OutOfRange = 11,
  /**
   * The operation is not implemented or is not supported/enabled in this
   * service.
   */
  AppbioticErrorCode_Unimplemented = 12,
  /**
   * Internal errors.
   */
  AppbioticErrorCode_Internal = 13,
  /**
   * The service is currently unavailable.
   */
  AppbioticErrorCode_Unavailable = 14,
  /**
   * Unrecoverable data loss or corruption.
   */
  AppbioticErrorCode_DataLoss = 15,
  /**
   * The request does not have valid authentication credentials for the
   * operation.
   */
  AppbioticErrorCode_Unauthenticated = 16,
};


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
  AppbioticCodeFfi_OwnedVec response;
  /**
   * The status of the result, i.e., whether successful or not.
   */
  struct AppbioticCodeError_Status status;
} AppbioticCodeError_Result;

typedef struct AppbioticCodeError_ResultCallback {
  /**
   * Opaque context pointer used to restore state within completion.
   */
  void *ctx;
  /**
   * Function called upon completion of the operation with the results in
   * `result`, an unowned reference to the result.
   */
  void (*on_result)(void *ctx, struct AppbioticCodeError_Result result);
} AppbioticCodeError_ResultCallback;

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

#endif /* APPBIOTIC_CODE_ERROR */
