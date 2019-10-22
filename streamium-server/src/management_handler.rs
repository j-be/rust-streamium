use std::env;

use streamium_db::repo;
use streamium_importer::import;

use crate::StreamiumDbConn;
use rocket::response::Redirect;
use rocket::request::Form;


#[derive(FromForm)]
pub struct Stream {
    title: String,
    url: String,
    node_id: i32,
}

#[get("/import")]
pub fn import_files(conn: StreamiumDbConn) -> Redirect {
    import(&*conn, env::var("LIB_DIR").expect("LIB_DIR is missing").as_str());
    Redirect::to("/")
}

#[post("/streams", data = "<new_stream>")]
pub fn post_add_stream(conn: StreamiumDbConn, new_stream: Form<Stream>) -> Redirect {
    repo::create_stream(
        &*conn,
        new_stream.title.as_ref(),
        Some(new_stream.url.as_ref()),
        Some(new_stream.node_id));
    Redirect::to("/streams/2")
}

