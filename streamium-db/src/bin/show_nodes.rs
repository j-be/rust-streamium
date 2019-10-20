extern crate streamium_db;
extern crate diesel;
extern crate dotenv;

use self::streamium_db::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    repo::create_stream(&connection, "My test Category", "Some URL");
    let results = repo::get_nodes(&connection, 5, 8);

    println!("Displaying {} nodes", results.len());
    for node in results {
        println!("ID: {}", node.id);
        println!("Title: {}", node.title);
        println!("Url: {}", node.url);
        println!("Type: {}", node.node_type);
        println!("----------\n");
    }
}
