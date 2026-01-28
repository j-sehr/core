use crate::modules::{
    authentication::{
        dtos::{
            account::CreateAccountRequestDTO,
            authentication::{AuthenticationResponseDto, SignInRequestDto},
        },
        errors::service::AuthenticationServiceError,
        services::{
            account::AccountService, password::PasswordService, session::SessionService,
            token::TokenService,
        },
    },
    base::exports::{BaseId, request_info::RequestInfoExtractor},
};

#[derive(Debug, Clone)]
pub struct AuthenticationService {}

impl AuthenticationService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn authenticate(
        &self,
        account_service: &AccountService,
        session_service: &SessionService,
        token_service: &TokenService,
        pasword_service: &PasswordService,
        request_info: RequestInfoExtractor,
        signin: SignInRequestDto,
    ) -> Result<AuthenticationResponseDto, AuthenticationServiceError> {
        let account = account_service
            .get_account_by_username_and_password(
                pasword_service,
                &signin.username,
                &signin.password,
            )
            .await?;

        session_service
            .create_session(
                token_service,
                &account.id,
                request_info,
                "core-auth".to_string(),
            )
            .await
    }

    pub async fn register(
        &self,
        account_service: &AccountService,
        token_service: &TokenService,
        session_service: &SessionService,
        password_service: &PasswordService,
        request_info: RequestInfoExtractor,
        signin: SignInRequestDto,
    ) -> Result<AuthenticationResponseDto, AuthenticationServiceError> {
        tracing::debug!("Registering new user: {}", &signin.username);
        let account = account_service
            .create_account(
                password_service,
                CreateAccountRequestDTO {
                    password: signin.password,
                    username: signin.username,
                },
            )
            .await?;

        tracing::debug!("Creating session for new user: {}", &account.username);

        session_service
            .create_session(
                token_service,
                &account.id,
                request_info,
                "core-auth".to_string(),
            )
            .await
    }

    pub async fn logout(
        &self,
        session_service: &SessionService,
        session_id: &BaseId,
    ) -> Result<(), AuthenticationServiceError> {
        session_service.delete_session(session_id).await
    }

    pub async fn logout_all(
        &self,
        session_service: &SessionService,
        account_id: &BaseId,
    ) -> Result<(), AuthenticationServiceError> {
        session_service
            .delete_all_sessions_for_account(account_id)
            .await
    }

    pub async fn delete_account(
        // Here because we need to delete sessions as well
        &self,
        account_service: &AccountService,
        session_service: &SessionService,
        account_id: &BaseId,
    ) -> Result<(), AuthenticationServiceError> {
        session_service
            .delete_all_sessions_for_account(account_id)
            .await?;
        account_service.delete_account(account_id).await?;

        Ok(())
    }
}
