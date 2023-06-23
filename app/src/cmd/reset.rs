#[derive(Debug, Default)]
pub struct Reset {
    _msg: String,
}

impl Reset {
    /// Create a new `Ping` command with optional `msg`.
    pub fn new(_msg: String) -> Self {
        Self { _msg }
    }
}
