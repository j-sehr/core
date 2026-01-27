use crate::config::{enviroment::EnviromentConfiguration, file::FileConfiguration};
use axum::Router;

pub mod common;
pub mod config;
pub mod modules;
pub mod utils;

pub async fn bootstrap() -> anyhow::Result<(Router<()>, EnviromentConfiguration, FileConfiguration)>
{
    let (env_config, file_config) = config::load_configurations()?;
    let router = modules::initialize_modules(&env_config, &file_config).await?;

    Ok((router, env_config, file_config))
}
