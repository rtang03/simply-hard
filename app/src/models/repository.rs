use crate::model::Name;
use async_trait::async_trait;
use colored::*;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::{error, info};

#[derive(Debug, Default, Clone)]
pub struct PersonRepository {}

#[async_trait]
pub trait Respository {
    async fn set_name(&self, conn: &Connection) -> Result<(), surrealdb::Error>;
}

#[async_trait]
impl Respository for PersonRepository {
    async fn set_name(&self, conn: &Connection) -> Result<(), surrealdb::Error> {
        if let Err(error) = conn
            .db
            .set(
                "name",
                Name {
                    first: "Tobie",
                    last: "Morgan Hitchcock",
                },
            )
            .await
        {
            println!("{:?}", error);
        }

        // let response: surrealdb::Response = conn.db.query("CREATE person SET name = $name").await?;

        Ok(())
    }
}

impl PersonRepository {
    pub async fn say_hello(&self, conn: &Connection) -> Result<(), surrealdb::Error> {
        conn.db.health().await?;

        println!("Database is health");

        Ok(())
    }
}

#[derive(Debug)]
pub struct Connection {
    pub db: Surreal<surrealdb::engine::remote::ws::Client>,
}

// TODO: change username and password to using environment variables
// TODO: change the error! and info! message to using template laterial

impl Connection {
    pub async fn new() -> Self {
        if let Ok(db) = Surreal::new::<Ws>("127.0.0.1:8000").await {
            // Signin as a namespace, database, or root user
            match db
                .signin(Root {
                    username: "root",
                    password: "root",
                })
                .await
            {
                Ok(_) => info!(message = "Sign in as root".blue().to_string()),
                Err(err) => {
                    let err_info = format!("{:?}", err);
                    error!(error = %err_info);
                    panic!("{}", "failed to signin".red())
                }
            }

            match db.use_ns("test").use_db("test").await {
                Ok(_) => {
                    info!(message = "Use [test] namespace; [test] database".blue().to_string())
                }
                Err(err) => {
                    let err_info = format!("{:?}", err);
                    error!(error = %err_info)
                }
            }

            return Self { db };
        };
        panic!("{}", "failed to connect surrealdb".red());
    }
}
