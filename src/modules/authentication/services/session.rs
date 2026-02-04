use crate::{
    common::model::DatabaseModel,
    modules::{
        authentication::{
            config::authentication::AuthenticationConfiguration,
            dtos::{authentication::AuthenticationResponseDto, session::CreateSessionOptions},
            errors::service::*,
            models::{account::AccountModel, session::SessionModel},
            services::token::{TokenOpts, TokenService},
        },
        base::exports::{
            BaseDateTime, BaseId, DatabaseConnection, request_info::RequestInfoExtractor,
        },
    },
};

pub struct SessionService {
    database_connection: DatabaseConnection,
    authentication_config: AuthenticationConfiguration,
}

impl SessionService {
    pub fn new(
        authentication_config: AuthenticationConfiguration,
        database_connection: DatabaseConnection,
    ) -> Self {
        SessionService {
            database_connection,
            authentication_config,
        }
    }

    pub async fn get_all_sessions(&self) -> Result<Vec<SessionModel>, AuthenticationServiceError> {
        let sessions: Vec<SessionModel> = self
            .database_connection
            .query("SELECT * FROM type::table($table)")
            .bind(("table", SessionModel::table_name()))
            .await
            .map_err(AuthenticationServiceError::from_error)?
            .take(0)
            .map_err(|_| {
                AuthenticationServiceError::client(AuthenticationClientError::SessionNotFound)
            })?;

        Ok(sessions)
    }

    pub async fn get_session_by_id(
        &self,
        session_id: &BaseId,
    ) -> Result<SessionModel, AuthenticationServiceError> {
        let session: Option<SessionModel> = self
            .database_connection
            .select(session_id)
            .await
            .map_err(AuthenticationServiceError::from_error)?;

        session.ok_or(AuthenticationServiceError::client(
            AuthenticationClientError::SessionNotFound,
        ))
    }

    pub async fn get_all_sessions_for_account(
        &self,
        account_id: &BaseId,
    ) -> Result<Vec<SessionModel>, AuthenticationServiceError> {
        let sessions: Vec<SessionModel> = self
            .database_connection
            .query("SELECT * FROM type::table($table) where account_id = $account_id")
            .bind(("table", SessionModel::table_name()))
            .bind(("account_id", account_id.clone()))
            .await
            .map_err(AuthenticationServiceError::from_error)?
            .take(0)
            .map_err(|_| {
                AuthenticationServiceError::client(AuthenticationClientError::SessionNotFound)
            })?;

        Ok(sessions)
    }

    pub async fn get_session_by_refresh_token_hash(
        &self,
        refresh_token_hash: String,
    ) -> Result<SessionModel, AuthenticationServiceError> {
        let session_vec: Vec<SessionModel> = self
            .database_connection
            .query("SELECT * FROM type::table($table) where refresh_hash = $hash LIMIT 1")
            .bind(("table", SessionModel::table_name()))
            .bind(("hash", refresh_token_hash))
            .await
            .map_err(AuthenticationServiceError::from_error)?
            .take(0)
            .map_err(|_| {
                AuthenticationServiceError::client(AuthenticationClientError::SessionNotFound)
            })?;

        session_vec
            .into_iter()
            .next()
            .ok_or(AuthenticationServiceError::client(
                AuthenticationClientError::SessionNotFound,
            ))
    }

    pub async fn create_session(
        &self,
        token_service: &TokenService,
        account_id: &BaseId,
        request_info: RequestInfoExtractor,
        service: String,
    ) -> Result<AuthenticationResponseDto, AuthenticationServiceError> {
        let refresh_token = token_service.generate_refresh_token();
        let refresh_token_hash = token_service.hash_refresh_token(&refresh_token);
        let refresh_expires_at = chrono::Utc::now()
            + chrono::Duration::days(
                self.authentication_config.refresh_token_expiration_days as i64,
            );

        let create_session: Vec<SessionModel> = self
            .database_connection
            .insert(SessionModel::table_name())
            .content(CreateSessionOptions {
                account_id: account_id.clone(),
                is_active: true,
                ip_address: request_info.ip_address,
                user_agent: request_info.user_agent,
                expires_at: BaseDateTime::from(refresh_expires_at),
                refresh_hash: refresh_token_hash,
            })
            .await
            .map_err(AuthenticationServiceError::from_error)?;

        if create_session.is_empty() {
            return Err(AuthenticationServiceError::client(
                AuthenticationClientError::SessionNotFound,
            ));
        }

        let session = &create_session[0];
        let (access_token, access_token_expires_at) =
            token_service.generate_jwt(TokenOpts::new(
                AccountModel::to_named_format(account_id),
                SessionModel::to_named_format(&session.id),
                service,
            ))?;

        Ok(AuthenticationResponseDto {
            account_id: AccountModel::to_named_format(account_id),
            session_id: SessionModel::to_named_format(&session.id),
            access_token,
            refresh_token: refresh_token.to_string(),
            refresh_token_expires_at: refresh_expires_at,
            access_token_expires_at,
        })
    }

