use super::{
    config::{database::DatabaseConfiguration, logging::*},
    database::connection::connect_to_database,
};
use crate::{
    common::{module::Module, server::ServerSettings},
    config::{enviroment::EnviromentConfiguration, file::FileConfiguration},
    modules::base::exports::DatabaseConnection,
};
use anyhow::anyhow;
use std::{str::FromStr, sync::Mutex};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt};

pub struct BaseModule;
#[async_trait::async_trait]
impl Module for BaseModule {
    fn name(&self) -> &'static str {
        "core-base"
    }

    async fn initialize(
        &self,
        env_config: &EnviromentConfiguration,
        file_config: &FileConfiguration,
        server_settings: &Mutex<ServerSettings>,
    ) -> anyhow::Result<Option<axum::Router<()>>> {
        let log_level = get_log_level(env_config);
        let env_filter = EnvFilter::from_str(&log_level)?;
        let log_config = file_config
            .get_as::<LoggingConfiguration>()
            .unwrap_or_default();

        tracing_subscriber::registry()
            .with(
                fmt::layer()
                    .with_file(log_config.with_file)
                    .with_level(log_config.with_level)
                    .with_line_number(log_config.with_line_number)
                    .with_ansi(log_config.with_colors)
                    .with_thread_ids(log_config.with_thread_ids)
                    .with_thread_names(log_config.with_thread_names)
                    .with_target(log_config.with_target)
                    .with_filter(env_filter),
            )
            .init();

        let db_config = DatabaseConfiguration::get_from_env_or_file(env_config, file_config)?;
        let db_connection = connect_to_database(&db_config).await?;
        {
            let mut settings = server_settings.lock().map_err(|e| {
                anyhow!("Failed to get mutex lock for setting database: {}", dbg!(e)) // VERY IMPOTANT dbg! HERE, FOR KNOWING WHEN IT HAPPENS
            })?;

            settings.database_connection = Some(db_connection);
            Ok::<(), anyhow::Error>(())
        }?;

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
