use diesel::{Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::{
    error::{TuduError, TuduResult},
    infrastructure::env,
};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn database_connection() -> SqliteConnection {
    let database_url = env::database_url_env();
    let database_url = database_url.to_str().unwrap();
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_database_migrations() -> TuduResult<usize> {
    let mut connection = database_connection();
    let success = connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| TuduError::DatabaseError(e.to_string()))?;
    Ok(success.len())
}
