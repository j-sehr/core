pub trait DatabaseModel {
    fn table_name() -> &'static str;
    fn key_prefix() -> String {
        format!("{}_", Self::table_name())
    }

    fn from_named_format(key: &str) -> Option<surrealdb::RecordId> {
        let id = key.strip_prefix(&Self::key_prefix())?;

        Some(surrealdb::RecordId::from((Self::table_name(), id)))
    }

    fn to_named_format(record_id: &surrealdb::RecordId) -> String {
        format!("{}{}", Self::key_prefix(), record_id.key())
    }
}
