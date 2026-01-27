use crate::{
    config::{enviroment::EnviromentConfiguration, file::FileConfiguration},
    modules::base::exports::DatabaseConnection,
};

pub type AppStateContext = AppContext;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub database: DatabaseConnection,
    pub file_config: FileConfiguration,
    pub env_config: EnviromentConfiguration,
}

impl AppContext {
    pub fn new(
        database: DatabaseConnection,
        file_config: FileConfiguration,
        env_config: EnviromentConfiguration,
    ) -> Self {
        Self {
            database,
            file_config,
            env_config,
        }
    }

    pub fn into_state_context(self) -> AppContext {
        self
    }
}

impl PartialEq for AppContext {
    fn eq(&self, other: &Self) -> bool {
        self.file_config == other.file_config && self.env_config == other.env_config
    }
}

impl Eq for AppContext {}
