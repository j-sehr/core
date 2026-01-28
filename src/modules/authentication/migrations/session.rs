use crate::modules::base::exports::DatabaseConnection;

pub async fn run_migration(db: &DatabaseConnection) -> anyhow::Result<()> {
    db.query(
        r#"
        DEFINE TABLE IF NOT EXISTS sessions SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS account_id   ON TABLE sessions TYPE record<accounts>;
        DEFINE FIELD IF NOT EXISTS refresh_hash ON TABLE sessions TYPE string;
        DEFINE FIELD IF NOT EXISTS created_at   ON TABLE sessions TYPE datetime DEFAULT time::now();
        DEFINE FIELD IF NOT EXISTS expires_at   ON TABLE sessions TYPE datetime;
        DEFINE FIELD IF NOT EXISTS is_active    ON TABLE sessions TYPE bool DEFAULT false;
        DEFINE FIELD IF NOT EXISTS user_agent   ON TABLE sessions TYPE string DEFAULT "";
        DEFINE FIELD IF NOT EXISTS ip_address   ON TABLE sessions TYPE string DEFAULT "";

        DEFINE INDEX IF NOT EXISTS session_refresh_unique ON TABLE sessions COLUMNS refresh_hash UNIQUE;
        DEFINE INDEX IF NOT EXISTS session_account_idx    ON TABLE sessions COLUMNS account_id;
        "#,
    ).await?;

    Ok(())
}
