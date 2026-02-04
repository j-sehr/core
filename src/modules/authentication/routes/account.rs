use crate::{
    common::model::DatabaseModel,
    error_return,
    modules::authentication::{
        auth_services::AuthenticationServiceGuard, auth_state::AuthenticatedGuard,
        dtos::account::*, errors::service::*, models::account::AccountModel,
    },
};

use axum::{Json, extract::Path, http::StatusCode};
use serde_json::{Value, json};

#[axum::debug_handler()]
async fn self_get_account(account_session: AuthenticatedGuard) -> (StatusCode, Json<Value>) {
    let account = account_session.account;
    let dto = AccountDTO::from(&account);

    (StatusCode::OK, Json(json!({"account": dto})))
}

#[axum::debug_handler()]
async fn self_delete_account(
    auth_services: AuthenticationServiceGuard,
    account_session: AuthenticatedGuard,
) -> (StatusCode, Json<Value>) {
    error_return!(let (
        authentication_service,
        account_service,
        _password_service,
        session_service,
        _token_service,
    ) = auth_services.authentication_service_with_deps());

    error_return!(
        authentication_service
            .delete_account(
                &account_service,
                &session_service,
                &account_session.account.id,
            )
            .await
    );

    (
        StatusCode::OK,
        Json(json!({"message": "Account deleted successfully"})),
    )
}

#[axum::debug_handler()]
async fn self_update_account(
    auth_services: AuthenticationServiceGuard,
    account_session: AuthenticatedGuard,
    Json(dto): Json<UpdateAccountRequestDTO>,
) -> (StatusCode, Json<Value>) {
    let account_id = account_session.account_id;
    error_return!(let account_service = auth_services.account_service());

    if let Some(username) = dto.username {
        error_return!(
            account_service
                .update_account_username(&account_id, &username)
                .await
        );
    }

    if let Some(password) = dto.password {
        error_return!( let password_service = auth_services.password_service());
        error_return!(
            account_service
                .update_account_password(&password_service, &account_id, &password)
                .await
        );
    }

    (
        StatusCode::OK,
        Json(json!({"message": "Account updated successfully"})),
    )
}

#[axum::debug_handler()]
async fn list_all_accounts(
    auth_services: AuthenticationServiceGuard,
    _: AuthenticatedGuard,
) -> (StatusCode, Json<Value>) {
    error_return!(let account_service = auth_services.account_service());
    error_return!(let accounts = account_service.get_all_accounts().await);

    let account_dtos: Vec<AccountDTO> = accounts.into_iter().map(AccountDTO::from).collect();

    (StatusCode::OK, Json(json!({"accounts": account_dtos})))
}

#[axum::debug_handler()]
async fn get_account_by_id(
    auth_services: AuthenticationServiceGuard,
    _: AuthenticatedGuard,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    error_return!(let account_id = AccountModel::from_named_format(&id).ok_or(AuthenticationServiceError::client(AuthenticationClientError::InvalidAccountId)));
    error_return!(let account_service = auth_services.account_service());
    error_return!(let account = account_service.get_account_by_id(&account_id).await);

    let account_dto = AccountDTO::from(&account);

    (StatusCode::OK, Json(json!({"account": account_dto})))
}

#[axum::debug_handler()]
async fn delete_account_by_id(
    auth_services: AuthenticationServiceGuard,
    _: AuthenticatedGuard,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    error_return!(let account_id = AccountModel::from_named_format(&id).ok_or(AuthenticationServiceError::client(AuthenticationClientError::InvalidAccountId)));
    error_return!(let (
        authentication_service,
        account_service,
        _password_service,
        session_service,
        _token_service,
    ) = auth_services.authentication_service_with_deps());

    error_return!(
        authentication_service
            .delete_account(&account_service, &session_service, &account_id)
            .await
    );

    (
        StatusCode::OK,
        Json(json!({"message": "Account deleted successfully"})),
    )
}

#[axum::debug_handler()]
async fn update_account_by_id(
    auth_services: AuthenticationServiceGuard,
    _: AuthenticatedGuard,
    Path(id): Path<String>,
    Json(dto): Json<UpdateAccountRequestDTO>,
) -> (StatusCode, Json<Value>) {
    error_return!(let account_id = AccountModel::from_named_format(&id).ok_or(AuthenticationServiceError::client(AuthenticationClientError::InvalidAccountId)));
    error_return!(let account_service = auth_services.account_service());
    if let Some(username) = dto.username {
        error_return!(
            account_service
                .update_account_username(&account_id, &username)
                .await
        );
    }

    if let Some(password) = dto.password {
        error_return!( let password_service = auth_services.password_service());
        error_return!(
            account_service
                .update_account_password(&password_service, &account_id, &password)
                .await
        );
    }

    (
        StatusCode::OK,
        Json(json!({"message": "Account updated successfully"})),
    )
}

pub fn routes() -> axum::Router {
    axum::Router::new()
        .route("/me", axum::routing::get(self_get_account))
        .route("/me", axum::routing::delete(self_delete_account))
        .route("/me", axum::routing::patch(self_update_account))
        .route("/all", axum::routing::get(list_all_accounts))
        .route("/{id}", axum::routing::get(get_account_by_id))
        .route("/{id}", axum::routing::patch(update_account_by_id))
        .route("/{id}", axum::routing::delete(delete_account_by_id))
}
