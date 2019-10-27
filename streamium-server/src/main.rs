#![feature(proc_macro_hygiene, decl_macro, int_error_matching)]

extern crate ifaces;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate streamium_db;
extern crate streamium_importer;

use std::env;
use std::thread;

use rocket_contrib::databases::diesel;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use dotenv::dotenv;

mod streamium_handler;
mod management_handler;
mod template_handler;
mod advertiser;

#[database("streamium_db")]
pub struct StreamiumDbConn(diesel::PgConnection);

fn main() {
    dotenv().ok();

    thread::spawn(move || {
        let server_address = env::var("ROCKET_ADDRESS")
            .expect("Error while getting ROCKET_ADDRESS!");
        let server_port = env::var("ROCKET_PORT")
            .expect("Error while getting ROCKET_PORT!")
            .parse::<u16>()
            .expect("Error while parsing ROCKET_PORT!");
        advertiser::listen(server_address.as_str(), server_port);
    });
    ignite_rocket();
}

fn ignite_rocket() {
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
            management_handler::delete_node,
            management_handler::add_to_favorites,
            management_handler::remove_from_favorites,
        ])
        .mount("/files", StaticFiles::from(env::var("LIB_DIR").expect("Cannot find LIB_DIR in env")))
        .attach(Template::fairing())
        .attach(StreamiumDbConn::fairing())
        .launch();
}
