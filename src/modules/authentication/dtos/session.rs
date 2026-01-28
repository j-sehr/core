use super::prelude::*;
use crate::{
    common::model::DatabaseModel,
    modules::authentication::{account_model::AccountModel, session_model::SessionModel},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSessionOptions {
    pub account_id: BaseId,
    pub refresh_hash: String,
    pub expires_at: BaseDateTime,
    pub user_agent: String,
    pub ip_address: String,
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
    pub id: String,
    pub account_id: String,
    pub ip_address: String,
    pub user_agent: String,
    pub expires_at: BaseDateTime,
}

impl From<SessionModel> for SessionDTO {
    fn from(session: SessionModel) -> Self {
        SessionDTO {
            id: SessionModel::to_named_format(&session.id),
            account_id: AccountModel::to_named_format(&session.account_id),
            ip_address: session.ip_address,
            user_agent: session.user_agent,
            expires_at: session.expires_at,
        }
    }
}

impl From<&SessionModel> for SessionDTO {
    fn from(session: &SessionModel) -> Self {
        SessionDTO {
            id: SessionModel::to_named_format(&session.id),
            account_id: AccountModel::to_named_format(&session.account_id),
            ip_address: session.ip_address.clone(),
            user_agent: session.user_agent.clone(),
            expires_at: session.expires_at.clone(),
        }
    }
}
