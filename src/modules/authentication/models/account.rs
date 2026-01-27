use crate::common::model::DatabaseModel;

use super::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountModel {
    pub id: BaseId,
    pub username: String,
    pub password: String,
    pub created_at: BaseDateTime,
    pub updated_at: BaseDateTime,
}

impl DatabaseModel for AccountModel {
    fn table_name() -> &'static str {
        "accounts"
    }

    fn key_prefix() -> String {
        "acc_".to_string()
    }
}
