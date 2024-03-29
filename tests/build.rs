use std::path::PathBuf;

use flat_config::{
    pool::{FlatPool, SimpleFlatPool},
    ConfigBuilder, ConfigError, TryUnwrap,
};

#[derive(Debug, Clone, PartialEq)]
pub enum VerboseLevel {
    Critical = 0,
    Error,
    Warning,
    Info,
    Debug,
}

impl TryFrom<isize> for VerboseLevel {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, <Self as TryFrom<isize>>::Error> {
        match value {
            0 => Ok(Self::Critical),
            1 => Ok(Self::Error),
            2 => Ok(Self::Warning),
            3 => Ok(Self::Info),
            v if v >= 4 => Ok(Self::Debug),
            v => Err(format!("invalid verbose level: {v}.")),
        }
    }
}

#[derive(Debug, Clone)]
struct AppConfiguration {
    app_name: String,
    database_dir: PathBuf,
    verbose_level: VerboseLevel,
    dry_run: bool,
}

#[derive(Default)]
struct AppConfigBuilder;

impl ConfigBuilder<AppConfiguration> for AppConfigBuilder {
    fn build(&self, config_pool: &impl FlatPool) -> Result<AppConfiguration, ConfigError> {
        // Application name has been already checked for existence and consistency
        let app_name = config_pool.unwrap("app_name").try_unwrap()?;

        let database_dir: String = config_pool.require("database_dir")?.try_unwrap()?;
        let database_dir = PathBuf::new().join(database_dir);

        let verbose_level: isize = config_pool.get_or("verbose_level", 0.into()).try_unwrap()?;
        let verbose_level = VerboseLevel::try_from(verbose_level)
            .map_err(|_| panic!("Negative verbose_level {verbose_level} shall never occure."))?;

        let dry_run = config_pool.get_or("dry_run", false.into()).try_unwrap()?;

        let config = AppConfiguration {
            app_name,
            database_dir: PathBuf::new().join(database_dir),
            verbose_level,
            dry_run,
        };

        Ok(config)
    }
}

#[test]
fn build() {
    let mut pool = SimpleFlatPool::default();
    pool.add("whatever", "something".into())
        .add("app_name", "Application".into())
        .add("database_dir", "/var/database".into())
        .add("verbose_level", 2.into())
        .add("dry_run", true.into());
    let config = AppConfigBuilder::default()
        .build(&pool)
        .map_err(|e| format!("{e}"))
        .unwrap();

    assert_eq!("Application".to_string(), config.app_name);
    assert_eq!(
        "/var/database".to_string(),
        config.database_dir.display().to_string()
    );
    assert_eq!(VerboseLevel::Warning, config.verbose_level);
    assert!(config.dry_run);
}

#[test]
#[should_panic]
fn require() {
    let mut pool = SimpleFlatPool::default();
    pool.add("verbose_level", 2.into())
        .add("app_name", "Application".into())
        .add("dry_run", true.into());
    let _config = AppConfigBuilder::default().build(&pool).unwrap();
}

#[test]
#[should_panic]
fn unwrap() {
    let mut pool = SimpleFlatPool::default();
    pool.add("verbose_level", 2.into())
        .add("database_dir", "/var/database".into())
        .add("dry_run", true.into());
    let _config = AppConfigBuilder::default().build(&pool);
}

#[test]
fn default() {
    let mut pool = SimpleFlatPool::default();
    pool.add("whatever", "something".into())
        .add("app_name", "Application".into())
        .add("database_dir", "/var/database".into());
    let config = AppConfigBuilder::default()
        .build(&pool)
        .map_err(|e| format!("{e}"))
        .unwrap();

    assert_eq!(VerboseLevel::Critical, config.verbose_level);
    assert!(!config.dry_run);
}
