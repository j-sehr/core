use crate::{
    models::{
        account::{Account, CreateAccountDTO, UpdateAccountDTO},
        prelude::BaseId,
        session::{CreateSessionOptions, Session, SessionDTO},
    },
    token,
};
use anyhow::Result;
use axum::extract::State;
use chrono::Utc;
use std::env;
use surrealdb::{
    self, Datetime, Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

use std::sync::Arc;
pub type DbState = Arc<Surreal<Client>>;
pub type DbExtractor = State<DbState>;

pub async fn init_db() -> Result<Arc<Surreal<Client>>> {
    let host = env::var("DATABASE_HOST")?;
    let port = env::var("DATABASE_PORT")?;
    let user = env::var("DATABASE_USER")?;
    let pass = env::var("DATABASE_PASSWORD")?;
    let db_name = env::var("DATABASE_NAME")?;
    let namespace = env::var("DATABASE_NAMESPACE")?;

    println!(
        "Connecting to database at {}:{} with user {}",
        host, port, user
    );
    let db = Surreal::new::<Ws>(format!("{}:{}", host, port)).await?;
    db.signin(Root {
        username: user.as_str(),
        password: pass.as_str(),
    })
    .await?;

    db.use_ns(namespace.as_str())
        .use_db(db_name.as_str())
        .await?;

    Ok(Arc::new(db))
}

pub async fn create_account(db: &Surreal<Client>, mut dto: CreateAccountDTO) -> Result<Account> {
    dto.password = crate::password::hash_password(&dto.password)?;

    let created_accounts: Vec<Account> = db.insert("accounts").content(dto).await?;
    if created_accounts.is_empty() {
        return Err(anyhow::anyhow!("Failed to create account"));
    }

    Ok(created_accounts.first().cloned().unwrap())
}

pub async fn get_account_by_username(
    db: &Surreal<Client>,
    username: &str,
) -> Result<Option<Account>> {
    let account: Option<Account> = db
        .query("SELECT * FROM accounts WHERE username = $username LIMIT 1")
        .bind(("username", username.to_string()))
        .await?
        .take(0)?;

    Ok(account)
}

pub async fn update_account(db: &Surreal<Client>, id: BaseId, dto: UpdateAccountDTO) -> Result<()> {
    let account_res = get_account_by_id(db, id.clone()).await?;
    if account_res.is_none() {
        return Err(anyhow::anyhow!("Account not found"));
    }

    let mut account = account_res.unwrap();

    if let Some(password) = &dto.password {
        account.password = crate::password::hash_password(password)?;
    }

    if let Some(username) = &dto.username {
        account.username = username.to_owned();
    }

    let res: Option<Account> = db.update(id).content(account).await?;
    res.ok_or_else(|| anyhow::anyhow!("Account not found"))?;
    Ok(())
}

pub async fn get_account_by_id(db: &Surreal<Client>, id: BaseId) -> Result<Option<Account>> {
    let account: Option<Account> = db.select(id).await?;

    Ok(account)
}

pub async fn delete_account(db: &Surreal<Client>, id: BaseId) -> Result<()> {
    let res: Option<Account> = db.delete(id).await?;
    res.ok_or_else(|| anyhow::anyhow!("Account not found"))?;
    Ok(())
}

pub async fn list_accounts(db: &Surreal<Client>) -> Result<Vec<Account>> {
    let accounts: Vec<Account> = db.select("accounts").await?;
    Ok(accounts)
}

pub async fn create_session(db: &Surreal<Client>, account_id: BaseId) -> Result<SessionDTO> {
    let refresh_token = token::generate_refresh_token()?;
    let session_opts = CreateSessionOptions {
        account_id: account_id.clone(),
        refresh_hash: token::hash_refresh_token(&refresh_token),
        expires_at: Datetime::from(Utc::now() + chrono::Duration::days(7)),
        is_active: true,
    };

    let created_vec: Vec<Session> = db.insert("sessions").content(session_opts).await?;
    if created_vec.is_empty() {
        return Err(anyhow::anyhow!("Failed to create session"));
    }

    let session = &created_vec[0];
    let access_token = token::generate_jwt(&account_id, &session.id)?;
    let session_dto = SessionDTO {
        account_id: session.account_id.clone(),
        access_token,
        refresh_token,
        expires_at: session.expires_at.clone(),
    };

    Ok(session_dto)
}

pub async fn get_session_by_refresh_token(
    db: &Surreal<Client>,
    refresh_token: &str,
) -> Result<Option<Session>> {
    let session: Option<Session> = db
        .query("SELECT * FROM sessions WHERE refresh_token = $refresh_token LIMIT 1")
        .bind(("refresh_token", refresh_token.to_string()))
        .await?
        .take(0)?;

    Ok(session)
}

pub async fn get_session_by_id(db: &Surreal<Client>, id: BaseId) -> Result<Option<Session>> {
    let session: Option<Session> = db.select(id).await?;
    Ok(session)
}

pub async fn delete_session(db: &Surreal<Client>, id: BaseId) -> Result<()> {
    let res: Option<Session> = db.delete(id).await?;
    res.ok_or_else(|| anyhow::anyhow!("Session not found"))?;
    Ok(())
}

pub async fn refresh_session(db: &Surreal<Client>, old_refresh_token: &str) -> Result<SessionDTO> {
    let session_res: Option<Session> = get_session_by_refresh_token(db, old_refresh_token).await?;
    let session = session_res.ok_or_else(|| anyhow::anyhow!("Invalid refresh token"))?;

    if session.expires_at < Datetime::from(Utc::now()) {
        return Err(anyhow::anyhow!("Refresh token has expired"));
    }

    delete_session(db, session.id).await?;
    create_session(db, session.account_id).await
}

pub async fn authenticate_account(
    db: &Surreal<Client>,
    username: &str,
    password: &str,
) -> Result<Option<Account>> {
    let account = get_account_by_username(db, username).await?;
    if let Some(acc) = &account {
        let is_valid = crate::password::verify_password(password, &acc.password)?;
        if !is_valid {
            return Ok(None);
        }
    }

    Ok(account)
}
