use crate::{
    common::model::DatabaseModel,
    modules::{
        authentication::{
            config::authentication::AuthenticationConfiguration,
            dtos::{authentication::AuthenticationResponseDto, session::CreateSessionOptions},
            models::{account::AccountModel, session::SessionModel},
            services::token::{TokenOpts, TokenService},
        },
        base::exports::{BaseDateTime, BaseId, DatabaseConnection},
    },
};
use chrono::DateTime;

const SERVICE_NAME: &str = "SessionService";

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

    pub async fn create_session(
        &self,
        token_service: &TokenService,
        account_id: &BaseId,
        service: String,
    ) -> anyhow::Result<AuthenticationResponseDto> {
        let refresh_token = token_service.generate_refresh_token()?;
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
                expires_at: BaseDateTime::from(refresh_expires_at),
                refresh_hash: refresh_token_hash,
            })
            .await?;

        if create_session.is_empty() {
            return Err(crate::log!(
                tracing::warn,
                "{} Failed to create a session",
                SERVICE_NAME
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
            access_token,
            refresh_token,
            refresh_token_expires_at: refresh_expires_at,
            access_token_expires_at: DateTime::from_timestamp_nanos(access_token_expires_at),
        })
    }

    pub async fn activate_session(&self, session_id: &BaseId) -> anyhow::Result<()> {
        self.database_connection
            .query("Update type::table($table) SET is_active = true WHERE id = $id && is_active = false")
            .bind(("table", SessionModel::table_name()))
            .bind(("id", session_id.clone()))
            .await?;

        Ok(())
    }

    pub async fn deactivate_session(&self, session_id: &BaseId) -> anyhow::Result<()> {
        self.database_connection
            .query("Update type::table($table) SET is_active = true WHERE id = $id ")
            .bind(("table", SessionModel::table_name()))
            .bind(("id", session_id.clone()))
            .await?;

        Ok(())
    }

    pub async fn get_session_by_id(
        &self,
        session_id: &BaseId,
    ) -> anyhow::Result<Option<SessionModel>> {
        let session: Option<SessionModel> = self.database_connection.select(session_id).await?;
        Ok(session)
    }

    pub async fn get_all_sessions_by_account_id(
        &self,
        account_id: &BaseId,
    ) -> anyhow::Result<Vec<SessionModel>> {
        let sessions: Vec<SessionModel> = self
            .database_connection
            .query("SELECT * FROM type::table($table) where account_id = $account_id")
            .bind(("table", SessionModel::table_name()))
            .bind(("account_id", account_id.clone()))
            .await?
            .take(0)
            .map_err(|e| {
                crate::log!(
                    tracing::warn,
                    "{} Failed get all sessions by account_id {}",
                    SERVICE_NAME,
                    e
                )
            })?;

        Ok(sessions)
    }

    pub async fn get_session_by_refresh_token_hash(
        &self,
        refresh_token_hash: String,
    ) -> anyhow::Result<Option<SessionModel>> {
        self.database_connection
            .query("SELECT * FROM type::table($table) where refresh_hash = $hash")
            .bind(("table", SessionModel::table_name()))
            .bind(("hash", refresh_token_hash))
            .await?
            .take(0)
            .map_err(|e| {
                crate::log!(
                    tracing::warn,
                    "{} Failed get one session by refresh_token {}",
                    SERVICE_NAME,
                    e
                )
            })
    }

    pub async fn refresh_session(
        &self,
        token_service: &TokenService,
        refresh_token_hash: String,
        service: String,
    ) -> anyhow::Result<AuthenticationResponseDto> {
        let session = self
            .get_session_by_refresh_token_hash(refresh_token_hash)
            .await?
            .ok_or_else(|| anyhow::anyhow!("{} Session not found", SERVICE_NAME))?;

        let refresh_token_expires_at = chrono::Utc::now()
            + chrono::Duration::days(
                self.authentication_config.refresh_token_expiration_days as i64,
            );

        let refresh_token = token_service.generate_refresh_token()?;
        let (access_token, expires_at_timestamp) = token_service.generate_jwt(TokenOpts::new(
            AccountModel::to_named_format(&session.account_id),
            SessionModel::to_named_format(&session.id),
            service,
        ))?;

        Ok(AuthenticationResponseDto {
            access_token,
            refresh_token,
            refresh_token_expires_at,
            access_token_expires_at: DateTime::from_timestamp_nanos(expires_at_timestamp),
        })
    }

    pub async fn delete_session(&self, session_id: &BaseId) -> anyhow::Result<()> {
        let _: Option<SessionModel> = self.database_connection.delete(session_id).await?;
        Ok(())
    }

    pub async fn delete_all_sessions_for_account_by_id(
        &self,
        account_id: &BaseId,
    ) -> anyhow::Result<()> {
        self.database_connection
            .query("DELETE FROM type::table($table) WHERE account_id = $account_id")
            .bind(("table", SessionModel::table_name()))
            .bind(("account_id", account_id.clone()))
            .await?;

        Ok(())
    }
}
