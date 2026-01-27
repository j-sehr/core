use crate::modules::base::exports::DatabaseConnection;

mod account;
mod session;

pub async fn run_migrations(db: &DatabaseConnection) -> anyhow::Result<()> {
    account::run_migration(db).await?;
    session::run_migration(db).await?;
    Ok(())
}
