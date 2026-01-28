use crate::{
    common::model::DatabaseModel,
    error_return,
    modules::authentication::{
        account_model::AccountModel,
        auth_services::AuthenticationServiceGuard,
        auth_state::{AuthenticatedGuard, RefreshTokenGuard},
        errors::service::*,
        session_dto::SessionDTO,
        session_model::SessionModel,
    },
};
use axum::{Json, extract::Path, http::StatusCode};
use serde_json::{Value, json};

#[axum::debug_handler()]
async fn refresh_session(
    auth_services: AuthenticationServiceGuard,
    refresh: RefreshTokenGuard,
) -> (StatusCode, Json<Value>) {
    error_return!(let (session_service, token_service) = auth_services.session_service_with_deps());
    error_return!(let auth_response = session_service
        .refresh_session(
            &token_service,
            refresh.refresh_token_hash,
            "core-auth".to_string(),
        )
        .await);

    (StatusCode::OK, Json(json!(auth_response)))
}

#[axum::debug_handler()]
async fn self_list_sessions(
    auth_services: AuthenticationServiceGuard,
    account_session: AuthenticatedGuard,
) -> (StatusCode, Json<Value>) {
    let account_id = account_session.account_id;
    error_return!(let (session_service, _token_service) = auth_services.session_service_with_deps());

    error_return!(let sessions = session_service
        .get_all_sessions_for_account(&account_id)
        .await);

    (
        StatusCode::OK,
        Json(
            json!({"sessions": sessions.into_iter().map(SessionDTO::from).collect::<Vec<SessionDTO>>()}),
        ),
    )
}

#[axum::debug_handler()]
async fn self_revoke_all_sessions(
    account_session: AuthenticatedGuard,
    auth_services: AuthenticationServiceGuard,
) -> (StatusCode, Json<Value>) {
    error_return!(let (session_service, _token_service) = auth_services.session_service_with_deps());
    let account_id = account_session.account_id;

    error_return!(
        session_service
            .deactivate_all_sessions_for_account(&account_id)
            .await
    );

    (
        StatusCode::OK,
        Json(json!({"message": "All sessions revoked successfully"})),
    )
}

#[axum::debug_handler()]
async fn self_revoke_session(
    _: AuthenticatedGuard,
    auth_services: AuthenticationServiceGuard,
    Path(session_id): Path<String>,
) -> (StatusCode, Json<Value>) {
    error_return!(let session_id = SessionModel::from_named_format(&session_id)
        .ok_or(AuthenticationServiceError::client(AuthenticationClientError::InvalidSessionId))
    );

    error_return!(let (session_service, _token_service) = auth_services.session_service_with_deps());
    error_return!(session_service.deactivate_session(&session_id).await);

    (
        StatusCode::OK,
        Json(json!({"message": "Session revoked successfully"})),
    )
}

#[axum::debug_handler()]
async fn list_sessions_for_account(
    _: AuthenticatedGuard,
    auth_services: AuthenticationServiceGuard,
    Path(account_id): Path<String>,
) -> (StatusCode, Json<Value>) {
    error_return!(let account_id = AccountModel::from_named_format(&account_id)
        .ok_or(AuthenticationServiceError::client(AuthenticationClientError::InvalidAccountId))
    );

    error_return!(let (session_service, _token_service) = auth_services.session_service_with_deps());
    error_return!(let sessions = session_service
        .get_all_sessions_for_account(&account_id)
        .await);

    (
        StatusCode::OK,
        Json(
            json!({"sessions": sessions.into_iter().map(SessionDTO::from).collect::<Vec<SessionDTO>>()}),
        ),
    )
}

#[axum::debug_handler()]
async fn revoke_all_sessions_by_account_id(
    _: AuthenticatedGuard,
    auth_services: AuthenticationServiceGuard,
    Path(account_id): Path<String>,
) -> (StatusCode, Json<Value>) {
    error_return!(let account_id = AccountModel::from_named_format(&account_id)
        .ok_or(AuthenticationServiceError::client(AuthenticationClientError::InvalidAccountId))
    );

    error_return!(let (session_service, _token_service) = auth_services.session_service_with_deps());
    error_return!(
        session_service
            .deactivate_all_sessions_for_account(&account_id)
            .await
    );

    (
        StatusCode::OK,
        Json(json!({"message": "All sessions revoked successfully"})),
    )
}

async fn revoke_session_for_account_by_id(
    _: AuthenticatedGuard,
    auth_services: AuthenticationServiceGuard,
    Path((account_id, session_id)): Path<(String, String)>,
) -> (StatusCode, Json<Value>) {
    error_return!(let session_id = SessionModel::from_named_format(&session_id)
        .ok_or(AuthenticationServiceError::client(AuthenticationClientError::InvalidSessionId))
    );

    error_return!(let account_id = AccountModel::from_named_format(&account_id)
        .ok_or(AuthenticationServiceError::client(AuthenticationClientError::InvalidAccountId))
    );

    error_return!(let (session_service, _token_service) = auth_services.session_service_with_deps());
    error_return!(
        session_service
            .deactivate_session_for_account(&session_id, &account_id)
            .await
    );

    (
        StatusCode::OK,
        Json(json!({"message": "Session revoked successfully"})),
    )
}

#[axum::debug_handler()]
async fn list_all_sessions(
    _: AuthenticatedGuard,
    auth_services: AuthenticationServiceGuard,
) -> (StatusCode, Json<Value>) {
    error_return!(let (session_service, _token_service) = auth_services.session_service_with_deps());
    error_return!(let sessions = session_service.get_all_sessions().await);
    let sessions_dto: Vec<SessionDTO> = sessions.into_iter().map(SessionDTO::from).collect();

    (StatusCode::OK, Json(json!({"sessions": sessions_dto})))
}

pub fn routes() -> axum::Router {
    axum::Router::new()
        .route("/refresh", axum::routing::post(refresh_session))
        .route("/self", axum::routing::get(self_list_sessions))
        .route(
            "/self/revoke",
            axum::routing::patch(self_revoke_all_sessions),
        )
        .route(
            "/self/revoke/{session_id}",
            axum::routing::patch(self_revoke_session),
        )
        .route("/all", axum::routing::get(list_all_sessions))
        .route(
            "/{account_id}",
            axum::routing::get(list_sessions_for_account),
        )
        .route(
            "/{account_id}/revoke",
            axum::routing::patch(revoke_all_sessions_by_account_id),
        )
        .route(
            "/{account_id}/revoke/{session_id}",
            axum::routing::patch(revoke_session_for_account_by_id),
        )
}
