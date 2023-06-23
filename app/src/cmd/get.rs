#[derive(Debug, Default)]
pub struct Get {
    _msg: String,
}

impl Get {
    /// Create a new `Ping` command with optional `msg`.
    pub fn new(_msg: String) -> Self {
        Self { _msg }
    }
}
