use crate::{ConfigError, FlatValue};

/// ## FlatPool trait
///
/// A FlatPool is an implementation that can return stored [FlatValue] instances to build
/// configuration containers.
pub trait FlatPool: std::fmt::Debug {
    /// Return a cloned value if present.
    fn get(&self, name: &str) -> Option<FlatValue>;

    /// Get a borrowed value from the pool if present.
    fn get_borrow(&self, name: &str) -> Option<&FlatValue>;

    /// Require the field to be present or return an error. This will prevent the builder to create
    /// the configuration. The returned value is cloned from the original in the pool.
    fn require(&self, name: &str) -> Result<FlatValue, ConfigError>;

    /// Like require but panic when not present. Use this method when you know the given field is
    /// present (mostly because it has already been checked by another library like Clap). This
    /// means panicking should never happen.
    fn unwrap(&self, name: &str) -> FlatValue;

    /// Get a value from the pool, if not present it returns the provided default value.
    fn get_or(&self, name: &str, default: FlatValue) -> FlatValue;

    /// Is this field present in the pool?
    fn has(&self, name: &str) -> bool;
}
