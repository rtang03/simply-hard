/// Returns PONG if no argument is provided, otherwise
/// return a copy of the argument as a bulk.
///
/// This command is often used to test if a connection
/// is still alive, or to measure latency.
#[derive(Debug, Default)]
pub struct Ping {
    msg: String,
}

impl Ping {
    /// Create a new `Ping` command with optional `msg`.
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
