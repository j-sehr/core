use crate::{
    common::app_state::AppContext,
    config::{enviroment::EnviromentConfiguration, file::FileConfiguration},
    modules::{
        authentication::{
            config::authentication::AuthenticationConfiguration,
            errors::service::AuthenticationServiceError,
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
    fn auth_config(&self) -> Result<AuthenticationConfiguration, AuthenticationServiceError> {
        self.file_config
            .get_as::<AuthenticationConfiguration>()
            .ok_or_else(|| {
                AuthenticationServiceError::ServerError(crate::log!(
                    tracing::error,
                    "Failed to load authentication configuration"
                ))
            })
    }

    pub fn token_service(&self) -> Result<TokenService, AuthenticationServiceError> {
        self.auth_config().map(TokenService::new)
    }

    pub fn password_service(&self) -> Result<PasswordService, AuthenticationServiceError> {
        Ok(PasswordService)
    }

    pub fn account_service(&self) -> Result<AccountService, AuthenticationServiceError> {
        Ok(AccountService::new(self.database_connection.clone()))
    }

    pub fn account_service_with_deps(
        &self,
    ) -> Result<(AccountService, PasswordService), AuthenticationServiceError> {
        let account_service = AccountService::new(self.database_connection.clone());
        let password_service = PasswordService;

        Ok((account_service, password_service))
    }

    pub fn session_service(&self) -> Result<SessionService, AuthenticationServiceError> {
        let auth_config = self.auth_config()?;

        Ok(SessionService::new(
            auth_config,
            self.database_connection.clone(),
        ))
    }

    pub fn session_service_with_deps(
        &self,
    ) -> Result<(SessionService, TokenService), AuthenticationServiceError> {
        let auth_config = self.auth_config()?;
        let token_service = TokenService::new(auth_config.clone());

        let session_service = SessionService::new(auth_config, self.database_connection.clone());

        Ok((session_service, token_service))
    }

    pub fn authentication_service(
        &self,
    ) -> Result<AuthenticationService, AuthenticationServiceError> {
        Ok(AuthenticationService::new())
    }

    pub fn authentication_service_with_deps(
        &self,
    ) -> Result<
        (
            AuthenticationService,
            AccountService,
            PasswordService,
            SessionService,
            TokenService,
        ),
        AuthenticationServiceError,
    > {
        let (account_service, password_service) = self.account_service_with_deps()?;
        let (session_service, token_service) = self.session_service_with_deps()?;
        let authentication_service = AuthenticationService::new();

        Ok((
            authentication_service,
            account_service,
            password_service,
            session_service,
            token_service,
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
