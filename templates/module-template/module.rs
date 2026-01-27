use crate::{
    common::{module::Module, server::ServerSettings},
    config::{enviroment::EnviromentConfiguration, file::FileConfiguration},
    modules::base::exports::DatabaseConnection,
};
use std::sync::Mutex;

pub(super) struct TemplateModule;
#[async_trait::async_trait]
impl Module for TemplateModule {
    fn name(&self) -> &'static str {
        "core-template"
    }

    async fn initialize(
        &self,
        env_config: &EnviromentConfiguration,
        file: &FileConfiguration,
        _: &Mutex<ServerSettings>,
    ) -> anyhow::Result<Option<axum::Router<()>>> {
        Ok(None)
    }

    async fn run_migrations(
        &self,
        _db: DatabaseConnection,
        _env_config: &EnviromentConfiguration,
        _file_config: &FileConfiguration,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
