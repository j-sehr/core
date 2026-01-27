use super::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSessionOptions {
    pub account_id: BaseId,
    pub refresh_hash: String,
    pub expires_at: BaseDateTime,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshSessionOptions {
    pub account_id: BaseId,
    pub session_id: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionDTO {
    pub account_id: BaseId,
    pub refresh_token: String,
    pub access_token: String,
    pub expires_at: BaseDateTime,
}
