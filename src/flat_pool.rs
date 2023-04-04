use std::collections::HashMap;

use super::{ConfigError, FlatValue};

/// ## Configuration Settings Pool
///
/// This contains flat values.
#[derive(Debug, Default)]
pub struct FlatPool {
    settings: HashMap<String, FlatValue>,
}

impl FlatPool {
    /// Add or replace a value in the pool.
    pub fn add(&mut self, name: &str, value: FlatValue) -> &mut Self {
        self.settings.insert(name.to_string(), value);

        self
    }

    /// Read a value from the pool if present.
    pub fn get(&self, name: &str) -> Option<&FlatValue> {
        self.settings.get(name)
    }

    /// Require the field to be present or return an error. This will prevent the builder to create
    /// the configuration. The returned value is cloned from the original in the pool.
    pub fn require(&self, name: &str) -> Result<FlatValue, ConfigError> {
        self.get(name)
            .cloned()
            .ok_or_else(|| ConfigError::Missing(name.to_string()))
    }

    /// Like require but panic when not present. Use this method when you know the given field is
    /// present (mostly because it has already been checked by another library like Clap). This
    /// means panicking should never happen.
    pub fn unwrap(&self, name: &str) -> FlatValue {
        self.require(name)
            .map_err(|e| panic!("This should never happen: {e}"))
            .unwrap()
    }

    /// Get a value from the pool, if not present it returns the provided default value.
    pub fn get_or(&self, name: &str, default: FlatValue) -> FlatValue {
        self.get(name).cloned().unwrap_or(default)
    }

    /// Is this field present in the pool?
    pub fn has(&self, name: &str) -> bool {
        self.settings.contains_key(name)
    }
}

#[cfg(test)]
mod tests {}
