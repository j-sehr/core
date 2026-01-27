use super::server::ServerSettings;
use crate::{
    config::{enviroment::EnviromentConfiguration, file::FileConfiguration},
    modules::base::exports::DatabaseConnection,
};
use std::sync::Mutex;

#[async_trait::async_trait]
pub trait Module {
    fn name(&self) -> &'static str;

    async fn initialize(
        &self,
        env_config: &EnviromentConfiguration,
        file_config: &FileConfiguration,
        server_settings: &Mutex<ServerSettings>,
    ) -> anyhow::Result<Option<axum::Router<()>>>;

    async fn run_migrations(
        &self,
        db: DatabaseConnection,
        env_config: &EnviromentConfiguration,
        file_config: &FileConfiguration,
    ) -> anyhow::Result<()>;
}
