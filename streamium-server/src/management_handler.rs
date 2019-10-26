use std::env;

use streamium_db::repo;
use streamium_importer::import;

use crate::StreamiumDbConn;
use rocket::response::Redirect;
use rocket::request::Form;
use streamium_db::models::Nodetypes;


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
    Redirect::to("/streams/-8")
}

#[post("/delete_node/<id>?<parent_id>")]
pub fn delete_node(conn: StreamiumDbConn, id: i32, parent_id: Option<i32>) -> Redirect {
    let node = repo::get_node(&*conn, id);

    if node.is_none() {
        return Redirect::to("/");
    }

    let mut url: String = "/".to_owned();

    repo::delete_node(&*conn, node.as_ref().unwrap());

    if parent_id.is_none() {
        return Redirect::to("/");
    }

    if node.as_ref().unwrap().node_type == Nodetypes::Stream {
        url.push_str("streams/");
    } else {
        url.push_str("nodes/")
    }
    return Redirect::to(format!("{}{}", url, parent_id.unwrap()));
}

#[post("/add_favorite/<id>?<node_order>")]
pub fn add_to_favorites(conn: StreamiumDbConn, id: i32, node_order: Option<i32>) -> Option<Redirect> {
    let node = repo::get_node(&*conn, id);

    node.map_or(None, |node| {
        repo::attach_node_to_parent(&*conn, node.id, -24, node_order);
        Some(Redirect::to(format!("/nodes/{}", id)))
    })
}

#[post("/delete_favorite/<id>")]
pub fn remove_from_favorites(conn: StreamiumDbConn, id: i32) -> Option<Redirect> {
    let node = repo::get_node(&*conn, id);

    node.map_or(None, |node| {
        repo::detach_node_from_parent(&*conn, node.id, -24);
        Some(Redirect::to(format!("/nodes/{}", id)))
    })
}

