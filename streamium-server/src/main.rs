#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate streamium_db;
extern crate streamium_importer;

use rocket_contrib::databases::diesel;

use streamium_db::models::Node;

mod streamium_handler;
mod management_handler;

#[derive(Serialize)]
pub struct NodeList {
    nodes: Vec<Node>,
    totnumelem: i64,
    fromindex: i64,
}

#[database("streamium_db")]
pub struct StreamiumDbConn(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .mount("/", routes![
            streamium_handler::get_nodes,
            management_handler::import_files,
            management_handler::all_nodes,
         ])
        .attach(StreamiumDbConn::fairing())
        .launch();
}
