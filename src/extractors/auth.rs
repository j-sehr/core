use crate::{
    connection::{self, DbState},
    models::{account::Account, prelude::BaseId},
    token,
};
use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, header},
};
use serde_json::Value;

#[derive(Debug)]
enum AuthenticationKind {
    Authenticated { user_id: BaseId, session_id: BaseId },
    RefreshToken { refresh_token_hash: String },
    NotAuthenticated,
}

impl FromRequestParts<DbState> for AuthenticationKind {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        db: &DbState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get(header::AUTHORIZATION);
        if auth_header.is_none() || auth_header.unwrap().to_str().is_err() {
            return Ok(AuthenticationKind::NotAuthenticated);
        }

        let header_parts = auth_header.unwrap().to_str().unwrap().split_whitespace();
        let header_kind = header_parts.clone().next().unwrap_or("").trim();
        let header_value = header_parts.clone().nth(1).unwrap_or("").trim();

        match header_kind {
            "Bearer" => {
                if header_value.is_empty() {
                    return Ok(AuthenticationKind::NotAuthenticated);
                }

                let verify_res = token::verify_jwt(header_value);
                if verify_res.is_err() {
                    return Ok(AuthenticationKind::NotAuthenticated);
                }

                let (account_id, session_id) = verify_res.unwrap();
                let session_res = connection::get_session_by_id(db, session_id.clone()).await;
                if session_res.is_err() {
                    return Ok(AuthenticationKind::NotAuthenticated);
                }

                let session_opt = session_res.unwrap();
                if session_opt.is_none() {
                    return Ok(AuthenticationKind::NotAuthenticated);
                }

                let session = session_opt.unwrap();
                if session.account_id != account_id || !session.is_active {
                    return Ok(AuthenticationKind::NotAuthenticated);
                }

                Ok(AuthenticationKind::Authenticated {
                    user_id: account_id,
                    session_id,
                })
            }
            "Refresh" => {
                if header_value.is_empty() {
                    return Ok(AuthenticationKind::NotAuthenticated);
                }

                let hash = token::hash_refresh_token(header_value);
                Ok(AuthenticationKind::RefreshToken {
                    refresh_token_hash: hash,
                })
            }
            _ => Ok(AuthenticationKind::NotAuthenticated),
        }
    }
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: BaseId,
    #[allow(dead_code)]
    pub session_id: BaseId,
    pub account: Account,
}

impl FromRequestParts<DbState> for AuthenticatedUser {
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        db: &DbState,
    ) -> Result<Self, Self::Rejection> {
        let auth_kind = AuthenticationKind::from_request_parts(parts, db).await;
        match auth_kind {
            Ok(AuthenticationKind::Authenticated {
                user_id,
                session_id,
            }) => {
                let account_res = connection::get_account_by_id(db, user_id.clone()).await;
                if account_res.is_err() {
                    dbg!("Failed to fetch account: {:?}", account_res.err());
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(serde_json::json!({"error": "Unauthorized"})),
                    ));
                }

                let account = account_res.unwrap();
                if account.is_none() {
                    dbg!("Account not found for user_id: {}", user_id);
                    let delete_session_res =
                        connection::delete_session(db, session_id.clone()).await;
                    if delete_session_res.is_err() {
                        dbg!(
                            "Failed to delete session for session_id {}: {:?}",
                            session_id,
                            delete_session_res.err()
                        );
                    }

                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(serde_json::json!({"error": "Unauthorized"})),
                    ));
                }

                Ok(AuthenticatedUser {
                    user_id,
                    session_id,
                    account: account.unwrap(),
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
pub struct RefreshToken {
    pub refresh_token_hash: String,
}

impl FromRequestParts<DbState> for RefreshToken {
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        db: &DbState,
    ) -> Result<Self, Self::Rejection> {
        let auth_kind = AuthenticationKind::from_request_parts(parts, db).await;
        match auth_kind {
            Ok(AuthenticationKind::RefreshToken { refresh_token_hash }) => {
                Ok(RefreshToken { refresh_token_hash })
            }
            _ => Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Unauthorized"})),
            )),
        }
    }
}

#[derive(Debug)]
pub struct NotAuthenticated;

impl FromRequestParts<DbState> for NotAuthenticated {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        db: &DbState,
    ) -> Result<Self, Self::Rejection> {
        let auth_kind = AuthenticationKind::from_request_parts(parts, db).await;
        match auth_kind {
            Ok(AuthenticationKind::NotAuthenticated) => Ok(NotAuthenticated),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct OptionalAuthenticatedUser {
    pub user_id: Option<BaseId>,
    pub session_id: Option<BaseId>,
    pub is_authenticated: bool,
}

impl FromRequestParts<DbState> for OptionalAuthenticatedUser {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        db: &DbState,
    ) -> Result<Self, Self::Rejection> {
        let auth_kind = AuthenticationKind::from_request_parts(parts, db).await;
        match auth_kind {
            Ok(AuthenticationKind::Authenticated {
                user_id,
                session_id,
            }) => Ok(OptionalAuthenticatedUser {
                user_id: Some(user_id),
                session_id: Some(session_id),
                is_authenticated: true,
            }),
            _ => Ok(OptionalAuthenticatedUser {
                user_id: None,
                session_id: None,
                is_authenticated: false,
            }),
        }
    }
}
