use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{
    common::configuration::ConfigurationKey,
    config::{enviroment::EnviromentConfiguration, file::FileConfiguration},
};

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    Root,
    Namespace,
    Database,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseConfiguration {
    pub host: String,
    pub port: u16,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub authentication_method: AuthenticationMethod,
}

impl ConfigurationKey for DatabaseConfiguration {
    fn get_config_key() -> &'static str {
        "database"
    }
}

impl DatabaseConfiguration {
    pub fn try_from_env(env_config: &EnviromentConfiguration) -> Option<Self> {
        let host = env_config.vars.get("DB_HOST")?.to_string();
        let port = env_config.vars.get("DB_PORT")?.parse::<u16>().ok()?;
        let namespace = env_config.vars.get("DB_NAMESPACE")?.to_string();
        let database = env_config.vars.get("DB_DATABASE")?.to_string();
        let username = env_config.vars.get("DB_USERNAME")?.to_string();
        let password = env_config.vars.get("DB_PASSWORD")?.to_string();
        let auth_method_str = env_config.vars.get("DB_AUTH_METHOD")?.to_string();

        let authentication_method = match auth_method_str.as_str() {
            "Root" => Some(AuthenticationMethod::Root),
            "Namespace" => Some(AuthenticationMethod::Namespace),
            "Database" => Some(AuthenticationMethod::Database),
            _ => return None,
        }?;

        Some(DatabaseConfiguration {
            host,
            port,
            namespace,
            database,
            username,
            password,
            authentication_method,
        })
    }

    pub fn get_from_env_or_file(
        env_config: &EnviromentConfiguration,
        file_config: &FileConfiguration,
    ) -> anyhow::Result<Self> {
        DatabaseConfiguration::try_from_env(env_config)
            .or_else(|| file_config.get_as::<Self>())
            .ok_or(anyhow!("No Database Config found in env or in the file"))
    }

    pub fn get_connection_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
