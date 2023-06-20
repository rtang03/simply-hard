extern crate config;

use config::{Config, Environment, File};
use notify::{event::ModifyKind, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{collections::HashMap, path::Path, time::Duration};
use tokio::sync::{mpsc, RwLock};

lazy_static::lazy_static! {
    pub static ref GLOBAL_SETTINGS: Settings<RwLock<Config>> = Settings::new();
}

// DEVELOPER NOTE
// https://rust-unofficial.github.io/patterns/idioms/default.html
// https://github.com/mehcode/config-rs/blob/master/examples/env-list/main.rs
// https://tokio.rs/tokio/tutorial/channels
// https://github.com/notify-rs/notify/blob/5f8cbebea354175ec6afbdd8863720742e531b3a/examples/async_monitor.rs
// https://github.com/mehcode/config-rs/blob/master/examples/watch/main.rs

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
/// use settings::Settings;
/// use std::collections::HashMap;
///
/// let s = Settings::new();
///    assert!(s.0.blocking_read().clone().try_deserialize::<HashMap<String, String>>().is_ok());
/// ```
#[derive(Debug, Default)]
pub struct Settings<T = RwLock<Config>>(pub T);

pub static ENV_FILENAME: &str = "env.toml";

impl Settings {
    fn load_config() -> Config {
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
            Err(e) => {
                println!("cannot load file, {}", e);
                Config::default()
            }
        }
    }

    pub fn new() -> Self {
        let config = Self::load_config();

        Self(RwLock::new(config))
    }

    pub async fn print_config(prefix: &str) {
        println!(
            " * {} configuration :: \n\x1b[31m{:?}\x1b[0m",
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

    pub async fn watch(&self) -> notify::Result<()> {
        let (tx, mut rx) = mpsc::channel(1);

        let mut watcher: RecommendedWatcher = Watcher::new(
            move |res| match res {
                Ok(Event {
                    kind: notify::event::EventKind::Modify(ModifyKind::Data(_)),
                    ..
                }) => {
                    tx.blocking_send(res).unwrap();
                }
                Err(err) => println!("notify error: {:?}", err),
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
                    println!(" * env.toml written; refreshing configuration ...");
                    // Settings::print_config("Before").await;
                    let mut write_lock = GLOBAL_SETTINGS.0.write().await;
                    *write_lock = Self::load_config();
                    drop(write_lock);
                    Settings::print_config("New").await;
                }
                Err(err) => println!("recv error: {:?}", err),
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
