use tracing::instrument;

/// Get the value of the key
#[derive(Debug)]
pub struct Get {
    /// Name of the key to retrieve
    key: String,
}

impl Get {
    /// Create a new `Get` command which fetches the `key`
    pub fn new(key: impl ToString) -> Get {
        Get {
            key: key.to_string(),
        }
    }

    /// Get the key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Apply the `Get` command to the specified `Db` instance.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self))]
    pub(crate) async fn apply(self) -> crate::Result<()> {
        Ok(())
    }
}
