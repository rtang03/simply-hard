use crate::models::{KeyValueStore, PersonRepository};
use crate::{Connection, InMemoryDatabase};
use tracing::{error, instrument};

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

    /// Apply the `Get` command to the specified `Db` instance.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self, repository, conn))]
    pub(crate) async fn apply<C>(
        self,
        repository: &PersonRepository,
        conn: &C,
    ) -> crate::Result<String>
    where
        C: Connection<Output = InMemoryDatabase> + Send + Sync,
    {
        match PersonRepository::get_value(repository, conn, self.key.as_str()).await {
            Ok(result) => Ok(result.value.into()),
            Err(err) => {
                error!(error = format!("{:?}", err));
                Err(err)
            }
        }
    }
}
