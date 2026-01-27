use crate::{
    common::model::DatabaseModel,
    modules::{
        authentication::{
            dtos::account::CreateAccountRequestDTO, models::account::AccountModel,
            services::password::PasswordService,
        },
        base::exports::{BaseId, DatabaseConnection},
    },
};

#[derive(Debug, Clone)]
pub struct AccountService {
    database_connection: DatabaseConnection,
}

impl AccountService {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn get_all_accounts(&self) -> anyhow::Result<Vec<AccountModel>> {
        let accounts: Vec<AccountModel> = self
            .database_connection
            .select(AccountModel::table_name())
            .await?;

        Ok(accounts)
    }

    pub async fn get_account_by_id(
        &self,
        account_id: &BaseId,
    ) -> anyhow::Result<Option<AccountModel>> {
        let account: Option<AccountModel> = self.database_connection.select(account_id).await?;
        Ok(account)
    }

    pub async fn get_account_by_username(
        &self,
        username: &str,
    ) -> anyhow::Result<Option<AccountModel>> {
        let accounts: Vec<AccountModel> = self
            .database_connection
            .query("SELECT * FROM type::table($table) WHERE username = $username")
            .bind(("table", AccountModel::table_name()))
            .bind(("username", username.to_string()))
            .await?
            .take(0)?;

        Ok(accounts.first().cloned())
    }

    pub async fn get_account_by_username_and_password(
        &self,
        pasword_service: &PasswordService,
        username: &str,
        password: &str,
    ) -> anyhow::Result<Option<AccountModel>> {
        let account_opt = self.get_account_by_username(username).await?;
        if account_opt.is_none() {
            return Ok(None);
        }

        let account = account_opt.unwrap();
        let is_password_valid = pasword_service.verify_password(&account.password, password)?;
        if !is_password_valid {
            return Ok(None);
        }

        Ok(Some(account))
    }

    pub async fn exists_username(&self, username: &str) -> anyhow::Result<bool> {
        self.get_account_by_username(username)
            .await
            .map(|opt| opt.is_some())
    }

    pub async fn create_account(
        &self,
        password_service: &PasswordService,
        mut create_account: CreateAccountRequestDTO,
    ) -> anyhow::Result<Result<AccountModel, String>> {
        let exists_username = self.exists_username(&create_account.username).await?;
        if exists_username {
            return Ok(Err("Username already exists.".to_string()));
        }

        let hashed_password = password_service.hash_password(&create_account.password)?;
        create_account.password = hashed_password;

        let created_accounts: Vec<AccountModel> = self
            .database_connection
            .insert(AccountModel::table_name())
            .content(create_account)
            .await?;

        Ok(created_accounts
            .first()
            .cloned()
            .ok_or_else(|| "Failed to create account due to an unknown error.".to_string()))
    }

    pub async fn update_account_password(
        &self,
        password_service: &PasswordService,
        account_id: &BaseId,
        new_password: &str,
    ) -> anyhow::Result<()> {
        let hashed_password = password_service.hash_password(new_password)?;

        self.database_connection
            .query("UPDATE type::table($table) SET password = $password, updated_at = time::now() WHERE id = $id")
            .bind(("table", AccountModel::table_name()))
            .bind(("password", hashed_password))
            .bind(("id", account_id.key().to_string()))
            .await?;

        Ok(())
    }

    pub async fn update_account_username(
        &self,
        account_id: &BaseId,
        new_username: &str,
    ) -> anyhow::Result<()> {
        self.database_connection
            .query("UPDATE type::table($table) SET username = $username, updated_at = time::now() WHERE id = $id")
            .bind(("table", AccountModel::table_name()))
            .bind(("username", new_username.to_string()))
            .bind(("id", account_id.key().to_string()))
            .await?;

        Ok(())
    }

    pub async fn delete_account(&self, account_id: &BaseId) -> anyhow::Result<()> {
        let _: Option<AccountModel> = self.database_connection.delete(account_id).await?;

        Ok(())
    }
}
