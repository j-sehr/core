use crate::{
    common::model::DatabaseModel,
    config::file::FileConfiguration,
    modules::authentication::{
        config::authentication::AuthenticationConfiguration,
        errors::service::*,
        models::{account::AccountModel, session::SessionModel},
    },
};
use chrono::{DateTime, Utc};
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
        // NOT A SERVICE FUNCTION ONLY FOR INITIALIZATION
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

    pub fn generate_jwt(
        &self,
        opts: TokenOpts,
    ) -> Result<(String, DateTime<Utc>), AuthenticationServiceError> {
        let key = Hmac::<Sha256>::new_from_slice(self.authentication_config.jwt_secret.as_bytes())
            .map_err(AuthenticationServiceError::from_error)?;
        let mut claims = opts.into_map();
        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(
                self.authentication_config.jwt_expiration_seconds as i64,
            ))
            .ok_or_else(|| {
                AuthenticationServiceError::ServerError(crate::log!(
                    tracing::error,
                    "{} Failed to calculate JWT expiration time",
                    SERVICE_NAME
                ))
            })?;

        claims.insert("exp", exp.to_rfc3339());
        claims
            .sign_with_key(&key)
            .map(|jwt| (jwt, exp))
            .map_err(AuthenticationServiceError::from_error)
    }

    pub fn verify_jwt(
        &self,
        token: &str,
    ) -> Result<(RecordId, RecordId), AuthenticationServiceError> {
        let key = Hmac::<Sha256>::new_from_slice(self.authentication_config.jwt_secret.as_bytes())
            .map_err(AuthenticationServiceError::from_error)?;

        let claims: BTreeMap<String, String> = token
            .verify_with_key(&key)
            .map_err(AuthenticationServiceError::from_error)?;

        let account_id = AccountModel::from_named_format(claims.get("account_id").ok_or(
            AuthenticationServiceError::client(AuthenticationClientError::InvalidAccessToken),
        )?)
        .ok_or(AuthenticationServiceError::client(
            AuthenticationClientError::InvalidAccountId,
        ))?;

        let session_id = SessionModel::from_named_format(claims.get("session_id").ok_or(
            AuthenticationServiceError::client(AuthenticationClientError::InvalidAccessToken),
        )?)
        .ok_or(AuthenticationServiceError::client(
            AuthenticationClientError::InvalidSessionId,
        ))?;

        let exp = claims
            .get("exp")
            .ok_or(AuthenticationServiceError::client(
                AuthenticationClientError::InvalidAccessToken,
            ))?
            .parse::<DateTime<Utc>>()
            .map_err(|_| {
                AuthenticationServiceError::client(AuthenticationClientError::InvalidAccessToken)
            })?;

        if chrono::Utc::now() > exp {
            return Err(AuthenticationServiceError::client(
                AuthenticationClientError::ExpiredAccessToken,
            ));
        }

        Ok((account_id, session_id))
    }

    pub fn generate_refresh_token(&self) -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        let token: [u8; 64] = rng.random();
        Sha256::digest(token)
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    pub fn hash_refresh_token(&self, refresh_token: &str) -> String {
        Sha256::digest(refresh_token.as_bytes())
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}
