use crate::{
    common::{module::Module, server::ServerSettings},
    config::{enviroment::EnviromentConfiguration, file::FileConfiguration},
    modules::{authentication::routes::routes, base::exports::DatabaseConnection},
};
use std::sync::Mutex;

pub struct AuthenticationModule;
#[async_trait::async_trait]
impl Module for AuthenticationModule {
    fn name(&self) -> &'static str {
        "core-auth"
    }

    async fn initialize(
        &self,
        _env_config: &EnviromentConfiguration,
        _file_config: &FileConfiguration,
        _: &Mutex<ServerSettings>,
    ) -> anyhow::Result<Option<axum::Router<()>>> {
        Ok(Some(routes()))
    }

    async fn run_migrations(
        &self,
        db_connection: DatabaseConnection,
        _env_config: &EnviromentConfiguration,
        _file_config: &FileConfiguration,
    ) -> anyhow::Result<()> {
        super::migrations::run_migrations(&db_connection).await?;
        Ok(())
    }
}
