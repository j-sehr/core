use crate::{
    connection::{self, DbExtractor, DbState},
    extractors::auth::{AuthenticatedUser, NotAuthenticated, RefreshToken},
    models::account::*,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};
use std::sync::Arc;
use surrealdb::{RecordId, Surreal, engine::remote::ws::Client};

#[axum::debug_handler(state = DbState)]
pub async fn create_account(
    State(db): DbExtractor,
    Json(dto): Json<CreateAccountDTO>,
) -> (StatusCode, Json<Value>) {
    let account_query = db
        .query("SELECT * FROM accounts WHERE username = $username LIMIT 1")
        .bind(("username", dto.username.clone()))
        .await;

    if account_query.is_err() {
        dbg!("Failed to query account: {:?}", account_query.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create account"})),
        );
    }

    let account = account_query
        .unwrap()
        .take::<Option<Account>>(0)
        .unwrap_or(None);

    if account.is_some() {
        dbg!("Username already exists: {}", dto.username);
        return (
            StatusCode::CONFLICT,
            Json(json!({"error": "Username already exists"})),
        );
    }

    let created_account_res = connection::create_account(&db, dto).await;
    if created_account_res.is_err() {
        dbg!("Failed to create account: {:?}", created_account_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create account"})),
        );
    }

    (
        StatusCode::CREATED,
        Json(json!({"message": "Account created successfully"})),
    )
}

#[axum::debug_handler(state = DbState)]
pub async fn get_account(account_session: AuthenticatedUser) -> (StatusCode, Json<Value>) {
    let account = account_session.account;
    let dto = AccountDTO {
        id: account.id,
        username: account.username.clone(),
        updated_at: account.updated_at,
        created_at: account.created_at,
    };

    (StatusCode::OK, Json(json!({"account": dto})))
}

#[axum::debug_handler(state = DbState)]
pub async fn delete_account(
    State(db): DbExtractor,
    account_session: AuthenticatedUser,
) -> (StatusCode, Json<Value>) {
    let user_id = account_session.user_id;

    let delete_account_res = connection::delete_account(&db, user_id).await;
    if delete_account_res.is_err() {
        dbg!("Failed to delete account: {:?}", delete_account_res.err());
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

#[axum::debug_handler(state = DbState)]
pub async fn update_account(
    State(db): DbExtractor,
    account_session: AuthenticatedUser,
    Json(dto): Json<UpdateAccountDTO>,
) -> (StatusCode, Json<Value>) {
    let user_id = account_session.user_id;

    let update_account_res = connection::update_account(&db, user_id, dto).await;
    if update_account_res.is_err() {
        dbg!("Failed to update account: {:?}", update_account_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update account"})),
        );
    }

    (
        StatusCode::OK,
        Json(json!({"message": "Account updated successfully"})),
    )
}

#[axum::debug_handler(state = DbState)]
pub async fn list_accounts(State(db): DbExtractor) -> (StatusCode, Json<Value>) {
    let accounts_res = connection::list_accounts(&db).await;
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
            id: account.id,
            username: account.username.clone(),
            updated_at: account.updated_at,
            created_at: account.created_at,
        })
        .collect();

    (StatusCode::OK, Json(json!({"accounts": account_dtos})))
}
#[axum::debug_handler(state = DbState)]
pub async fn get_account_by_id(
    State(db): DbExtractor,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    let account_res =
        connection::get_account_by_id(&db, RecordId::from_table_key("accounts", id)).await;
    if account_res.is_err() {
        dbg!("Failed to get account by id: {:?}", account_res.err());
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
        id: account.id,
        username: account.username.clone(),
        updated_at: account.updated_at,
        created_at: account.created_at,
    };

    (StatusCode::OK, Json(json!({"account": account_dto})))
}

#[axum::debug_handler(state = DbState)]
pub async fn login_account(
    State(db): DbExtractor,
    _: NotAuthenticated,
    Json(dto): Json<LoginDTO>,
) -> (StatusCode, Json<Value>) {
    let account_res = connection::authenticate_account(&db, &dto.username, &dto.password).await;
    if account_res.is_err() {
        dbg!("Failed to authenticate account: {:?}", account_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to authenticate account"})),
        );
    }

    if account_res.as_ref().unwrap().is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid username or password"})),
        );
    }

    let account = account_res.unwrap().unwrap();
    let session_res = connection::create_session(&db, account.id).await;
    if session_res.is_err() {
        dbg!("Failed to create session: {:?}", session_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create session"})),
        );
    }

    (
        StatusCode::OK,
        Json(json!({"session": session_res.unwrap()})),
    )
}

#[axum::debug_handler(state = DbState)]
pub async fn logout_account(
    State(db): DbExtractor,
    user: AuthenticatedUser,
) -> (StatusCode, Json<Value>) {
    let session_res = connection::get_session_by_id(&db, user.session_id).await;
    if session_res.is_err() {
        dbg!("Failed to get session: {:?}", session_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to logout"})),
        );
    }

    if session_res.as_ref().unwrap().is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid refresh token"})),
        );
    }

    let delete_session_res =
        connection::delete_session(&db, session_res.unwrap().unwrap().id).await;
    if delete_session_res.is_err() {
        dbg!("Failed to delete session: {:?}", delete_session_res.err());
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

#[axum::debug_handler(state = DbState)]
pub async fn refresh_session(
    State(db): DbExtractor,
    refresh: RefreshToken,
) -> (StatusCode, Json<Value>) {
    let refresh_token_hash = refresh.refresh_token_hash;

    let session_res = connection::get_session_by_refresh_token(&db, &refresh_token_hash).await;
    if session_res.is_err() {
        dbg!("Failed to get session: {:?}", session_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to refresh session"})),
        );
    }

    if session_res.as_ref().unwrap().is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid refresh token"})),
        );
    }

    let session_res = connection::refresh_session(&db, &refresh_token_hash).await;
    if session_res.is_err() {
        dbg!("Failed to create new session: {:?}", session_res.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to refresh session"})),
        );
    }

    (
        StatusCode::OK,
        Json(json!({"session": session_res.unwrap()})),
    )
}

#[axum::debug_handler]
pub async fn health_check() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"status": "ok"})))
}

#[axum::debug_handler(state = DbState)]
pub async fn test(_: AuthenticatedUser) -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({"message": "Authenticated access successful"})),
    )
}

fn app_router() -> axum::Router<DbState> {
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

pub fn register_handlers(router: axum::Router, db: Arc<Surreal<Client>>) -> axum::Router {
    router.merge(app_router().with_state(db))
}
