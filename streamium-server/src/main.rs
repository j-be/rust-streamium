#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate streamium_db;

use rocket_contrib::databases::diesel;

use streamium_db::models::Node;

mod streamium_handler;

#[derive(Serialize)]
pub struct NodeList {
    nodes: Vec<Node>
}

#[database("streamium_db")]
pub struct StreamiumDbConn(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .mount("/", routes![
            streamium_handler::get_nodes,
         ])
        .attach(StreamiumDbConn::fairing())
        .launch();
}
