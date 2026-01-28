use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthenticationServiceError {
    #[error("Server error: {0}")]
    ServerError(#[from] anyhow::Error),
    #[error("{0}")]
    ClientError(#[from] AuthenticationClientError),
}

#[derive(Error, Debug)]
pub enum AuthenticationClientError {
    #[error("Invalid credentials provided.")]
    InvalidCredentials,

    #[error("User account is locked.")]
    AccountLocked,
    #[error("User account not found.")]
    AccountNotFound,
    #[error("User account already exists.")]
    AccountAlreadyExists,

    #[error("Insufficient refresh token")]
    InvalidRefreshToken,
    #[error("Expired refresh token")]
    ExpiredRefreshToken,
    #[error("Invalid access token")]
    InvalidAccessToken,
    #[error("Expired access token")]
    ExpiredAccessToken,

    #[error("Invalid session Id")]
    InvalidSessionId,
    #[error("Invalid account Id")]
    InvalidAccountId,

    #[error("Authentication required.")]
    AuthenticationRequired,

    #[error("Session not found.")]
    SessionNotFound,
}

impl AuthenticationClientError {
    pub fn is_authentication_error(&self) -> bool {
        matches!(
            self,
            AuthenticationClientError::InvalidCredentials
                | AuthenticationClientError::InvalidAccessToken
                | AuthenticationClientError::ExpiredAccessToken
                | AuthenticationClientError::AuthenticationRequired
        )
    }

    pub fn is_refresh_token_error(&self) -> bool {
        matches!(
            self,
            AuthenticationClientError::InvalidRefreshToken
                | AuthenticationClientError::ExpiredRefreshToken
        )
    }

    pub fn is_account_error(&self) -> bool {
        matches!(
            self,
            AuthenticationClientError::AccountLocked | AuthenticationClientError::AccountNotFound
        )
    }

    pub fn is_session_error(&self) -> bool {
        matches!(self, AuthenticationClientError::SessionNotFound)
    }
}

impl AuthenticationServiceError {
    pub fn from_error(e: impl Into<anyhow::Error>) -> Self {
        AuthenticationServiceError::ServerError(e.into())
    }

    pub fn client(client_error: AuthenticationClientError) -> Self {
        AuthenticationServiceError::ClientError(client_error)
    }

    pub fn is_client_error(&self) -> bool {
        matches!(self, AuthenticationServiceError::ClientError(_))
    }

    pub fn is_server_error(&self) -> bool {
        matches!(self, AuthenticationServiceError::ServerError(_))
    }
}
