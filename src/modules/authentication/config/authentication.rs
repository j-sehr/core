use crate::common::configuration::ConfigurationKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfiguration {
    pub jwt_secret: String,
    pub jwt_expiration_seconds: u64,
    pub refresh_token_expiration_days: u64,
}

impl ConfigurationKey for AuthenticationConfiguration {
    fn get_config_key() -> &'static str {
        "authentication"
    }
}
