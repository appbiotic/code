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

#[derive(Clone, Debug, IntoStaticStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    /// Internal errors. This means that some invariants expected by the
    /// underlying system have been broken. This error code is reserved for
    /// serious errors.
    Internal(ErrorStatus),
    /// Unknown error. For example, this error may be returned when a Status
    /// value received from another address space belongs to an error space
    /// that is not known in this address space. Also errors raised by APIs
    /// that do not return enough error information may be converted to this
    /// error.
    Unknown(ErrorStatus),
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
        }
    }

    // TODO: Build macros to automate building of the error helper functions.

    pub fn internal<S: AsRef<str>>(message: S) -> Error {
        Error::Internal(ErrorStatus::default().with_message(message))
    }

    pub fn unknown<S: AsRef<str>>(message: S) -> Error {
        Error::Unknown(ErrorStatus::default().with_message(message))
    }

    /// Appends a `ErrorDetails::DebugInfo` with info from `error`.
    pub fn with_error<E: fmt::Display>(self, error: E) -> Error {
        match self {
            Error::Internal(details) => Error::Internal(details.with_error(error)),
            Error::Unknown(details) => Error::Unknown(details.with_error(error)),
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
#[derive(Clone, Debug)]
pub struct ErrorStatus {
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// `details` field in a `ErrorDetails::LocalizedMessage`.
    pub message: Option<String>,
    /// A list of messages that carry the error details.  There is a common set
    /// of message types for APIs to use.    
    pub details: Option<Vec<ErrorDetails>>,
}

impl Default for ErrorStatus {
    fn default() -> Self {
        ErrorStatus {
            message: None,
            details: None,
        }
    }
}

impl ErrorStatus {
    pub fn with_message<M: AsRef<str>>(self, message: M) -> Self {
        ErrorStatus {
            message: Some(message.as_ref().to_owned()),
            details: self.details,
        }
    }

    pub fn with_error<E: fmt::Display>(self, error: E) -> Self {
        let mut details = self.details.unwrap_or_default();
        details.push(ErrorDetails::DebugInfo {
            stack_entries: None,
            detail: Some(error.to_string()),
        });
        ErrorStatus {
            message: self.message,
            details: Some(details),
        }
    }
}

/// The specific details of an error that may be optionally forwarded to an
/// end-user.
///
/// These error detail kinds and documentation have been imported from
/// https://github.com/googleapis/googleapis/blob/f36c65081b19e0758ef5696feca27c7dcee5475e/google/rpc/error_details.proto.
#[derive(Clone, Debug, IntoStaticStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
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
        /// https://www.rfc-editor.org/rfc/bcp/bcp47.txt.
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

#[derive(Clone, Debug)]
pub struct Field {
    path_reversed: Vec<Property>,
}

impl Field {
    pub fn new(property: Property) -> Self {
        Field {
            path_reversed: vec![property],
        }
    }

    pub fn with_context(mut self, context: Property) -> Self {
        self.path_reversed.push(context);
        self
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..self.path_reversed.len()).rev() {
            write!(
                f,
                r#"{}"#,
                self.path_reversed.get(i).ok_or_else(|| fmt::Error)?,
            )?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let error = Error::internal("Something bad happened").with_error("Invalid operation");
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
}
