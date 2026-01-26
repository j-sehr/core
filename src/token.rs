use crate::models::prelude::BaseId;
use anyhow::Result;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use surrealdb::RecordId;

fn get_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn generate_jwt(account_id: &BaseId, session_id: &BaseId) -> Result<String> {
    let key = Hmac::<Sha256>::new_from_slice(get_secret().as_bytes())?;
    let mut claims = BTreeMap::new();
    claims.insert("account_id", account_id.key().to_string());
    claims.insert("session_id", session_id.key().to_string());

    claims.sign_with_key(&key).map_err(Into::into)
}

pub fn verify_jwt(token: &str) -> Result<(BaseId, BaseId)> {
    let key = Hmac::<Sha256>::new_from_slice(get_secret().as_bytes())?;
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;
    let account_id = claims
        .get("account_id")
        .ok_or_else(|| anyhow::anyhow!("account_id not found in token"))?;
    let session_id = claims
        .get("session_id")
        .ok_or_else(|| anyhow::anyhow!("session_id not found in token"))?;
    Ok((
        RecordId::from_table_key("accounts", account_id),
        RecordId::from_table_key("sessions", session_id),
    ))
}

pub fn generate_refresh_token() -> Result<String> {
    use rand::Rng;
    let mut rng = rand::rng();
    let token: [u8; 32] = rng.random();
    Ok(Sha256::digest(token)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect())
}

pub fn hash_refresh_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}
