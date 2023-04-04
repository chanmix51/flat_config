use std::collections::HashMap;

use super::{config_setting::ConfigSetting, error::ConfigError};

/// ## Configuration Settings Pool
///
/// This contains flat values.
#[derive(Debug, Default)]
pub struct ConfigSettingPool {
    settings: HashMap<String, ConfigSetting>,
}

impl ConfigSettingPool {
    /// Add or replace a value in the pool.
    pub fn add(&mut self, name: &str, value: ConfigSetting) -> &mut Self {
        self.settings.insert(name.to_string(), value);

        self
    }

    /// Read a value from the pool if present.
    pub fn get(&self, name: &str) -> Option<&ConfigSetting> {
        self.settings.get(name)
    }

    /// Require the field to be present or return an error. This will prevent the builder to create
    /// the configuration. The returned value is cloned from the original in the pool.
    pub fn require(&self, name: &str) -> Result<ConfigSetting, ConfigError> {
        self.get(name)
            .cloned()
            .ok_or_else(|| ConfigError::Missing(name.to_string()))
    }

    /// Like require but panic when not present. Use this method when you know the given field is
    /// present (mostly because it has already been checked by another library like Clap). This
    /// means panicking should never happen.
    pub fn unwrap(&self, name: &str) -> ConfigSetting {
        self.require(name)
            .map_err(|e| panic!("This should never happen: {e}"))
            .unwrap()
    }

    /// Get a value from the pool, if not present it returns the provided default value.
    pub fn get_or(&self, name: &str, default: ConfigSetting) -> ConfigSetting {
        self.get(name).cloned().unwrap_or(default)
    }

    /// Is this field present in the pool?
    pub fn has(&self, name: &str) -> bool {
        self.settings.contains_key(name)
    }
}

/// ### Configuration builder trait.
///
/// Implementations of this trait create configurations reading data from the given
/// [ConfigSettingPool].
///
/// ```rust
/// use flat_config::{
///    config_setting::{TryUnwrap, ConfigSetting},
///    error::ConfigError,
///    setting_pool::{ConfigBuilder, ConfigSettingPool},
/// };
///
///
/// pub struct MyConfig {
///   setting_a: isize,
///   setting_b: String,
///   setting_c: Option<bool>,
/// }
///
/// pub struct MyConfigBuilder;
///
/// impl ConfigBuilder<MyConfig> for MyConfigBuilder {
///     fn build(&self, config_pool: &ConfigSettingPool) -> Result<MyConfig, ConfigError> {
///         let setting_a: isize = config_pool.require("setting_a")?.try_unwrap()?;
///         let setting_b: String = config_pool
///             .get_or("setting_b", ConfigSetting::Text("something".to_string()))
///             .try_unwrap()?;
///         let setting_c: Option<bool> = config_pool.get("setting_c").map(|v| v.try_unwrap().unwrap());
///
///         Ok(MyConfig { setting_a, setting_b, setting_c })
///     }
/// }
/// ```
pub trait ConfigBuilder<T> {
    fn build(&self, config_pool: &ConfigSettingPool) -> Result<T, ConfigError>;
}
