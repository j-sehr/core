pub mod account;
pub mod session;

pub(super) mod prelude {
    pub use serde::{Deserialize, Serialize};
    use surrealdb::{Datetime, RecordId};

    pub type BaseId = RecordId;
    pub type Timestamp = Datetime;
}
