#ifndef APPBIOTIC_CODE_ERROR
#define APPBIOTIC_CODE_ERROR

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <CoreFoundation/CFAvailability.h>

typedef CF_ENUM(int32_t, appbiotic_code_error_Code) {
  /**
   * Not an error; returned on success.
   */
  appbiotic_code_error_Code_Ok = 0,
  /**
   * The operation was cancelled, typically by the caller.
   */
  appbiotic_code_error_Code_Cancelled = 1,
  /**
   * Unknown error.
   */
  appbiotic_code_error_Code_Unknown = 2,
  /**
   * The client specified an invalid argument.
   */
  appbiotic_code_error_Code_InvalidArgument = 3,
  /**
   * The deadline expired before the operation could complete.
   */
  appbiotic_code_error_Code_DeadlineExceeded = 4,
  /**
   * Some requested entity (e.g., file or directory) was not found.
   */
  appbiotic_code_error_Code_NotFound = 5,
  /**
   * The entity that a client attempted to create (e.g., file or directory)
   * already exists.
   */
  appbiotic_code_error_Code_AlreadyExists = 6,
  /**
   * The caller does not have permission to execute the specified
   * operation.
   */
  appbiotic_code_error_Code_PermissionDenied = 7,
  /**
   * Some resource has been exhausted, perhaps a per-user quota, or
   * perhaps the entire file system is out of space.
   */
  appbiotic_code_error_Code_ResourceExhausted = 8,
  /**
   * The operation was rejected because the system is not in a state
   * required for the operation's execution.
   */
  appbiotic_code_error_Code_FailedPrecondition = 9,
  /**
   * The operation was aborted, typically due to a concurrency issue such as
   * a sequencer check failure or transaction abort.
   */
  appbiotic_code_error_Code_Aborted = 10,
  /**
   * The operation was attempted past the valid range.
   */
  appbiotic_code_error_Code_OutOfRange = 11,
  /**
   * The operation is not implemented or is not supported/enabled in this
   * service.
   */
  appbiotic_code_error_Code_Unimplemented = 12,
  /**
   * Internal errors.
   */
  appbiotic_code_error_Code_Internal = 13,
  /**
   * The service is currently unavailable.
   */
  appbiotic_code_error_Code_Unavailable = 14,
  /**
   * Unrecoverable data loss or corruption.
   */
  appbiotic_code_error_Code_DataLoss = 15,
  /**
   * The request does not have valid authentication credentials for the
   * operation.
   */
  appbiotic_code_error_Code_Unauthenticated = 16,
};


#endif /* APPBIOTIC_CODE_ERROR */
