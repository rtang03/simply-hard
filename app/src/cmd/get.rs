#[derive(Debug, Default)]
pub struct Get {
    msg: String,
}

impl Get {
    /// Create a new `Ping` command with optional `msg`.
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
