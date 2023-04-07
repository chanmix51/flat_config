use std::collections::HashMap;

use crate::{ConfigError, FlatValue};

use super::FlatPool;

/// ## Configuration Settings Pool
///
/// This contains flat values.
#[derive(Debug, Default)]
pub struct SimpleFlatPool {
    settings: HashMap<String, FlatValue>,
}

impl SimpleFlatPool {
    /// Add or replace a value in the pool.
    pub fn add(&mut self, name: &str, value: FlatValue) -> &mut Self {
        self.settings.insert(name.to_string(), value);

        self
    }
}

impl FlatPool for SimpleFlatPool {
    /// Return a cloned value if present.
    fn get(&self, name: &str) -> Option<FlatValue> {
        self.settings.get(name).cloned()
    }

    /// Get a borrowed value from the pool if present.
    fn get_borrow(&self, name: &str) -> Option<&FlatValue> {
        self.settings.get(name)
    }

    /// Require the field to be present or return an error. This will prevent the builder to create
    /// the configuration. The returned value is cloned from the original in the pool.
    fn require(&self, name: &str) -> Result<FlatValue, ConfigError> {
        self.get(name)
            .ok_or_else(|| ConfigError::Missing(name.to_string()))
    }

    /// Like require but panic when not present. Use this method when you know the given field is
    /// present (mostly because it has already been checked by another library like Clap). This
    /// means panicking should never happen.
    fn unwrap(&self, name: &str) -> FlatValue {
        self.require(name)
            .map_err(|e| panic!("This should never happen: {e}"))
            .unwrap()
    }

    /// Get a value from the pool, if not present it returns the provided default value.
    fn get_or(&self, name: &str, default: FlatValue) -> FlatValue {
        self.get(name).unwrap_or(default)
    }

    /// Is this field present in the pool?
    fn has(&self, name: &str) -> bool {
        self.settings.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_settings() {
        let mut pool = SimpleFlatPool::default();
        pool.add("thing_a", "thing_a".into())
            .add("how_much", 2.into())
            .add("is_real", true.into());

        assert_eq!(3, pool.settings.len());
        assert_eq!(
            &FlatValue::Text("thing_a".to_string()),
            pool.settings.get("thing_a").unwrap()
        );
        assert_eq!(
            &FlatValue::Integer(2_isize),
            pool.settings.get("how_much").unwrap()
        );
        assert_eq!(
            &FlatValue::Boolean(true),
            pool.settings.get("is_real").unwrap()
        )
    }

    #[test]
    fn get_borrow() {
        let mut pool = SimpleFlatPool::default();
        pool.add("thing_a", "thing_a".into())
            .add("how_much", 2.into())
            .add("is_real", true.into());

        assert_eq!(
            Some(&FlatValue::Text("thing_a".to_string())),
            pool.get_borrow("thing_a")
        );
        assert_eq!(
            Some(&FlatValue::Integer(2_isize)),
            pool.get_borrow("how_much")
        );
        assert_eq!(Some(&FlatValue::Boolean(true)), pool.get_borrow("is_real"));
        assert_eq!(None, pool.get_borrow("unexistent"));
    }

    #[test]
    fn get_settings() {
        let mut pool = SimpleFlatPool::default();
        pool.add("thing_a", "thing_a".into())
            .add("how_much", 2.into())
            .add("is_real", true.into());

        assert_eq!(
            Some(FlatValue::Text("thing_a".to_string())),
            pool.get("thing_a")
        );
        assert_eq!(Some(FlatValue::Integer(2_isize)), pool.get("how_much"));
        assert_eq!(Some(FlatValue::Boolean(true)), pool.get("is_real"));
        assert_eq!(None, pool.get("unexistent"));
    }

    #[test]
    fn get_or_default() {
        let mut pool = SimpleFlatPool::default();
        pool.add("thing_a", "thing_a".into())
            .add("how_much", 2.into())
            .add("is_real", true.into());

        assert_eq!(
            FlatValue::Text("thing_a".to_string()),
            pool.get_or("thing_a", "nope".into())
        );
        assert_eq!(
            FlatValue::Integer(2_isize),
            pool.get_or("how_much", 0.into())
        );
        assert_eq!(
            FlatValue::Boolean(true),
            pool.get_or("is_real", false.into())
        );
        assert_eq!(
            FlatValue::Text("existing".to_string()),
            pool.get_or("unexistent", "existing".into())
        );
    }

    #[test]
    fn require() {
        let mut pool = SimpleFlatPool::default();
        pool.add("thing_a", "thing_a".into())
            .add("how_much", 2.into())
            .add("is_real", true.into());

        let result = pool.require("thing_a");
        assert!(result.is_ok());
        assert_eq!(FlatValue::Text("thing_a".to_string()), result.unwrap());

        let result = pool.require("unexistent");
        assert!(result.is_err());
    }

    #[test]
    fn unwrap_ok() {
        let mut pool = SimpleFlatPool::default();
        pool.add("thing_a", "thing_a".into())
            .add("how_much", 2.into())
            .add("is_real", true.into());

        assert_eq!(
            FlatValue::Text("thing_a".to_string()),
            pool.unwrap("thing_a")
        );
    }

    #[test]
    #[should_panic]
    fn unwrap_panic() {
        let mut pool = SimpleFlatPool::default();
        pool.add("thing_a", "thing_a".into())
            .add("how_much", 2.into())
            .add("is_real", true.into());

        pool.unwrap("unexistent");
    }

    #[test]
    fn exist() {
        let mut pool = SimpleFlatPool::default();
        pool.add("thing_a", "thing_a".into())
            .add("how_much", 2.into())
            .add("is_real", true.into());

        assert!(pool.has("thing_a"));
        assert!(!pool.has("unexistent"));
    }
}
