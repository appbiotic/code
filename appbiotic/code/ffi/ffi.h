#ifndef APPBIOTIC_CODE_FFI
#define APPBIOTIC_CODE_FFI

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A raw ptr byte slice for ffi.
 */
typedef struct AppbioticCodeFfi_Vec {
  uint8_t *data;
  uintptr_t len;
} AppbioticCodeFfi_Vec;

/**
 * A raw ptr string for ffi.
 */
typedef struct AppbioticCodeFfi_String {
  char *bytes;
} AppbioticCodeFfi_String;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Frees the memory of a [`AppbioticCodeFfi_Vec`] pointer.
 *
 * # Safety
 *
 * Undefined behavior if pointer is not for the correct type.
 */
void AppbioticCodeFfi_Vec_Drop(struct AppbioticCodeFfi_Vec *ptr);

/**
 * Frees the memory of a [`AppbioticCodeFfi_String`] pointer.
 *
 * # Safety
 *
 * Undefined behavior if pointer is not for the correct type.
 */
void AppbioticCodeFfi_String_drop(struct AppbioticCodeFfi_String *ptr);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* APPBIOTIC_CODE_FFI */
