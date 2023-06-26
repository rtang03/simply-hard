use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

#[derive(Debug, Default, Clone)]
pub struct PersonRepository {}

pub trait Respository {
    fn get() -> Result<(), surrealdb::Error>;
}

impl Respository for PersonRepository {
    fn get() -> Result<(), surrealdb::Error> {
        Ok(())
    }
}

impl PersonRepository {
    // pub async fn new() -> Self {
    //     Self {
    //         connection: Connection::new().await,
    //     }
    // }
    pub async fn perform(&self, conn: &Connection) -> Result<(), surrealdb::Error> {
        conn.db.health().await?;
        println!("i am ok");
        Ok(())
    }
}

#[derive(Debug)]
pub struct Connection {
    pub db: Surreal<surrealdb::engine::remote::ws::Client>,
}

impl Connection {
    pub async fn new() -> Self {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

        db.use_ns("test").use_db("test").await.unwrap();

        // panic!("failed to connect surrealdb");
        Connection { db }
    }
}
