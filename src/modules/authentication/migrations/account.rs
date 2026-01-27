use crate::modules::base::exports::DatabaseConnection;

pub async fn run_migration(db: &DatabaseConnection) -> anyhow::Result<()> {
    db.query(
        r#"
        DEFINE TABLE IF NOT EXISTS accounts SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS username      ON TABLE accounts TYPE string;
        DEFINE FIELD IF NOT EXISTS password ON TABLE accounts TYPE string;
        DEFINE FIELD IF NOT EXISTS created_at    ON TABLE accounts TYPE datetime DEFAULT time::now();
        DEFINE FIELD IF NOT EXISTS updated_at    ON TABLE accounts TYPE datetime VALUE time::now();

        DEFINE INDEX IF NOT EXISTS account_username_unique ON TABLE accounts COLUMNS username UNIQUE;
        "#,
    ).await?;

    Ok(())
}
