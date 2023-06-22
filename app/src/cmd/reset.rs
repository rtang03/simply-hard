#[derive(Debug, Default)]
pub struct Reset {
    msg: String,
}

impl Reset {
    /// Create a new `Ping` command with optional `msg`.
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
