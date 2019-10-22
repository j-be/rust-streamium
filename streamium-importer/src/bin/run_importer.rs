use std::env;

use diesel::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;
use streamium_importer::import;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    import(&connection, "/home/juri/Music/".as_ref());
}
