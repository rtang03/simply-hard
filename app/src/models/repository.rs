use crate::{models::KeyValue, AppError, Connection, InMemoryDatabase};
use async_trait::async_trait;

// NOTE:
// https://github.com/surrealdb/surrealdb/tree/main/lib

#[derive(Debug, Default, Clone)]
pub struct PersonRepository {}

#[async_trait]
pub trait KeyValueStore<'a> {
    type Output;

    async fn get_value<C>(&self, conn: &'a C, key: &'a str) -> crate::Result<Self::Output>
    where
        C: Connection<Output = InMemoryDatabase> + Send + Sync;

    async fn set_value<C>(
        &self,
        conn: &'a C,
        key: &'a str,
        value: &'a str,
    ) -> crate::Result<Self::Output>
    where
        C: Connection<Output = InMemoryDatabase> + Send + Sync;
}

#[async_trait]
impl<'a> KeyValueStore<'a> for PersonRepository {
    type Output = KeyValue<'a>;

    async fn get_value<C>(&self, conn: &'a C, key: &'a str) -> crate::Result<Self::Output>
    where
        C: Connection<Output = InMemoryDatabase> + Send + Sync,
    {
        let result: surrealdb::Result<Option<KeyValue>> =
            conn.get_db().db.select(("kv", key)).await;

        match result {
            Ok(result) => Ok(result.unwrap()),
            Err(err) => Err(AppError::SurrealdbGetError(err)),
        }
    }

    async fn set_value<C>(
        &self,
        conn: &'a C,
        key: &'a str,
        value: &'a str,
    ) -> crate::Result<Self::Output>
    where
        C: Connection<Output = InMemoryDatabase> + Send + Sync,
    {
        let record: Result<Option<KeyValue>, surrealdb::Error> = conn
            .get_db()
            .db
            .create(("kv", key))
            .content(KeyValue {
                key: key.into(),
                value: value.into(),
            })
            .await;

        match record {
            Ok(result) => Ok(result.unwrap()),
            Err(err) => Err(AppError::SurrealdbSetError(err)),
        }
    }
}
