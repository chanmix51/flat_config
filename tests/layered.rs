use std::path::PathBuf;

use flat_config::{
    pool::{FlatPool, LayeredFlatPool, SimpleFlatPool},
    ConfigBuilder, ConfigError, TryUnwrap,
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct AppConfiguration {
    app_name: String,
    database_dir: PathBuf,
    start_epoch: usize,
    dry_run: bool,
}

#[derive(Default)]
struct AppConfigBuilder;

impl ConfigBuilder<AppConfiguration> for AppConfigBuilder {
    fn build(&self, config_pool: &impl FlatPool) -> Result<AppConfiguration, ConfigError> {
        let app_name: String = config_pool.unwrap("app_name").try_unwrap()?;
        let database_dir: String = config_pool.unwrap("database_dir").try_unwrap()?;
        let start_epoch: isize = config_pool.unwrap("start_epoch").try_unwrap()?;
        let dry_run: bool = config_pool.get_or("dry_run", false.into()).try_unwrap()?;

        let config = AppConfiguration {
            app_name,
            database_dir: PathBuf::new().join(&database_dir),
            start_epoch: usize::try_from(start_epoch).map_err(|e| {
                ConfigError::IncorrectValue(format!(
                    "Could not cast start_epoch {start_epoch} as usize. Error: {e}"
                ))
            })?,
            dry_run,
        };

        Ok(config)
    }
}

#[test]
fn build_layered_pool() {
    // This could be the hard coded default config values for the application.
    let mut default_pool = SimpleFlatPool::default();
    default_pool
        .add("database_dir", "/var/database".into())
        .add("start_epoch", 0.into())
        .add("dry_run", false.into());

    // This could be the configuration settings read from a config file
    let mut file_pool = SimpleFlatPool::default();
    file_pool
        .add("database_dir", "/alternate/dir".into())
        .add("app_name", "whatever".into())
        .add("start_epoch", 3.into());

    let config_pool = LayeredFlatPool::new(vec![Box::new(default_pool), Box::new(file_pool)]);
    let config = AppConfigBuilder::default().build(&config_pool).unwrap();

    assert_eq!("whatever".to_string(), config.app_name);
}
