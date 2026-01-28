use crate::{
    common::{app_state::AppContext, module::Module, server::ServerSettings},
    config::{enviroment::EnviromentConfiguration, file::FileConfiguration},
};
use axum::{Extension, Router};
use std::sync::Mutex;

pub mod authentication;
pub mod base;

pub fn get_modules() -> Vec<Box<dyn Module>> {
    vec![
        Box::new(base::exports::BaseModule),
        Box::new(authentication::exports::AuthenticationModule),
    ]
}

pub async fn initialize_modules(
    env_config: &EnviromentConfiguration,
    file_config: &FileConfiguration,
) -> anyhow::Result<Router<()>> {
    let server_settings = Mutex::new(ServerSettings::new(
        file_config.host.clone(),
        file_config.port,
    ));
    let mut router = Router::new();

    for module in get_modules() {
        let router_opt = module
            .initialize(env_config, file_config, &server_settings)
            .await?;

        let db_connection = {
            let server_settings_lock = server_settings.lock().map_err(|e| {
                crate::log!(
                    tracing::error,
                    "Failed to get the Server Settings Lock: {}",
                    e
                )
            })?;

            let db_connection = server_settings_lock.get_database_connection();
            if db_connection.is_none() {
                tracing::error!("Failed to establish database connection");
                std::process::exit(1);
            }
            db_connection.unwrap().clone()
        };

        module
            .run_migrations(db_connection, env_config, file_config)
            .await?;

        tracing::info!("Module '{}' initialized", module.name());

        if router_opt.is_none() {
            continue;
        }

        router = router.merge(router_opt.unwrap());
    }

    let server_settings_lock = server_settings.lock().map_err(|e| {
        crate::log!(
            tracing::error,
            "Failed to get the Server Settings Lock: {}",
            e
        )
    })?;

    let db_connection = server_settings_lock.get_database_connection();
    if db_connection.is_none() {
        tracing::error!("Failed to establish database connection");
        std::process::exit(1);
    }

    let app_state = AppContext::new(
        db_connection.unwrap().clone(),
        file_config.clone(),
        env_config.clone(),
    )
    .into_state_context();

    Ok(router.layer(Extension(app_state)))
}
