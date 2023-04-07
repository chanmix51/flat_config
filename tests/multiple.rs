use flat_config::{
    pool::{FlatPool, SimpleFlatPool},
    ConfigBuilder, ConfigError,
};

#[derive(Clone, Debug, PartialEq)]
struct ConfigA {
    something: isize,
}

#[derive(Clone, Debug, PartialEq)]
struct ConfigB {
    something_else: String,
}

#[derive(Default)]
struct AppConfigBuilder;

impl ConfigBuilder<ConfigA> for AppConfigBuilder {
    fn build(&self, _config_pool: &impl FlatPool) -> Result<ConfigA, ConfigError> {
        let config = ConfigA { something: 1 };

        Ok(config)
    }
}

impl ConfigBuilder<ConfigB> for AppConfigBuilder {
    fn build(&self, _config_pool: &impl FlatPool) -> Result<ConfigB, ConfigError> {
        let config = ConfigB {
            something_else: "pika".to_string(),
        };

        Ok(config)
    }
}

#[test]
fn both_config() {
    let builder = AppConfigBuilder::default();
    let config_a: ConfigA = builder.build(&SimpleFlatPool::default()).unwrap();
    let config_b: ConfigB = builder.build(&SimpleFlatPool::default()).unwrap();

    assert_eq!(ConfigA { something: 1 }, config_a);
    assert_eq!(
        ConfigB {
            something_else: "pika".to_string()
        },
        config_b
    );
}
