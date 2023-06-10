use crate::libs::config::constants;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    let database_uri = &constants::DATABASE_URI;
    PgConnection::establish(database_uri).unwrap_or_else(|_| panic!("Error connecting to database"))
}
