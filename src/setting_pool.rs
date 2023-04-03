use std::collections::HashMap;

use super::{config_setting::ConfigSetting, error::ConfigError};

#[derive(Debug, Default)]
pub struct ConfigSettingPool {
    settings: HashMap<String, ConfigSetting>,
}

impl ConfigSettingPool {
    pub fn add(&mut self, name: &str, value: ConfigSetting) -> &mut Self {
        self.settings.insert(name.to_string(), value);

        self
    }

    pub fn get(&self, name: &str) -> Option<&ConfigSetting> {
        self.settings.get(name)
    }

    pub fn require(&self, name: &str) -> Result<ConfigSetting, ConfigError> {
        self.get(name)
            .cloned()
            .ok_or_else(|| ConfigError::Missing(name.to_string()))
    }

    pub fn get_or(&self, name: &str, default: ConfigSetting) -> ConfigSetting {
        self.get(name).cloned().unwrap_or(default)
    }

    pub fn has(&self, name: &str) -> bool {
        self.settings.contains_key(name)
    }
}

pub trait ConfigBuilder<T> {
    fn build(&self, config_pool: &ConfigSettingPool) -> Result<T, ConfigError>;
}
