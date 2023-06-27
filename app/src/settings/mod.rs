//!
//! Settings
//!

use colored::*;
use config::{Config, Environment, File};
use notify::{event::ModifyKind, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{collections::HashMap, io::prelude::*, path::Path, time::Duration};
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info};

lazy_static::lazy_static! {
    pub static ref GLOBAL_SETTINGS: Settings<RwLock<Config>> = Settings::new();
}

// NOTE
// https://rust-unofficial.github.io/patterns/idioms/default.html
// https://github.com/mehcode/config-rs/blob/master/examples/env-list/main.rs
// https://tokio.rs/tokio/tutorial/channels
// https://github.com/notify-rs/notify/blob/5f8cbebea354175ec6afbdd8863720742e531b3a/examples/async_monitor.rs
// https://github.com/mehcode/config-rs/blob/master/examples/watch/main.rs
// https://stackoverflow.com/questions/29428227/return-local-string-as-a-slice-str

/// Global configuration, based on both config file "env.toml" and environment variables.
/// System environment variables, prefixed with "APP_", will be parsed. It will watch for
/// file content changes, every two seconds, asynchronously. The configuration is stored in
/// global static variables.
///
/// TODO: if env.toml is removed or renamed, it is unhandled here.
/// TODO: if it shall accept many sources of changes, and frequently change occurs,
/// it may uplift to multithread implementation
///
/// # Example
///
/// ```
/// use app::Settings;
/// use std::collections::HashMap;
///
/// let s = Settings::new();
///    assert!(s.0.blocking_read().clone().try_deserialize::<HashMap<String, String>>().is_ok());
/// ```
#[cfg_attr(feature = "server", derive(Debug, Default))]
pub struct Settings<T = RwLock<Config>>(pub T);

pub static ENV_FILENAME: &str = "env.toml";

impl Settings {
    /// Load configuration from env.toml file and environment variables
    /// environment variables shall be prefixed with APP_
    fn load_config() -> Config {
        let path = Path::new(ENV_FILENAME);
        let display = path.display();

        // create "env.toml" if it doesn't exist
        if !path.exists() {
            let mut new_file = match std::fs::File::create(path) {
                Ok(new_file) => new_file,
                Err(e) => panic!("Could not create file {}: {}", display, e),
            };

            // TODO: change loading sample_env from static file, as a future enhancement
            let sample_env = "debug = true";

            match new_file.write_all(sample_env.as_bytes()) {
                Ok(_) => info!(message = format!("{}", "env.toml created successfully.".blue())),
                Err(e) => panic!("Could not write to file {}: {}", display, e),
            }
        }

        // load configuration into Config object
        let env_config = Config::builder()
            .add_source(File::with_name(ENV_FILENAME))
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build();

        match env_config {
            Ok(config) => config,
            Err(err) => {
                error!(error = format!("cannot load file, {:?}", err));
                Config::default()
            }
        }
    }

    pub fn new() -> Self {
        let config = Self::load_config();

        Self(RwLock::new(config))
    }

    /// Print config information
    pub async fn print_config(prefix: &str) {
        println!(
            " * {} configuration * \n\t\x1b[31m{:?}\x1b[0m",
            prefix,
            GLOBAL_SETTINGS
                .0
                .read()
                .await
                .clone()
                .try_deserialize::<HashMap<String, String>>()
                .unwrap()
        );
    }

    /// Get configuration item
    pub async fn get_config_item(key: &str) -> Option<String> {
        match GLOBAL_SETTINGS
            .0
            .read()
            .await
            .clone()
            .try_deserialize::<HashMap<String, String>>()
        {
            Ok(config) => {
                if let Some(val) = config.get(key) {
                    let mut string = String::new();
                    string.push_str(val);
                    Some(string)
                } else {
                    None
                }
            }
            Err(err) => {
                error!(error = format!("fail to get config item, {:?}", err));
                None
            }
        }
    }

    pub async fn watch(&self) -> notify::Result<()> {
        let (tx, mut rx) = mpsc::channel(1);

        let mut watcher: RecommendedWatcher = Watcher::new(
            move |res| match res {
                Ok(Event {
                    kind: notify::event::EventKind::Modify(ModifyKind::Data(_)),
                    ..
                }) => tx.blocking_send(res).unwrap(),
                Err(err) => error!(error = format!("notify error, {:?}", err)),
                _ => {
                    // every content change emits two events, DataChange and MetaChange
                    // this arm ignores below metadata change events
                    // Event { kind: Modify(Metadata(Any)) }
                }
            },
            notify::Config::default().with_poll_interval(Duration::from_secs(2)),
        )?;

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(Path::new(ENV_FILENAME), RecursiveMode::NonRecursive)?;

        while let Some(res) = rx.recv().await {
            match res {
                Ok(Event {
                    kind: notify::event::EventKind::Modify(_),
                    ..
                }) => {
                    info!("{}", "env.toml written; refreshing configuration".blue());

                    // Settings::print_config("Before").await;
                    let mut write_lock = GLOBAL_SETTINGS.0.write().await;
                    *write_lock = Self::load_config();
                    drop(write_lock);
                    Settings::print_config("New").await;
                }
                Err(err) => error!(error = format!("recv error, {:?}", err)),
                _ => {
                    // ignore event
                }
            }
        }
        Ok(())
    }
}

#[test]
fn test_settings_new() {
    std::env::set_var("APP_UNITTEST", "unit test");
    let s = Settings::new();

    assert!(s
        .0
        .blocking_read()
        .clone()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()
        .get("unittest")
        .is_some());

    assert!(s
        .0
        .blocking_read()
        .clone()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()
        .get("noop")
        .is_none());

    std::env::remove_var("APP_UNITTEST");
}
