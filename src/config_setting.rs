use crate::error::ConfigError;

#[derive(Debug, Clone)]
pub enum ConfigSetting {
    Integer(isize),
    Text(String),
    Boolean(bool),
}

impl ConfigSetting {
    fn display(&self) -> String {
        let subtype: &str = match self {
            Self::Integer(_) => "integer",
            Self::Text(_) => "text",
            Self::Boolean(_) => "boolean",
        };

        subtype.to_string()
    }

    pub fn try_unwrap_integer(&self) -> Result<isize, ConfigError> {
        match self {
            Self::Integer(i) => Ok(*i),
            _ => Err(ConfigError::TypeMismatch {
                expected: "integer".to_string(),
                present: self.display(),
            }),
        }
    }

    pub fn try_unwrap_text(&self) -> Result<String, ConfigError> {
        match self {
            Self::Text(t) => Ok(t.to_string()),
            _ => Err(ConfigError::TypeMismatch {
                expected: "text".to_string(),
                present: self.display(),
            }),
        }
    }

    pub fn try_unwrap_bool(&self) -> Result<bool, ConfigError> {
        match self {
            Self::Boolean(b) => Ok(*b),
            _ => Err(ConfigError::TypeMismatch {
                expected: "boolean".to_string(),
                present: self.display(),
            }),
        }
    }
}

impl From<isize> for ConfigSetting {
    fn from(value: isize) -> Self {
        Self::Integer(value)
    }
}

impl From<&str> for ConfigSetting {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<bool> for ConfigSetting {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
