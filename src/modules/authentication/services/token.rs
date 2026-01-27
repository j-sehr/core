use crate::{
    common::model::DatabaseModel,
    config::file::FileConfiguration,
    modules::authentication::{
        config::authentication::AuthenticationConfiguration,
        models::{account::AccountModel, session::SessionModel},
    },
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use surrealdb::RecordId;

const SERVICE_NAME: &str = "TokenService";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenOpts {
    pub account_id: String,
    pub session_id: String,
    pub service: String,
}

impl TokenOpts {
    pub fn new(account_id: String, session_id: String, service: String) -> Self {
        Self {
            account_id,
            session_id,
            service,
        }
    }

    pub fn into_map(self) -> BTreeMap<&'static str, String> {
        let mut map = BTreeMap::new();
        map.insert("account_id", self.account_id);
        map.insert("session_id", self.session_id);
        map.insert("service", self.service);
        map
    }
}

pub struct TokenService {
    authentication_config: AuthenticationConfiguration,
}

impl TokenService {
    pub fn new(authentication_config: AuthenticationConfiguration) -> Self {
        Self {
            authentication_config,
        }
    }

    pub fn from_file_config(file_config: FileConfiguration) -> anyhow::Result<Self> {
        let authentication_config = file_config
            .get_as::<AuthenticationConfiguration>()
            .ok_or_else(|| {
                crate::log!(
                    tracing::error,
                    "{} Failed to load authentication config",
                    SERVICE_NAME
                )
            })?;

        Ok(Self::new(authentication_config))
    }

    pub fn generate_jwt(&self, opts: TokenOpts) -> anyhow::Result<(String, i64)> {
        let key = Hmac::<Sha256>::new_from_slice(self.authentication_config.jwt_secret.as_bytes())?;
        let mut claims = opts.into_map();
        let exp = chrono::Utc::now().timestamp()
            + self.authentication_config.jwt_expiration_seconds as i64;

        claims.insert("exp", exp.to_string());
        claims
            .sign_with_key(&key)
            .map(|jwt| (jwt, exp))
            .map_err(Into::into)
    }

    pub fn verify_jwt(&self, token: &str) -> anyhow::Result<(RecordId, RecordId)> {
        let key = Hmac::<Sha256>::new_from_slice(self.authentication_config.jwt_secret.as_bytes())?;
        let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;

        let account_id = AccountModel::from_named_format(
            claims
                .get("account_id")
                .ok_or_else(|| anyhow::anyhow!("account_id not found in token"))?,
        )
        .ok_or_else(|| crate::log!(tracing::warn, "{} Wrong account_id", SERVICE_NAME))?;

        let session_id = SessionModel::from_named_format(
            claims
                .get("session_id")
                .ok_or_else(|| anyhow::anyhow!("session_id not found in token"))?,
        )
        .ok_or_else(|| crate::log!(tracing::warn, "{} Wrong Session Id", SERVICE_NAME))?;

        let exp = claims
            .get("exp")
            .ok_or_else(|| anyhow::anyhow!("exp not found in token"))?
            .parse::<i64>()?;

        if chrono::Utc::now().timestamp() > exp {
            return Err(anyhow::anyhow!("Token has expired"));
        }

        Ok((account_id, session_id))
    }

    pub fn generate_refresh_token(&self) -> anyhow::Result<String> {
        use rand::Rng;
        let mut rng = rand::rng();
        let token: [u8; 64] = rng.random();
        Ok(Sha256::digest(token)
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect())
    }

    pub fn hash_refresh_token(&self, refresh_token: &str) -> String {
        Sha256::digest(refresh_token.as_bytes())
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}
