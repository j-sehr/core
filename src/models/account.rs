use super::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: BaseId,
    pub username: String,
    pub password: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAccountDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateAccountDTO {
    pub username: Option<String>,

    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountDTO {
    pub id: BaseId,
    pub username: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct DeleteAccountDTO {
    pub id: BaseId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}
