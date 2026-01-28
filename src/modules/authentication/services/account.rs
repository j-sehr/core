use crate::{
    common::model::DatabaseModel,
    modules::{
        authentication::{
            dtos::account::CreateAccountRequestDTO, errors::service::*,
            models::account::AccountModel, services::password::PasswordService,
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

    pub async fn get_all_accounts(&self) -> Result<Vec<AccountModel>, AuthenticationServiceError> {
        let accounts: Vec<AccountModel> = self
            .database_connection
            .select(AccountModel::table_name())
            .await
            .map_err(AuthenticationServiceError::from_error)?;

        Ok(accounts)
    }

    pub async fn get_account_by_id(
        &self,
        account_id: &BaseId,
    ) -> Result<AccountModel, AuthenticationServiceError> {
        self.database_connection
            .select(account_id)
            .await
            .map(|val| {
                val.ok_or(AuthenticationServiceError::client(
                    AuthenticationClientError::AccountNotFound,
                ))
            })
            .map_err(AuthenticationServiceError::from_error)?
    }

    pub async fn get_account_by_username(
        &self,
        username: &str,
    ) -> Result<AccountModel, AuthenticationServiceError> {
        let accounts: Vec<AccountModel> = self
            .database_connection
            .query("SELECT * FROM type::table($table) WHERE username = $username")
            .bind(("table", AccountModel::table_name()))
            .bind(("username", username.to_string()))
            .await
            .map_err(|e| AuthenticationServiceError::ServerError(e.into()))?
            .take(0)
            .map_err(|_| {
                AuthenticationServiceError::client(AuthenticationClientError::AccountNotFound)
            })?;

        accounts
            .first()
            .cloned()
            .ok_or(AuthenticationServiceError::client(
                AuthenticationClientError::AccountNotFound,
            ))
    }

    pub async fn get_account_by_username_and_password(
        &self,
        pasword_service: &PasswordService,
        username: &str,
        password: &str,
    ) -> Result<AccountModel, AuthenticationServiceError> {
        let account = self.get_account_by_username(username).await?;
        let is_password_valid = pasword_service.verify_password(&account.password, password)?;
        if !is_password_valid {
            return Err(AuthenticationServiceError::client(
                AuthenticationClientError::InvalidCredentials,
            ));
        }

        Ok(account)
    }

    pub async fn exists_username(
        &self,
        username: &str,
    ) -> Result<bool, AuthenticationServiceError> {
        let res = self.get_account_by_username(username).await;
        match res {
            Err(AuthenticationServiceError::ClientError(
                AuthenticationClientError::AccountNotFound,
            )) => Ok(false),
            Err(e) => Err(e),
            _ => Ok(true),
        }
    }

    pub async fn create_account(
        &self,
        password_service: &PasswordService,
        mut create_account: CreateAccountRequestDTO,
    ) -> Result<AccountModel, AuthenticationServiceError> {
        let exists_username = self.exists_username(&create_account.username).await?;
        if exists_username {
            return Err(AuthenticationServiceError::client(
                AuthenticationClientError::AccountAlreadyExists,
            ));
        }

        let hashed_password = password_service.hash_password(&create_account.password)?;
        create_account.password = hashed_password;

        let created_accounts: Vec<AccountModel> = self
            .database_connection
            .insert(AccountModel::table_name())
            .content(create_account)
            .await
            .map_err(AuthenticationServiceError::from_error)?;

        created_accounts
            .first()
            .cloned()
            .ok_or(AuthenticationServiceError::ServerError(anyhow::anyhow!(
                "Account creation failed without a specific error."
            )))
    }

    pub async fn update_account_password(
        &self,
        password_service: &PasswordService,
        account_id: &BaseId,
        new_password: &str,
    ) -> Result<(), AuthenticationServiceError> {
        let hashed_password = password_service.hash_password(new_password)?;

        self.database_connection
            .query("UPDATE type::table($table) SET password = $password, updated_at = time::now() WHERE id = $id")
            .bind(("table", AccountModel::table_name()))
            .bind(("password", hashed_password))
            .bind(("id", account_id.key().to_string()))
            .await
            .map_err(AuthenticationServiceError::from_error)?;

        Ok(())
    }

    pub async fn update_account_username(
        &self,
        account_id: &BaseId,
        new_username: &str,
    ) -> Result<(), AuthenticationServiceError> {
        self.database_connection
            .query("UPDATE type::table($table) SET username = $username, updated_at = time::now() WHERE id = $id")
            .bind(("table", AccountModel::table_name()))
            .bind(("username", new_username.to_string()))
            .bind(("id", account_id.key().to_string()))
            .await
            .map_err(AuthenticationServiceError::from_error)?;

        Ok(())
    }

    pub async fn delete_account(
        &self,
        account_id: &BaseId,
    ) -> Result<(), AuthenticationServiceError> {
        let _: Option<AccountModel> = self
            .database_connection
            .delete(account_id)
            .await
            .map_err(AuthenticationServiceError::from_error)?;

        Ok(())
    }
}
