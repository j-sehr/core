use crate::{
    error_return,
    modules::{
        authentication::{
            auth_services::AuthenticationServiceGuard,
            auth_state::{AuthenticatedGuard, NotAuthenticatedGuard},
            authentication_dto::SignInRequestDto,
        },
        base::exports::request_info::RequestInfoExtractor,
    },
};
use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

#[axum::debug_handler()]
async fn sign_up(
    request_info: RequestInfoExtractor,
    auth_services: AuthenticationServiceGuard,
    Json(dto): Json<SignInRequestDto>,
) -> (StatusCode, Json<Value>) {
    error_return!(let (
        authentication_service,
        account_service,
        password_service,
        session_service,
        token_service,
    ) = auth_services.authentication_service_with_deps());

    error_return!(let auth_response = dbg!(authentication_service
        .register(
            &account_service,
            &token_service,
            &session_service,
            &password_service,
            request_info,
            dto,
        )
        .await));

    (StatusCode::CREATED, Json(json!(auth_response)))
}

#[axum::debug_handler()]
async fn sign_in(
    request_info: RequestInfoExtractor,
    auth_services: AuthenticationServiceGuard,
    _: NotAuthenticatedGuard,
    Json(dto): Json<SignInRequestDto>,
) -> (StatusCode, Json<Value>) {
    error_return!(let (
        authentication_service,
        account_service,
        password_service,
        session_service,
        token_service,
    ) = auth_services.authentication_service_with_deps());

    error_return!(let auth_response = authentication_service
        .authenticate(
            &account_service,
            &session_service,
            &token_service,
            &password_service,
            request_info,
            dto,
        )
        .await);

    (StatusCode::OK, Json(json!(auth_response)))
}

#[axum::debug_handler()]
async fn sign_out(
    auth_services: AuthenticationServiceGuard,
    account_session: AuthenticatedGuard,
) -> (StatusCode, Json<Value>) {
    error_return!(let (
        authentication_service,
        _account_service,
        _password_service,
        session_service,
        _token_service,
    ) = auth_services.authentication_service_with_deps());

    error_return!(
        authentication_service
            .logout(&session_service, &account_session.session_id)
            .await
    );

    (
        StatusCode::OK,
        Json(json!({"message": "Logged out successfully"})),
    )
}

pub fn routes() -> axum::Router {
    axum::Router::new()
        .route("/sign-up", axum::routing::post(sign_up))
        .route("/sign-in", axum::routing::post(sign_in))
        .route("/sign-out", axum::routing::post(sign_out))
}
