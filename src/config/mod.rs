use std::path::PathBuf;

pub mod enviroment;
pub mod file;

const DEFAULT_CONFIG_FILE_PATH: &str = "./config.toml";
const CONFIG_FILE_ENV_VAR: &str = "CONFIG_FILE_PATH";

pub fn load_configurations()
-> anyhow::Result<(enviroment::EnviromentConfiguration, file::FileConfiguration)> {
    let env_config = enviroment::load_environment_config()?;
    let config_file_path = PathBuf::from(
        env_config
            .vars
            .get(CONFIG_FILE_ENV_VAR)
            .cloned()
            .unwrap_or_else(|| DEFAULT_CONFIG_FILE_PATH.to_string()),
    );

    let file_config = file::load_file_configuration(&config_file_path)?;

    Ok((env_config, file_config))
}
