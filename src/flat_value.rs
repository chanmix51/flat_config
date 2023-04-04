use crate::ConfigError;

#[derive(Debug, Clone)]
pub enum FlatValue {
    Integer(isize),
    Text(String),
    Boolean(bool),
}

impl FlatValue {
    fn display(&self) -> String {
        let subtype: &str = match self {
            Self::Integer(_) => "integer",
            Self::Text(_) => "text",
            Self::Boolean(_) => "boolean",
        };

        subtype.to_string()
    }
}

pub trait TryUnwrap<T> {
    fn try_unwrap(&self) -> Result<T, ConfigError>;
}

impl TryUnwrap<isize> for FlatValue {
    fn try_unwrap(&self) -> Result<isize, ConfigError> {
        match self {
            Self::Integer(i) => Ok(*i),
            _ => Err(ConfigError::TypeMismatch {
                expected: "integer".to_string(),
                present: self.display(),
            }),
        }
    }
}

impl TryUnwrap<String> for FlatValue {
    fn try_unwrap(&self) -> Result<String, ConfigError> {
        match self {
            Self::Text(t) => Ok(t.to_string()),
            _ => Err(ConfigError::TypeMismatch {
                expected: "text".to_string(),
                present: self.display(),
            }),
        }
    }
}

impl TryUnwrap<bool> for FlatValue {
    fn try_unwrap(&self) -> Result<bool, ConfigError> {
        match self {
            Self::Boolean(b) => Ok(*b),
            _ => Err(ConfigError::TypeMismatch {
                expected: "boolean".to_string(),
                present: self.display(),
            }),
        }
    }
}

impl From<isize> for FlatValue {
    fn from(value: isize) -> Self {
        Self::Integer(value)
    }
}

impl From<&str> for FlatValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<bool> for FlatValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // FlatValue has to implement From<isize>
    #[test]
    fn from_isize() {
        let value: FlatValue = 2_isize.into();

        assert!(matches!(value, FlatValue::Integer(v) if v == 2));
    }

    // FlatValue has to implement From<&str>
    #[test]
    fn from_str() {
        let value: FlatValue = "whatever".into();

        assert!(matches!(value, FlatValue::Text(v) if v == "whatever".to_string()));
    }

    // FlatValue has to implement From<bool>
    #[test]
    fn from_bool() {
        let value: FlatValue = true.into();

        assert!(matches!(value, FlatValue::Boolean(v) if v));
    }

    // TryUnwrap for FlatValue::Integer
    #[test]
    fn try_unwrap_isize() {
        let value: FlatValue = 2_isize.into();

        assert_eq!(2_isize, value.try_unwrap().unwrap())
    }

    // TryUnwrap for FlatValue::Text
    #[test]
    fn try_unwrap_text() {
        let value: FlatValue = "whatever".into();
        let original: String = value.try_unwrap().unwrap();

        assert_eq!("whatever".to_string(), original)
    }

    // TryUnwrap for FlatValue::Boolean
    #[test]
    fn try_unwrap_boo() {
        let value: FlatValue = true.into();

        let original: bool = value.try_unwrap().unwrap();

        assert!(original)
    }
}
