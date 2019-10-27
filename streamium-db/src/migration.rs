embed_migrations!("migrations");

use std::env;

use diesel::prelude::*;

pub fn migrate() {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Migration failed!");
}
