use crate::Settings;
use colored::Colorize;
use surrealdb::{engine::local::Mem, engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tracing::{error, info};

/// Inmemory database
#[derive(Debug)]
pub struct InMemoryDatabase {
    pub db: Surreal<surrealdb::engine::local::Db>,
    pub namespace: String,
    pub database_name: String,
}

/// Remote connection via SurrealDb client
#[derive(Debug)]
pub struct RemoteDatabase {
    pub db: Surreal<surrealdb::engine::remote::ws::Client>,
    pub port: u32,
    pub host: String,
    pub namespace: String,
    pub database_name: String,
    pub username: String,
    pub password: String,
}

/// SurrealDb client connection
#[tonic::async_trait]
pub trait Connection {
    type Output;
    async fn new() -> Self;
    fn get_db(&self) -> Self::Output;
}

#[tonic::async_trait]
impl Connection for InMemoryDatabase {
    type Output = InMemoryDatabase;

    fn get_db(&self) -> Self::Output {
        Self {
            db: self.db.to_owned(),
            namespace: self.namespace.to_owned(),
            database_name: self.database_name.to_owned(),
        }
    }

    async fn new() -> Self {
        let namespace = Settings::get_config_item("SURREALDB_NS")
            .await
            .unwrap_or("test".to_owned());

        let database_name = Settings::get_config_item("SURREALDB_DB")
            .await
            .unwrap_or("test".to_owned());

        // NOTE: if () is change to "Strict" server mode, will throw an NSNotFound exception
        // when submittting query.
        let db: surrealdb::Result<Surreal<surrealdb::engine::local::Db>> =
            Surreal::new::<Mem>(()).await;

        if let Ok(db) = db {
            match db
                .use_ns(namespace.clone())
                .use_db(database_name.clone())
                .await
            {
                Ok(_) => {
                    info!(
                        message = "Use namespace / database".blue().to_string(),
                        namespace, database_name
                    );
                }
                Err(err) => {
                    let err_info = format!("{:?}", err);
                    error!(error = %err_info);
                    panic!("{}", "failed to connect surrealdb".red());
                }
            };

            return Self {
                db,
                namespace,
                database_name,
            };
        };

        panic!("{}", "failed to connect surrealdb".red());
    }
}

#[tonic::async_trait]
impl Connection for RemoteDatabase {
    type Output = RemoteDatabase;

    fn get_db(&self) -> Self::Output {
        RemoteDatabase {
            db: self.db.to_owned(),
            namespace: self.namespace.to_owned(),
            database_name: self.database_name.to_owned(),
            port: self.port.to_owned(),
            host: self.host.to_owned(),
            username: self.username.to_owned(),
            password: self.password.to_owned(),
        }
    }

    async fn new() -> Self {
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
                Ok(_) => {
                    info!(
                        message = "Use namespace / database".blue().to_string(),
                        namespace, database_name
                    );
                }
                Err(err) => {
                    let err_info = format!("{:?}", err);
                    error!(error = %err_info);
                    panic!("{}", "failed to connect surrealdb".red());
                }
            };

            return Self {
                db,
                port,
                host,
                username,
                password,
                namespace,
                database_name,
            };
        }
        panic!("{}", "failed to connect surrealdb".red());
    }
}
