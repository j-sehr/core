use crate::{
    common::model::DatabaseModel,
    modules::authentication::{
        auth_services::AuthenticationServiceGuard,
        auth_state::{AuthenticatedGuard, NotAuthenticatedGuard, RefreshTokenGuard},
        authentication_dto::SignInRequestDto,
        dtos::account::*,
        models::account::AccountModel,
    },
};
use axum::{Json, extract::Path, http::StatusCode};
use serde_json::{Value, json};

#[axum::debug_handler()]
async fn create_account(
    auth_services: AuthenticationServiceGuard,
    Json(dto): Json<SignInRequestDto>,
) -> (StatusCode, Json<Value>) {
    let auth_svc_res = auth_services.get_all_services();
    if auth_svc_res.is_err() {
        tracing::error!(
            "Failed to get authentication services: {:?}",
            auth_svc_res.err()
        );

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create account"})),
        );
    }

    let (token_service, password_service, account_service, session_service, authentication_service) =
        auth_svc_res.unwrap();

    let register_res = authentication_service
        .register(
            &account_service,
            &token_service,
            &session_service,
            &password_service,
            dto,
        )
        .await;

    if register_res.is_err() {
        tracing::error!("Failed to register account: {:?}", register_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create account"})),
        );
    }

    let register_client_res = register_res.unwrap();
    if register_client_res.is_err() {
        tracing::debug!(
            "Failed to register account: {:?}",
            register_client_res.clone().err()
        );
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": register_client_res.err().unwrap().to_string()})),
        );
    }

    let auth_response = register_client_res.unwrap();

    (StatusCode::CREATED, Json(json!(auth_response)))
}

#[axum::debug_handler()]
async fn get_account(account_session: AuthenticatedGuard) -> (StatusCode, Json<Value>) {
    let account = account_session.account;
    let dto = AccountDTO {
        id: AccountModel::to_named_format(&account.id),
        username: account.username.clone(),
        updated_at: account.updated_at,
        created_at: account.created_at,
    };

    (StatusCode::OK, Json(json!({"account": dto})))
}

#[axum::debug_handler()]
async fn delete_account(
    auth_services: AuthenticationServiceGuard,
    account_session: AuthenticatedGuard,
) -> (StatusCode, Json<Value>) {
    let auth_svc_res = auth_services.get_all_services();
    if auth_svc_res.is_err() {
        tracing::error!(
            "Failed to get authentication services: {:?}",
            auth_svc_res.err()
        );

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create account"})),
        );
    }

    let (_token, _password, account_service, session_service, authentication_service) =
        auth_svc_res.unwrap();

    let delete_res = authentication_service
        .delete_account(
            &account_service,
            &session_service,
            &account_session.account.id,
        )
        .await;

    if delete_res.is_err() {
        tracing::error!("Failed to delete account: {:?}", delete_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to delete account"})),
        );
    }

    (
        StatusCode::OK,
        Json(json!({"message": "Account deleted successfully"})),
    )
}

#[axum::debug_handler()]
async fn update_account(
    auth_services: AuthenticationServiceGuard,
    account_session: AuthenticatedGuard,
    Json(dto): Json<UpdateAccountRequestDTO>,
) -> (StatusCode, Json<Value>) {
    let account_id = account_session.account_id;
    let account_service_res = auth_services.account_service();
    if account_service_res.is_err() {
        tracing::error!(
            "Failed to get account service: {:?}",
            account_service_res.err()
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update account"})),
        );
    }

    let account_service = account_service_res.unwrap();

    if let Some(username) = dto.username {
        let update_res = account_service
            .update_account_username(&account_id, &username)
            .await;
        if update_res.is_err() {
            tracing::error!("Failed to update username: {:?}", update_res.err());
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to update account"})),
            );
        }
    }

    if let Some(password) = dto.password {
        let password_service_res = auth_services.password_service();
        if password_service_res.is_err() {
            tracing::error!(
                "Failed to get password service: {:?}",
                password_service_res.err()
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to update account"})),
            );
        }

        let password_service = password_service_res.unwrap();
        let update_res = account_service
            .update_account_password(&password_service, &account_id, &password)
            .await;

        if update_res.is_err() {
            tracing::error!("Failed to update password: {:?}", update_res.err());
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to update account"})),
            );
        }
    }

    (
        StatusCode::OK,
        Json(json!({"message": "Account updated successfully"})),
    )
}

#[axum::debug_handler()]
async fn list_accounts(
    auth_services: AuthenticationServiceGuard,
    _: AuthenticatedGuard,
) -> (StatusCode, Json<Value>) {
    let account_service_res = auth_services.account_service();
    if account_service_res.is_err() {
        tracing::error!(
            "Failed to get account service: {:?}",
            account_service_res.err()
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to list accounts"})),
        );
    }

    let account_service = account_service_res.unwrap();
    let accounts_res = account_service.get_all_accounts().await;
    if accounts_res.is_err() {
        dbg!("Failed to list accounts: {:?}", accounts_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to list accounts"})),
        );
    }

    let accounts = accounts_res.unwrap();
    let account_dtos: Vec<AccountDTO> = accounts
        .into_iter()
        .map(|account| AccountDTO {
            id: AccountModel::to_named_format(&account.id),
            username: account.username.clone(),
            updated_at: account.updated_at,
            created_at: account.created_at,
        })
        .collect();

    (StatusCode::OK, Json(json!({"accounts": account_dtos})))
}

