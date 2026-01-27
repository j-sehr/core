use crate::modules::base::config::database::*;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
};

pub type DatabaseConnection = Surreal<Client>;

pub async fn connect_to_database(
    db_config: &DatabaseConfiguration,
) -> anyhow::Result<DatabaseConnection> {
    let db = Surreal::new::<Ws>(db_config.get_connection_string()).await?;

    match db_config.authentication_method {
        AuthenticationMethod::Root => {
            db.signin(surrealdb::opt::auth::Root {
                username: &db_config.username,
                password: &db_config.password,
            })
            .await?;

            db.use_ns(&db_config.namespace)
                .use_db(&db_config.database)
                .await?;
        }

        AuthenticationMethod::Namespace => {
            db.signin(surrealdb::opt::auth::Namespace {
                namespace: &db_config.namespace,
                username: &db_config.username,
                password: &db_config.password,
            })
            .await?;

            db.use_db(&db_config.database).await?;
        }

        AuthenticationMethod::Database => {
            db.signin(surrealdb::opt::auth::Database {
                namespace: &db_config.namespace,
                database: &db_config.database,
                username: &db_config.username,
                password: &db_config.password,
            })
            .await?;
        }
    };

    Ok(db)
}
