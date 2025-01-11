use crate::settings::settings;
use diesel::{Connection, PgConnection};

pub fn pg_connection() -> PgConnection {
    let settings = settings();
    let server_ip = &settings.database.database_ip;
    let server_port = &settings.database.database_port;
    let database_name = &settings.database.database_name;
    let database_user = &settings.database.database_user;
    let database_password = &settings.database.database_password;
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database_user, database_password, server_ip, server_port, database_name
    );

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
