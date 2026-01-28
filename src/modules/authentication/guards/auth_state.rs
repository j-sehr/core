use crate::modules::{
    authentication::{
        guards::auth_services::AuthenticationServiceGuard, models::account::AccountModel,
    },
    base::exports::BaseId,
};
use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, header},
};
use serde_json::Value;

#[derive(Debug)]
enum AuthenticationKind {
    Authenticated {
        account_id: BaseId,
        session_id: BaseId,
    },
    RefreshToken {
        refresh_token_hash: String,
    },
    NotAuthenticated,
}

impl FromRequestParts<()> for AuthenticationKind {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &(),
    ) -> Result<Self, Self::Rejection> {
        let auth_svc_guard_res = AuthenticationServiceGuard::from_request_parts(parts, &()).await;
        if auth_svc_guard_res.is_err() {
            tracing::error!(
                "Authentication service guard error: {:?}",
                auth_svc_guard_res.err()
            );
            return Ok(AuthenticationKind::NotAuthenticated);
        }

        let auth_header = { parts.headers.get(header::AUTHORIZATION) };
        if auth_header.is_none() || auth_header.unwrap().to_str().is_err() {
            return Ok(AuthenticationKind::NotAuthenticated);
        }

        let auth_svc_guard = auth_svc_guard_res.unwrap();
        let token_service_res = auth_svc_guard.token_service();
        if token_service_res.is_err() {
            tracing::error!(
                "Token service retrieval error: {:?}",
                token_service_res.err()
            );
            return Ok(AuthenticationKind::NotAuthenticated);
        }

        let token_service = token_service_res.unwrap();

        let header_parts = auth_header.unwrap().to_str().unwrap().split_whitespace();
        let header_kind = header_parts.clone().next().unwrap_or("").trim();
        let header_value = header_parts.clone().nth(1).unwrap_or("").trim();

        match header_kind {
            "Bearer" => {
                if header_value.is_empty() {
                    return Ok(AuthenticationKind::NotAuthenticated);
                }

                let verify_res = token_service.verify_jwt(header_value);
                if verify_res.is_err() {
                    return Ok(AuthenticationKind::NotAuthenticated);
                }

                let (account_id, session_id) = verify_res.unwrap();
                Ok(AuthenticationKind::Authenticated {
                    account_id,
                    session_id,
                })
            }
            "Refresh" => {
                if header_value.is_empty() {
                    return Ok(AuthenticationKind::NotAuthenticated);
                }

                let hash = token_service.hash_refresh_token(header_value);
                Ok(AuthenticationKind::RefreshToken {
                    refresh_token_hash: hash,
                })
            }
            _ => Ok(AuthenticationKind::NotAuthenticated),
        }
    }
}

#[derive(Debug)]
pub struct AuthenticatedGuard {
    pub account_id: BaseId,
    #[allow(dead_code)]
    pub session_id: BaseId,
    pub account: AccountModel,
}

impl FromRequestParts<()> for AuthenticatedGuard {
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &(),
    ) -> Result<Self, Self::Rejection> {
        let auth_kind = AuthenticationKind::from_request_parts(parts, &()).await;
        let auth_svc_guard_res = AuthenticationServiceGuard::from_request_parts(parts, &()).await;
        if auth_svc_guard_res.is_err() {
            tracing::error!(
                "Authentication service guard error: {:?}",
                auth_svc_guard_res.err()
            );
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Unauthorized"})),
            ));
        }

        let auth_svc_guard = auth_svc_guard_res.unwrap();
        let account_service_res = auth_svc_guard.account_service();
        if account_service_res.is_err() {
            tracing::error!(
                "Account service retrieval error: {:?}",
                account_service_res.err()
            );

            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Unauthorized"})),
            ));
        }

        let account_service = account_service_res.unwrap();

        match auth_kind {
            Ok(AuthenticationKind::Authenticated {
                account_id,
                session_id,
            }) => {
                let account_res = account_service.get_account_by_id(&account_id).await;
                if account_res.is_err() {
                    tracing::debug!("Failed to fetch account: {:?}", account_res.err());
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(serde_json::json!({"error": "Unauthorized"})),
                    ));
                }

                let account = account_res.unwrap();

                Ok(AuthenticatedGuard {
                    account_id,
                    session_id,
                    account,
                })
            }
            _ => Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Unauthorized"})),
            )),
        }
    }
}

#[derive(Debug)]
pub struct RefreshTokenGuard {
    pub refresh_token_hash: String,
}

impl FromRequestParts<()> for RefreshTokenGuard {
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &(),
    ) -> Result<Self, Self::Rejection> {
        let auth_kind = AuthenticationKind::from_request_parts(parts, &()).await;
        match auth_kind {
            Ok(AuthenticationKind::RefreshToken { refresh_token_hash }) => {
                Ok(RefreshTokenGuard { refresh_token_hash })
            }
            _ => Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Unauthorized"})),
            )),
        }
    }
}

#[derive(Debug)]
pub struct NotAuthenticatedGuard;

impl FromRequestParts<()> for NotAuthenticatedGuard {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &(),
    ) -> Result<Self, Self::Rejection> {
        let auth_kind = AuthenticationKind::from_request_parts(parts, &()).await;
        match auth_kind {
            Ok(AuthenticationKind::NotAuthenticated) => Ok(NotAuthenticatedGuard),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct OptionalAuthenticatedGuard {
    pub account_id: Option<BaseId>,
    pub session_id: Option<BaseId>,
    pub is_authenticated: bool,
}

impl FromRequestParts<()> for OptionalAuthenticatedGuard {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &(),
    ) -> Result<Self, Self::Rejection> {
        let auth_kind = AuthenticationKind::from_request_parts(parts, &()).await;
        match auth_kind {
            Ok(AuthenticationKind::Authenticated {
                account_id,
                session_id,
            }) => Ok(OptionalAuthenticatedGuard {
                account_id: Some(account_id),
                session_id: Some(session_id),
                is_authenticated: true,
            }),
            _ => Ok(OptionalAuthenticatedGuard {
                account_id: None,
                session_id: None,
                is_authenticated: false,
            }),
        }
    }
}
