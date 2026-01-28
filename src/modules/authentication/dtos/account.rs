use super::prelude::*;
use crate::{common::model::DatabaseModel, modules::authentication::account_model::AccountModel};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountDTO {
    pub id: String,
    pub username: String,
    pub created_at: BaseDateTime,
    pub updated_at: BaseDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAccountRequestDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateAccountRequestDTO {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteAccountRequestDTO {
    pub id: String,
}

impl From<AccountModel> for AccountDTO {
    fn from(account: AccountModel) -> Self {
        AccountDTO {
            id: AccountModel::to_named_format(&account.id),
            username: account.username,
            created_at: account.created_at,
            updated_at: account.updated_at,
        }
    }
}

impl From<&AccountModel> for AccountDTO {
    fn from(account: &AccountModel) -> Self {
        AccountDTO {
            id: AccountModel::to_named_format(&account.id),
            username: account.username.clone(),
            created_at: account.created_at.clone(),
            updated_at: account.updated_at.clone(),
        }
    }
}
