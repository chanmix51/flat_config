use crate::{ConfigError, FlatPool};

/// ### Configuration builder trait.
///
/// Implementations of this trait create configurations reading data from the given
/// [ConfigSettingPool].
///
/// ```rust
/// use flat_config::{ConfigBuilder, TryUnwrap, FlatValue, ConfigError, FlatPool};
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
///     fn build(&self, config_pool: &FlatPool) -> Result<MyConfig, ConfigError> {
///         let setting_a: isize = config_pool.require("setting_a")?.try_unwrap()?;
///         let setting_b: String = config_pool
///             .get_or("setting_b", FlatValue::Text("something".to_string()))
///             .try_unwrap()?;
///         let setting_c: Option<bool> = config_pool.get("setting_c").map(|v| v.try_unwrap().unwrap());
///
///         Ok(MyConfig { setting_a, setting_b, setting_c })
///     }
/// }
/// ```
pub trait ConfigBuilder<T> {
    fn build(&self, config_pool: &FlatPool) -> Result<T, ConfigError>;
}
