use crate::{model::Name, Settings};
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

/// SurrealDb client connection
#[derive(Debug)]
pub struct Connection {
    pub db: Surreal<surrealdb::engine::remote::ws::Client>,
}

impl Connection {
    pub async fn new() -> Self {
        // load from global setting, verify as u32. If NONE, use default 8000
        let port = Settings::get_config_item("SURREALDB_PORT")
            .await
            .unwrap_or("8000".to_owned())
            .parse::<u32>()
            .unwrap_or(8000);

        let host = Settings::get_config_item("SURREALDB_HOST")
            .await
            .unwrap_or("127.0.0.1".to_owned());

        let namespace = Settings::get_config_item("SURREALDB_NS")
            .await
            .unwrap_or("test".to_owned());

        let database_name = Settings::get_config_item("SURREALDB_DB")
            .await
            .unwrap_or("test".to_owned());

        let username = Settings::get_config_item("SURREALDB_USERNAME")
            .await
            .unwrap_or("root".to_owned());

        let password = Settings::get_config_item("SURREALDB_PASSWORD")
            .await
            .unwrap_or("root".to_owned());

        if let Ok(db) = Surreal::new::<Ws>(format!("{}:{}", host, port)).await {
            info!(
                message = format!("{}", "Connecting SurrealDb".blue()),
                host, port
            );

            // Signin as a namespace, database, or root user
            match db
                .signin(Root {
                    username: &username,
                    password: &password,
                })
                .await
            {
                Ok(_) => info!(message = "SurrealDb sign-in".blue().to_string(), username),
                Err(err) => {
                    let err_info = format!("{:?}", err);
                    error!(error = %err_info);
                    panic!("{}", "failed to signin".red())
                }
            }

            match db
                .use_ns(namespace.clone())
                .use_db(database_name.clone())
                .await
            {
                Ok(_) => info!(
                    message = "Use namespace / database".blue().to_string(),
                    namespace, database_name
                ),
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
