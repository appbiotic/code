#ifndef APPBIOTIC_CODE_FFI
#define APPBIOTIC_CODE_FFI

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A raw ptr byte slice for ffi that cleans up its memory when dropped.
 */
typedef struct AppbioticCodeFfi_OwnedVec {
  uint8_t *data;
  uintptr_t len;
} AppbioticCodeFfi_OwnedVec;

/**
 * A raw ptr string for ffi.
 */
typedef struct AppbioticCodeFfi_String {
  char *bytes;
} AppbioticCodeFfi_String;

/**
 * A raw ptr byte slice for ffi which does not try to delete memory when
 * dropped.
 */
typedef struct AppbioticCodeFfi_ReferencedVec {
  uint8_t *data;
  uintptr_t len;
} AppbioticCodeFfi_ReferencedVec;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Frees the memory of a [`AppbioticCodeFfi_OwnedVec`] pointer.
 *
 * # Safety
 *
 * Undefined behavior if pointer is not for the correct type.
 */
void AppbioticCodeFfi_OwnedVec_Drop(struct AppbioticCodeFfi_OwnedVec *ptr);

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
