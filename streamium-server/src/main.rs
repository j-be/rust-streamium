#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate streamium_db;
extern crate streamium_importer;

use rocket_contrib::databases::diesel;

use streamium_db::models::Node;
use rocket_contrib::templates::Template;

mod streamium_handler;
mod management_handler;
mod template_handler;

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
            template_handler::get_root_nodes,
            template_handler::get_children_nodes,
            template_handler::get_children_streams,
            template_handler::get_add_stream,
         ])
        .mount("/", routes![
            management_handler::import_files,
            management_handler::post_add_stream,
        ])
        .attach(Template::fairing())
        .attach(StreamiumDbConn::fairing())
        .launch();
}
