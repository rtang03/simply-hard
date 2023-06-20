// pub mod settings;

// use crate::settings::{ENV_FILENAME, GLOBAL_SETTINGS};

// // DEVELOPER NOTE
// // https://cheats.rs/

// #[tokio::main]
// async fn main() {
//     println!("** Initializing configuration **");
//     println!("loading file: {}", ENV_FILENAME);

//     settings::Settings::print_config("Initial").await;

//     if let Err(e) = GLOBAL_SETTINGS.watch().await {
//         println!("watch error: {:?}", e);
//         println!("Quitting...");
//     }
// }
