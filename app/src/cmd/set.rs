use crate::{
    models::{KeyValueStore, PersonRepository},
    Connection, InMemoryDatabase,
};
use tracing::{error, instrument};

/// Set `key` to hold the string `value`.
///
/// If `key` already holds a value, it is overwritten, regardless of its type.
/// Any previous time to live associated with the key is discarded on successful
/// SET operation.
///
#[derive(Debug)]
pub struct Set {
    /// the lookup key
    key: String,

    /// the value to be stored
    value: String,
}

impl Set {
    /// Create a new `Set` command which sets `key` to `value`.
    pub fn new(key: impl ToString, value: impl ToString) -> Self {
        Set {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    /// Parse a `Set` instance from a received frame.
    ///
    /// The `Parse` argument provides a cursor-like API to read fields from the
    /// `Frame`. At this point, the entire frame has already been received from
    /// the socket.
    ///
    /// The `SET` string has already been consumed.
    ///
    /// # Returns
    ///
    /// Returns the `Set` value on success. If the frame is malformed, `Err` is
    /// returned.
    ///
    #[instrument(skip(self, repository, conn), name = "db_set_value")]
    pub(crate) async fn apply<C>(
        self,
        repository: &PersonRepository,
        conn: &C,
    ) -> crate::Result<String>
    where
        C: Connection<Output = InMemoryDatabase> + Send + Sync,
    {
        match PersonRepository::set_value(repository, conn, self.key.as_str(), self.value.as_str())
            .await
        {
            Ok(_) => Ok("Ok".to_owned()),
            Err(err) => {
                error!(error = format!("{:?}", err));
                Err(err)
            }
        }
    }
}
