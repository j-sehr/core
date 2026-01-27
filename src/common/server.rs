use crate::modules::base::exports::DatabaseConnection;

pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub database_connection: Option<DatabaseConnection>,
}

impl ServerSettings {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            database_connection: None,
        }
    }

    pub fn set_database_connection(&mut self, db_conn: DatabaseConnection) {
        self.database_connection = Some(db_conn);
    }

    pub fn get_database_connection(&self) -> Option<&DatabaseConnection> {
        self.database_connection.as_ref()
    }
}
