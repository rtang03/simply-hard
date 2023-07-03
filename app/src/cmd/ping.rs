use crate::{AppError, Connection, InMemoryDatabase};
use tracing::instrument;

/// Returns PONG if no argument is provided, otherwise
/// return a copy of the argument as a bulk.
///
/// This command is often used to test if a connection
/// is still alive, or to measure latency.
#[derive(Debug)]
pub struct Ping {
    message: String,
}

impl Ping {
    /// Create a new `Ping` command with optional `message`.
    pub fn new(message: impl ToString) -> Ping {
        Ping {
            message: message.to_string(),
        }
    }

    /// Apply the `Ping` command and return the message.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self, conn))]
    pub(crate) async fn apply<C>(self, conn: &C) -> crate::Result<String>
    where
        C: Connection<Output = InMemoryDatabase>,
    {
        match conn.get_db().db.health().await {
            Ok(_) => Ok(self.message.to_uppercase()),
            Err(err) => Err(AppError::SurrealdbUnHealthy(err)),
        }
    }
}
