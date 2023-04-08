# Flat Config
A dead simple configuration management tool.

The way this configuration manager works is the following:

![Flat Config overview diagram](assets/overview.jpg)

The code sequence is the following:

 * Create the configuration container required by the application services
 * Create the builder that will check, clean, set default values and turn flat values into Rust typed values.
 * Gather flat data from configuration source (Clap, Config etc.)

```rust
use flat_config::{
   config_setting::{TryUnwrap, ConfigSetting},
   error::ConfigError,
   setting_pool::{ConfigBuilder, ConfigSettingPool},
};

/// My application configuration.
pub struct MyConfig {
  setting_a: isize,
  setting_b: String,
  setting_c: Option<bool>,
}

/// Stateless configuration builder.
pub struct MyConfigBuilder;

/// Definition for building [MyConfig] instances.
impl ConfigBuilder<MyConfig> for MyConfigBuilder {
    fn build(&self, config_pool: &impl FlatPool) -> Result<MyConfig, ConfigError> {
        let setting_a: isize = config_pool.require("setting_a")?.try_unwrap()?;
        let setting_b: String = config_pool
            .get_or("setting_b", ConfigSetting::Text("something".to_string()))
            .try_unwrap()?;
        let setting_c: Option<bool> = config_pool.get("setting_c").map(|v| v.try_unwrap().unwrap());

        Ok(MyConfig { setting_a, setting_b, setting_c })
    }
}
```

Now, just grab the configuration settings and build the configuration instance from them:

```rust
    let mut pool = SimpleFlatPool::default();
    pool.add("setting_a", 200.into())
        .add("setting_c", None);
    let config = AppConfigBuilder::default()
        .build(&pool)
        .map_err(|e| format!("{e}"))
        .unwrap();

    assert_eq!("something".to_string(), config.setting_b);
```

In more complex configuration setups, there may be several sources of data with a notion of precedence. In most general cases it is like the following: 

>   hard coded default settings < configuration file settings < environment settings < command line parameters settings

FlatConfig achieves this by grouping several simple flat pools with a priority order:

```rust
// ↓ This could be the configuration settings read from a config file ↓
let mut default_pool = SimpleFlatPool::default();
file_pool
    .add("database_dir", "/var/database".into())
    .add("start_epoch", 0.into())
    .add("dry_run", false.into());

// ↓ This could be the configuration settings read from command line parameters
let mut file_pool = SimpleFlatPool::default();
parameter_pool
    .add("database_dir", "/alternate/dir".into())
    .add("app_name", "whatever".into())
    .add("start_epoch", 3.into());

// The order of the flat pools in the vector defines the precedence ↓
let config_pool = LayeredFlatPool::new(vec![Box::new(file_pool), Box::new(parameter_pool)]);
let config = AppConfigBuilder::default().build(&config_pool).unwrap();

assert!(!config.dry_run);
assert_eq!(3, config.start_epoch);
```
