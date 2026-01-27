use crate::{common::configuration::ConfigurationKey, config::enviroment::EnviromentConfiguration};
use serde::{Deserialize, Serialize};

pub fn get_log_level(env_config: &EnviromentConfiguration) -> String {
    env_config
        .vars
        .get("log_level")
        .cloned()
        .unwrap_or("INFO".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingConfiguration {
    pub with_colors: bool,
    pub with_file: bool,
    pub with_line_number: bool,
    pub with_thread_ids: bool,
    pub with_thread_names: bool,
    pub with_target: bool,
    pub with_level: bool,
}

impl ConfigurationKey for LoggingConfiguration {
    fn get_config_key() -> &'static str {
        "logging"
    }
}

impl Default for LoggingConfiguration {
    fn default() -> Self {
        LoggingConfiguration {
            with_colors: true,
            with_file: true,
            with_line_number: true,
            with_thread_ids: true,
            with_thread_names: true,
            with_target: true,
            with_level: true,
        }
    }
}
