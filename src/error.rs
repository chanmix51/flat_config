use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ConfigError {
    /// Configuration setting named is missing.
    Missing(String),

    /// Type mismatch
    TypeMismatch { expected: String, present: String },

    /// The value is incorrect, give a useful context error message (field name, why the value was
    /// wrong or what was expected.
    IncorrectValue(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing(field) => write!(f, "CONFIGURATION ERROR: Field '{field}' is missing."),
            Self::TypeMismatch { expected, present } => {
                write!(
                    f,
                    "CONFIGURATION ERROR: Type mismatch, expected '{expected}' got '{present}'."
                )
            }
            Self::IncorrectValue(message) => {
                write!(f, "CONFIGURATION ERROR: Incorrect value: {message}.")
            }
        }
    }
}

impl Error for ConfigError {}
