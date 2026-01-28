use axum::extract::State;
use surrealdb::{Datetime, RecordId};

pub use super::database::connection::DatabaseConnection;
pub use super::extractors::*;
pub use super::module::BaseModule;
pub type BaseId = RecordId;
pub type BaseDateTime = Datetime;
#[allow(dead_code)]
pub type DbConnectionState = State<DatabaseConnection>; // If this will be used in future, remove allow(dead_code)
