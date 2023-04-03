use std::path::PathBuf;

use flat_config::{
    error::ConfigError,
    setting_pool::{ConfigBuilder, ConfigSettingPool},
};

#[derive(Debug, Default, Clone)]
struct AppConfiguration {
    environment: String,
    database_dir: PathBuf,
    verbose_level: usize,
    dry_run: bool,
}

#[derive(Default)]
struct AppConfigBuilder;

impl ConfigBuilder<AppConfiguration> for AppConfigBuilder {
    fn build(&self, config_pool: &ConfigSettingPool) -> Result<AppConfiguration, ConfigError> {
        let environment = config_pool.require("environment")?.try_unwrap_text()?;
        let database_dir = config_pool.require("database_dir")?.try_unwrap_text()?;
        let database_dir = PathBuf::new().join(database_dir);

        let verbose_level = config_pool
            .get_or("verbose_level", 0.into())
            .try_unwrap_integer()?;
        let verbose_level = usize::try_from(verbose_level).map_err(|e| {
            ConfigError::IncorrectValue(format!(
                "Verbose level ({verbose_level}) could notbe converted to usize. Error: {e}"
            ))
        })?;
        let dry_run = config_pool
            .get_or("dry_run", false.into())
            .try_unwrap_bool()?;

        let config = AppConfiguration {
            environment,
            database_dir: PathBuf::new().join(database_dir),
            verbose_level,
            dry_run,
        };

        Ok(config)
    }
}

#[test]
fn build() {
    let mut pool = ConfigSettingPool::default();
    pool.add("environment", "production".into())
        .add("database_dir", "/var/database".into())
        .add("verbose_level", 2.into())
        .add("dry_run", true.into());
    let config = AppConfigBuilder::default()
        .build(&pool)
        .map_err(|e| format!("{e}"))
        .unwrap();

    assert_eq!("production".to_string(), config.environment);
    assert_eq!(
        "/var/database".to_string(),
        config.database_dir.display().to_string()
    );
    assert_eq!(2, config.verbose_level);
    assert!(config.dry_run);
}

#[test]
#[should_panic]
fn require() {
    let mut pool = ConfigSettingPool::default();
    pool.add("environment", "production".into())
        .add("verbose_level", 2.into())
        .add("dry_run", true.into());
    let _config = AppConfigBuilder::default()
        .build(&pool)
        .map_err(|e| format!("{e}"))
        .unwrap();
}

#[test]
fn default() {
    let mut pool = ConfigSettingPool::default();
    pool.add("environment", "production".into())
        .add("database_dir", "/var/database".into());
    let config = AppConfigBuilder::default()
        .build(&pool)
        .map_err(|e| format!("{e}"))
        .unwrap();

    assert_eq!(0, config.verbose_level);
    assert!(!config.dry_run);
}
