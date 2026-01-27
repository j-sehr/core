use crate::modules::{
    authentication::{
        dtos::{
            account::CreateAccountRequestDTO,
            authentication::{AuthenticationResponseDto, SignInRequestDto},
        },
        services::{
            account::AccountService, password::PasswordService, session::SessionService,
            token::TokenService,
        },
    },
    base::exports::BaseId,
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
        signin: SignInRequestDto,
    ) -> anyhow::Result<Result<AuthenticationResponseDto, String>> {
        let account_opt = account_service
            .get_account_by_username_and_password(
                pasword_service,
                &signin.username,
                &signin.password,
            )
            .await?;

        if account_opt.is_none() {
            return Ok(Err("Invalid credentials provided.".to_string()));
        }

        let account = account_opt.unwrap();

        #[allow(clippy::redundant_closure)] // false positive
        session_service
            .create_session(token_service, &account.id, "core-auth".to_string())
            .await
            .map(|auth| Ok(auth))
    }

    pub async fn register(
        &self,
        account_service: &AccountService,
        token_service: &TokenService,
        session_service: &SessionService,
        password_service: &PasswordService,
        signin: SignInRequestDto,
    ) -> anyhow::Result<Result<AuthenticationResponseDto, String>> {
        let account_res = account_service
            .create_account(
                password_service,
                CreateAccountRequestDTO {
                    password: signin.password,
                    username: signin.username,
                },
            )
            .await?;

        if account_res.is_err() {
            return Ok(Err(account_res.err().unwrap()));
        }

        let account = account_res.ok().unwrap();
        #[allow(clippy::redundant_closure)] // false positive
        session_service
            .create_session(token_service, &account.id, "core-auth".to_string())
            .await
            .map(|auth| Ok(auth))
    }

    pub async fn refresh_token(
        &self,
        session_service: &SessionService,
        token_service: &TokenService,
        refresh_token_hash: String,
    ) -> anyhow::Result<AuthenticationResponseDto> {
        session_service
            .refresh_session(token_service, refresh_token_hash, "core-auth".to_string())
            .await
    }

    pub async fn logout(
        &self,
        session_service: &SessionService,
        session_id: &BaseId,
    ) -> anyhow::Result<()> {
        session_service.delete_session(session_id).await
    }

    pub async fn logout_all(
        &self,
        session_service: &SessionService,
        account_id: &BaseId,
    ) -> anyhow::Result<()> {
        session_service
            .delete_all_sessions_for_account_by_id(account_id)
            .await
    }

    pub async fn delete_account(
        &self,
        account_service: &AccountService,
        session_service: &SessionService,
        account_id: &BaseId,
    ) -> anyhow::Result<()> {
        session_service
            .delete_all_sessions_for_account_by_id(account_id)
            .await?;
        account_service.delete_account(account_id).await?;

        Ok(())
    }
}
