use crate::{
    common::app_state::AppContext,
    config::{enviroment::EnviromentConfiguration, file::FileConfiguration},
    modules::{
        authentication::{
            config::authentication::AuthenticationConfiguration,
            services::{
                account::AccountService, authentication::AuthenticationService,
                password::PasswordService, session::SessionService, token::TokenService,
            },
        },
        base::exports::DatabaseConnection,
    },
};
use axum::extract::FromRequestParts;

const GUARD_NAME: &str = "AuthenticationServiceGuard";

#[derive(Debug, Clone)]
pub struct AuthenticationServiceGuard {
    database_connection: DatabaseConnection,
    #[allow(dead_code)]
    env_config: EnviromentConfiguration,
    file_config: FileConfiguration,
}

impl AuthenticationServiceGuard {
    fn auth_config(&self) -> anyhow::Result<AuthenticationConfiguration> {
        self.file_config
            .get_as::<AuthenticationConfiguration>()
            .ok_or_else(|| {
                crate::log!(
                    tracing::error,
                    "{} Failed to load authentication config",
                    GUARD_NAME
                )
            })
    }

    pub fn token_service(&self) -> anyhow::Result<TokenService> {
        self.auth_config().map(TokenService::new)
    }

    pub fn password_service(&self) -> anyhow::Result<PasswordService> {
        Ok(PasswordService)
    }

    pub fn account_service(&self) -> anyhow::Result<AccountService> {
        Ok(AccountService::new(self.database_connection.clone()))
    }

    pub fn session_service(&self) -> anyhow::Result<SessionService> {
        let auth_config = self.auth_config()?;

        Ok(SessionService::new(
            auth_config,
            self.database_connection.clone(),
        ))
    }

    pub fn authentication_service(&self) -> anyhow::Result<AuthenticationService> {
        Ok(AuthenticationService::new())
    }

    pub fn get_all_services(
        &self,
    ) -> anyhow::Result<(
        TokenService,
        PasswordService,
        AccountService,
        SessionService,
        AuthenticationService,
    )> {
        Ok((
            self.token_service()?,
            self.password_service()?,
            self.account_service()?,
            self.session_service()?,
            self.authentication_service()?,
        ))
    }
}

impl FromRequestParts<()> for AuthenticationServiceGuard {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &(),
    ) -> Result<Self, Self::Rejection> {
        let app_state_opt = &parts.extensions.get::<AppContext>();

        if app_state_opt.is_none() {
            tracing::error!("{} Failed to get AppContext from request parts", GUARD_NAME);
            return Err(());
        }

        let app_state = app_state_opt.unwrap();

        Ok(AuthenticationServiceGuard {
            database_connection: app_state.database.clone(),
            env_config: app_state.env_config.clone(),
            file_config: app_state.file_config.clone(),
        })
    }
}
