use surrealdb::{Datetime, RecordId};

pub use super::database::connection::DatabaseConnection;
pub use super::extractors::*;
pub use super::module::BaseModule;
pub type BaseId = RecordId;
pub type BaseDateTime = Datetime;
