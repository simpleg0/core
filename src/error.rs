use std::fmt::{Debug, Display, Formatter};

use crate::error_kind::SERIALIZATION_FAILURE;

#[macro_export]
macro_rules! ok_or_return_error {
    ($statement: expr, $error_kind: expr, $error_message: expr) => {
        match $statement {
            Ok(value) => value,
            Err(error) => {
                return Err(Error::new(
                    $error_kind,
                    format!("{}: {}", $error_message, error),
                ))
            }
        }
    };
}

#[macro_export]
macro_rules! some_or_return_error {
    ($statement: expr, $error_kind: expr, $error_message: expr) => {
        match $statement {
            Some(value) => value,
            None => return Err(Error::new($error_kind, $error_message)),
        }
    };
}

pub struct Error {
    kind: String,
    message: String,
}

impl Error {
    pub fn new(kind: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            message: message.into(),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::new(SERIALIZATION_FAILURE, error.to_string())
    }
}
