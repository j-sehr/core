use super::prelude::*;

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
