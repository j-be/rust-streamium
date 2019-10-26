use rocket_contrib::templates::Template;
use streamium_db::repo;

use crate::StreamiumDbConn;
use streamium_db::models::{Node, Nodetypes};

const ROOT_NODE: Node = Node {
    id: -1,
    title: String::new(),
    url: None,
    artist: None,
    year: None,
    album: None,
    track_number: None,
    node_type: Nodetypes::Container,
};

#[derive(Serialize)]
struct NodeContext {
    node: Node,
}

#[derive(Serialize)]
struct NodesContext {
    node: Node,
    children: Vec<Node>,
}

#[get("/")]
pub fn get_root_nodes(conn: StreamiumDbConn) -> Template {
    let root_nodes = repo::get_root_nodes(&*conn);
    let context = NodesContext { node: ROOT_NODE, children: root_nodes };
    Template::render("index", context)
}

#[get("/nodes/<id>")]
pub fn get_children_nodes(conn: StreamiumDbConn, id: i32) -> Option<Template> {
    let node = repo::get_node(&*conn, id);

    if node.is_none() {
        return None;
    }

    let children = repo::get_nodes(&*conn, Some(node.as_ref().unwrap().id), 0, 1000);

    Some(Template::render("nodes", NodesContext{node: node.unwrap(), children}))
}

#[get("/streams/<id>")]
pub fn get_children_streams(conn: StreamiumDbConn, id: i32) -> Option<Template> {
    let node = repo::get_node(&*conn, id);

    if node.is_none() {
        return None;
    }

    let children = repo::get_nodes(&*conn, Some(node.as_ref().unwrap().id), 0, 1000);

    Some(Template::render("streams", NodesContext { node: node.unwrap(), children }))
}

#[get("/add_stream/<id>")]
pub fn get_add_stream(conn: StreamiumDbConn, id: i32) -> Option<Template> {
    let node = repo::get_node(&*conn, id);

    if node.is_none() {
        return None;
    }

    Some(Template::render("add_stream", NodeContext { node: node.unwrap() }))
}