    pub async fn activate_session(
        &self,
        session_id: &BaseId,
    ) -> Result<bool, AuthenticationServiceError> {
        let sessions: Vec<SessionModel> = self.database_connection
            .query("UPDATE type::table($table) SET is_active = true WHERE id = $id AND is_active = false RETURN AFTER")
            .bind(("table", SessionModel::table_name()))
            .bind(("id", session_id.clone()))
            .await.map_err(AuthenticationServiceError::from_error)?
            .take(0).map_err(|_| AuthenticationServiceError::client(AuthenticationClientError::SessionNotFound))?;

        Ok(!sessions.is_empty())
    }

    pub async fn activate_session_for_account(
        &self,
        session_id: &BaseId,
        account_id: &BaseId,
    ) -> Result<bool, AuthenticationServiceError> {
        let sessions: Vec<SessionModel> = self.database_connection
            .query("UPDATE type::table($table) SET is_active = true WHERE id = $id AND account_id = $account_id RETURN AFTER")
            .bind(("table", SessionModel::table_name()))
            .bind(("id", session_id.clone()))
            .bind(("account_id", account_id.clone()))
            .await.map_err(AuthenticationServiceError::from_error)?
            .take(0).map_err(|_| AuthenticationServiceError::client(AuthenticationClientError::SessionNotFound))?;

        Ok(!sessions.is_empty())
    }

    pub async fn deactivate_session(
        &self,
        session_id: &BaseId,
    ) -> Result<bool, AuthenticationServiceError> {
        let sessions: Vec<SessionModel> = self
            .database_connection
            .query("UPDATE type::table($table) SET is_active = false WHERE id = $id RETURN AFTER")
            .bind(("table", SessionModel::table_name()))
            .bind(("id", session_id.clone()))
            .await
            .map_err(AuthenticationServiceError::from_error)?
            .take(0)
            .map_err(|_| {
                AuthenticationServiceError::client(AuthenticationClientError::SessionNotFound)
            })?;

        Ok(!sessions.is_empty())
    }

    pub async fn deactivate_session_for_account(
        &self,
        session_id: &BaseId,
        account_id: &BaseId,
    ) -> Result<bool, AuthenticationServiceError> {
        let sessions: Vec<SessionModel> = self.database_connection
            .query("UPDATE type::table($table) SET is_active = false WHERE id = $id AND account_id = $account_id RETURN AFTER")
            .bind(("table", SessionModel::table_name()))
            .bind(("id", session_id.clone()))
            .bind(("account_id", account_id.clone()))
            .await.map_err(AuthenticationServiceError::from_error)?
            .take(0).map_err(|_| AuthenticationServiceError::client(AuthenticationClientError::SessionNotFound))?;

        Ok(!sessions.is_empty())
    }

    pub async fn deactivate_all_sessions_for_account(
        &self,
        account_id: &BaseId,
    ) -> Result<bool, AuthenticationServiceError> {
        let sessions: Vec<SessionModel> = self.database_connection
            .query("UPDATE type::table($table) SET is_active = false WHERE account_id = $account_id RETURN AFTER")
            .bind(("table", SessionModel::table_name()))
            .bind(("account_id", account_id.clone()))
            .await.map_err(AuthenticationServiceError::from_error)?
            .take(0).map_err(|_| AuthenticationServiceError::client(AuthenticationClientError::SessionNotFound))?;

        Ok(!sessions.is_empty())
    }

    pub async fn refresh_session(
        &self,
        token_service: &TokenService,
        refresh_token_hash: String,
        service: String,
    ) -> Result<AuthenticationResponseDto, AuthenticationServiceError> {
        let session = self
            .get_session_by_refresh_token_hash(refresh_token_hash)
            .await?;

        let refresh_token_expires_at = chrono::Utc::now()
            + chrono::Duration::days(
                self.authentication_config.refresh_token_expiration_days as i64,
            );

        let refresh_token = token_service.generate_refresh_token();
        let (access_token, access_token_expires_at) =
            token_service.generate_jwt(TokenOpts::new(
                AccountModel::to_named_format(&session.account_id),
                SessionModel::to_named_format(&session.id),
                service,
            ))?;

        Ok(AuthenticationResponseDto {
            account_id: AccountModel::to_named_format(&session.account_id),
            session_id: SessionModel::to_named_format(&session.id),
            access_token,
            refresh_token,
            refresh_token_expires_at,
            access_token_expires_at,
        })
    }

    pub async fn delete_session(
        &self,
        session_id: &BaseId,
    ) -> Result<(), AuthenticationServiceError> {
        let _: Option<SessionModel> = self
            .database_connection
            .delete(session_id)
            .await
            .map_err(AuthenticationServiceError::from_error)?;
        Ok(())
    }

    pub async fn delete_all_sessions_for_account(
        &self,
        account_id: &BaseId,
    ) -> Result<(), AuthenticationServiceError> {
        self.database_connection
            .query("DELETE FROM type::table($table) WHERE account_id = $account_id")
            .bind(("table", SessionModel::table_name()))
            .bind(("account_id", account_id.clone()))
            .await
            .map_err(AuthenticationServiceError::from_error)?;

        Ok(())
    }
}
