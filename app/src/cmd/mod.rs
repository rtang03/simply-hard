mod ping;
pub use ping::Ping;

/// Enumeration of supported commands.
///
/// Methods called on `Command` are delegated to the command implementation.
#[derive(Debug)]
pub enum Command {
    Ping(Ping),
}

impl Command {
    /// Returns the command name
    pub(crate) fn get_name(&self) -> &str {
        match self {
            Command::Ping(_) => "ping",
        }
    }
}
