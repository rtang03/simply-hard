extern crate config;

use config::{Config, Environment, File};
use std::{collections::HashMap, env};

// DEVELOPER NOTE
// https://rust-unofficial.github.io/patterns/idioms/default.html
// https://github.com/mehcode/config-rs/blob/master/examples/env-list/main.rs

/// Global configuration, based on both config file "env.toml" and environment variables.
/// System environment variables, prefixed with "APP_", will be parsed.
///
/// # Example
///
/// ```
/// use settings::Settings;
///
/// let s = Settings::new();
/// assert!(!s.0.is_empty());
/// assert_eq!(s.0.get("package").unwrap(), "Simply-Hard");
/// ```
#[derive(Debug, Default)]
pub struct Settings(pub HashMap<String, String>);

impl Settings {
    pub fn new() -> Self {
        env::set_var("APP_PACKAGE", "Simply-Hard");

        let env_config = Config::builder()
            .add_source(File::with_name("env.toml"))
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build();

        let config = match env_config {
            Ok(config) => config,
            Err(e) => {
                println!("cannot load file, {}", e);
                return Self::default();
            }
        };

        Self(
            config
                .try_deserialize::<HashMap<String, String>>()
                .unwrap_or_default(),
        )
    }
}

#[test]
fn test_settings_new() {
    let s = Settings::new();
    println!("{:?}", s);
    assert!(!s.0.is_empty());
    assert_eq!(s.0.get("package").unwrap(), "Simply-Hard");
}
