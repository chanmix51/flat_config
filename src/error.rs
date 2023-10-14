use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ConfigError {
    /// Configuration setting named is missing.
    Missing { field_name: String, fields: String },

    /// Type mismatch
    TypeMismatch { expected: String, present: String },

    /// The value is incorrect, give a useful context error message (field name, why the value was
    /// wrong or what was expected.
    IncorrectValue(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing { field_name, fields } => write!(f, "CONFIGURATION ERROR: Field '{field_name}' is missing. Available fields are: '{fields}'."),
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
