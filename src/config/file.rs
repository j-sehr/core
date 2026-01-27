use crate::common::configuration::ConfigurationKey;
use figment::{providers::Format, value::Value};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileConfiguration {
    pub host: String,
    pub port: u16,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl FileConfiguration {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.extra.get(key)
    }

    pub fn get_as<T: serde::de::DeserializeOwned + ConfigurationKey>(&self) -> Option<T> {
        self.extra
            .get(T::get_config_key())
            .and_then(|value| value.deserialize().ok())
    }
}

pub fn load_file_configuration(file_path: &Path) -> anyhow::Result<FileConfiguration> {
    let figment = figment::Figment::from(figment::providers::Toml::file(file_path));
    let config = figment.extract()?;

    Ok(config)
}
