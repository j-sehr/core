use crate::common::model::DatabaseModel;

use super::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionModel {
    pub id: BaseId,
    pub account_id: BaseId,
    pub refresh_hash: String,
    pub created_at: BaseDateTime,
    pub expires_at: BaseDateTime,
    pub is_active: bool,
}

impl DatabaseModel for SessionModel {
    fn table_name() -> &'static str {
        "sessions"
    }

    fn key_prefix() -> String {
        "ses_".to_string()
    }
}
