use figment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Default, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Environment {
    #[default]
    Development,
    Staging,
    Production,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct EnviromentConfiguration {
    #[serde(default)]
    pub env_mode: Environment,
    #[serde(flatten, default)]
    pub vars: HashMap<String, String>,
}

impl EnviromentConfiguration {
    pub fn get_var(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }
}

pub fn load_environment_config() -> anyhow::Result<EnviromentConfiguration> {
    let figment = figment::Figment::from(figment::providers::Env::prefixed("CORE_"));
    figment
        .extract::<EnviromentConfiguration>()
        .map_err(|e| anyhow::anyhow!(e))
}