#[axum::debug_handler()]
async fn get_account_by_id(
    auth_services: AuthenticationServiceGuard,
    _: AuthenticatedGuard,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    let account_id_opt = AccountModel::from_named_format(&id);
    if account_id_opt.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid account ID format"})),
        );
    }

    let account_id = account_id_opt.unwrap();

    let account_service_res = auth_services.account_service();
    if account_service_res.is_err() {
        tracing::error!(
            "Failed to get account service: {:?}",
            account_service_res.err()
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to get account"})),
        );
    }

    let account_service = account_service_res.unwrap();

    let account_res = account_service.get_account_by_id(&account_id).await;
    if account_res.is_err() {
        tracing::warn!("Failed to get account by id: {:?}", account_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to get account"})),
        );
    }

    let account_opt = account_res.unwrap();
    if account_opt.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Account not found"})),
        );
    }

    let account = account_opt.unwrap();
    let account_dto = AccountDTO {
        id: AccountModel::to_named_format(&account.id),
        username: account.username.clone(),
        updated_at: account.updated_at,
        created_at: account.created_at,
    };

    (StatusCode::OK, Json(json!({"account": account_dto})))
}

#[axum::debug_handler()]
async fn login_account(
    auth_services: AuthenticationServiceGuard,
    _: NotAuthenticatedGuard,
    Json(dto): Json<SignInRequestDto>,
) -> (StatusCode, Json<Value>) {
    let auth_svc_res = auth_services.get_all_services();
    if auth_svc_res.is_err() {
        tracing::error!(
            "Failed to get authentication services: {:?}",
            auth_svc_res.err()
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to login"})),
        );
    }
    let (token_service, password_service, account_service, session_service, authentication_service) =
        auth_svc_res.unwrap();

    let auth_server_res = authentication_service
        .authenticate(
            &account_service,
            &session_service,
            &token_service,
            &password_service,
            dto,
        )
        .await;

    if auth_server_res.is_err() {
        dbg!(
            "Failed to authenticate account: {:?}",
            auth_server_res.err()
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to authenticate account"})),
        );
    }

    let auth_client_res = auth_server_res.unwrap();
    if auth_client_res.is_err() {
        tracing::debug!(
            "Failed to authenticate account: {:?}",
            auth_client_res.clone().err()
        );
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid username or password"})),
        );
    }

    let auth_response = auth_client_res.unwrap();
    (StatusCode::OK, Json(json!(auth_response)))
}

#[axum::debug_handler()]
async fn logout_account(
    auth_services: AuthenticationServiceGuard,
    account_session: AuthenticatedGuard,
) -> (StatusCode, Json<Value>) {
    let auth_svc_res = auth_services.get_all_services();
    if auth_svc_res.is_err() {
        tracing::error!(
            "Failed to get authentication services: {:?}",
            auth_svc_res.err()
        );

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to logout"})),
        );
    }

    let (
        _token_service,
        _password_service,
        _account_service,
        session_service,
        authentication_service,
    ) = auth_svc_res.unwrap();

    let logout_res = authentication_service
        .logout(&session_service, &account_session.session_id)
        .await;

    if logout_res.is_err() {
        tracing::error!("Failed to logout: {:?}", logout_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to logout"})),
        );
    }

    (
        StatusCode::OK,
        Json(json!({"message": "Logged out successfully"})),
    )
}

#[axum::debug_handler()]
async fn refresh_session(
    auth_services: AuthenticationServiceGuard,
    refresh: RefreshTokenGuard,
) -> (StatusCode, Json<Value>) {
    let auth_svc_res = auth_services.get_all_services();
    if auth_svc_res.is_err() {
        tracing::error!(
            "Failed to get authentication services: {:?}",
            auth_svc_res.err()
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to refresh session"})),
        );
    }

    let (
        token_service,
        _password_service,
        _account_service,
        session_service,
        authentication_service,
    ) = auth_svc_res.unwrap();

    let refresh_session_res = authentication_service
        .refresh_token(&session_service, &token_service, refresh.refresh_token_hash)
        .await;

    if refresh_session_res.is_err() {
        tracing::debug!("Failed to get session: {:?}", refresh_session_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to refresh session"})),
        );
    }

    let auth_response = refresh_session_res.unwrap();

    (StatusCode::OK, Json(json!(auth_response)))
}

#[axum::debug_handler]
async fn health_check() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"status": "ok"})))
}

#[axum::debug_handler()]
async fn test(_: AuthenticatedGuard) -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({"message": "Authenticated access successful"})),
    )
}

pub fn router() -> axum::Router<()> {
    let router = axum::Router::new();
    router
        .route("/accounts/register", axum::routing::post(create_account))
        .route("/accounts", axum::routing::get(list_accounts))
        .route("/accounts/{id}", axum::routing::get(get_account_by_id))
        .route("/accounts/me", axum::routing::get(get_account))
        .route("/accounts/me", axum::routing::delete(delete_account))
        .route("/accounts/me", axum::routing::put(update_account))
        .route("/accounts/login", axum::routing::post(login_account))
        .route("/accounts/logout", axum::routing::get(logout_account))
        .route("/accounts/refresh", axum::routing::post(refresh_session))
        .route("/health", axum::routing::get(health_check))
        .route("/test", axum::routing::get(test))
}
