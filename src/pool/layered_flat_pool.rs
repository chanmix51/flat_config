use crate::{ConfigError, FlatValue};

use super::FlatPool;

type Layer = Box<dyn FlatPool>;

/// ## LayeredFlatPool
///
/// This flat pool can take several sources and use a path to determine which data override others.
#[derive(Debug)]
pub struct LayeredFlatPool {
    layers: Vec<Layer>,
}

impl LayeredFlatPool {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }
}

impl FlatPool for LayeredFlatPool {
    fn get(&self, name: &str) -> Option<FlatValue> {
        self.get_borrow(name).cloned()
    }

    fn has(&self, name: &str) -> bool {
        self.get_borrow(name).is_some()
    }

    fn unwrap(&self, name: &str) -> FlatValue {
        self.get(name).unwrap()
    }

    fn get_or(&self, name: &str, default: FlatValue) -> FlatValue {
        self.get(name).unwrap_or(default)
    }

    fn require(&self, name: &str) -> Result<FlatValue, ConfigError> {
        self.get(name)
            .ok_or_else(|| ConfigError::Missing(name.to_string()))
    }

    fn get_borrow(&self, name: &str) -> Option<&FlatValue> {
        self.layers
            .iter()
            .rev()
            .find_map(|layer| layer.get_borrow(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::SimpleFlatPool;

    fn init_pools() -> (SimpleFlatPool, SimpleFlatPool) {
        let mut pool1 = SimpleFlatPool::default();
        pool1.add("setting_1", 9.into());
        pool1.add("shared_setting", "first".into());
        let mut pool2 = SimpleFlatPool::default();
        pool2.add("shared_setting", "second".into());
        pool2.add("setting_2", true.into());

        (pool1, pool2)
    }

    #[test]
    fn get_both() {
        let (pool1, pool2) = init_pools();
        let pool = LayeredFlatPool::new(vec![Box::new(pool1), Box::new(pool2)]);
        let value = pool.get_borrow("shared_setting").unwrap();

        assert_eq!(&FlatValue::Text("second".to_string()), value);
    }

    #[test]
    fn get_last() {
        let (pool1, pool2) = init_pools();
        let pool = LayeredFlatPool::new(vec![Box::new(pool1), Box::new(pool2)]);
        let value = pool.get_borrow("setting_1").unwrap();

        assert_eq!(&FlatValue::Integer(9), value);
    }

    #[test]
    fn get_first() {
        let (pool1, pool2) = init_pools();
        let pool = LayeredFlatPool::new(vec![Box::new(pool1), Box::new(pool2)]);
        let value = pool.get_borrow("setting_2").unwrap();

        assert_eq!(&FlatValue::Boolean(true), value);
    }

    #[test]
    fn get_unexistent() {
        let (pool1, pool2) = init_pools();
        let pool = LayeredFlatPool::new(vec![Box::new(pool1), Box::new(pool2)]);
        let value = pool.get_borrow("unexistent");

        assert_eq!(None, value);
    }

    #[test]
    fn has_parameter() {
        let (pool1, pool2) = init_pools();
        let pool = LayeredFlatPool::new(vec![Box::new(pool1), Box::new(pool2)]);

        assert!(pool.has("shared_setting"));
        assert!(pool.has("setting_1"));
        assert!(pool.has("setting_2"));
    }

    #[test]
    fn get_instance() {
        let (pool1, pool2) = init_pools();
        let pool = LayeredFlatPool::new(vec![Box::new(pool1), Box::new(pool2)]);

        let value = pool.get("shared_setting").unwrap();
        assert_eq!(FlatValue::Text("second".to_string()), value);

        let value = pool.get("setting_1").unwrap();
        assert_eq!(FlatValue::Integer(9), value);

        let value = pool.get("setting_2").unwrap();
        assert_eq!(FlatValue::Boolean(true), value);

        let value = pool.get("unexistent");
        assert_eq!(None, value);
    }

    #[test]
    fn get_unwrap() {
        let (pool1, pool2) = init_pools();
        let pool = LayeredFlatPool::new(vec![Box::new(pool1), Box::new(pool2)]);

        let value = pool.unwrap("shared_setting");
        assert_eq!(FlatValue::Text("second".to_string()), value);

        let value = pool.unwrap("setting_1");
        assert_eq!(FlatValue::Integer(9), value);

        let value = pool.unwrap("setting_2");
        assert_eq!(FlatValue::Boolean(true), value);
    }

    #[test]
    #[should_panic]
    fn get_unwrap_fail() {
        let (pool1, pool2) = init_pools();
        let pool = LayeredFlatPool::new(vec![Box::new(pool1), Box::new(pool2)]);

        let _value = pool.unwrap("unexistent");
    }

    #[test]
    fn get_or() {
        let (pool1, pool2) = init_pools();
        let pool = LayeredFlatPool::new(vec![Box::new(pool1), Box::new(pool2)]);

        let value = pool.get_or("shared_setting", "default".into());
        assert_eq!(FlatValue::Text("second".to_string()), value);

        let value = pool.get_or("setting_1", 0.into());
        assert_eq!(FlatValue::Integer(9), value);

        let value = pool.get_or("setting_2", false.into());
        assert_eq!(FlatValue::Boolean(true), value);

        let value = pool.get_or("unexistent", 42.into());
        assert_eq!(FlatValue::Integer(42), value);
    }

    #[test]
    fn require() {
        let (pool1, pool2) = init_pools();
        let pool = LayeredFlatPool::new(vec![Box::new(pool1), Box::new(pool2)]);

        let value = pool.require("shared_setting").unwrap();
        assert_eq!(FlatValue::Text("second".to_string()), value);

        let value = pool.require("setting_1").unwrap();
        assert_eq!(FlatValue::Integer(9), value);

        let value = pool.require("setting_2").unwrap();
        assert_eq!(FlatValue::Boolean(true), value);

        let _value = pool.require("unexistent").unwrap_err();
    }
}
