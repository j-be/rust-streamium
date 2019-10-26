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
    is_node_favorite: bool,
    children: Vec<Node>,
}

#[get("/")]
pub fn get_root_nodes(conn: StreamiumDbConn) -> Template {
    let root_nodes = repo::get_root_nodes(&*conn);
    let context = NodesContext { node: ROOT_NODE, is_node_favorite: true, children: root_nodes };
    Template::render("index", context)
}

#[get("/nodes/<id>")]
pub fn get_children_nodes(conn: StreamiumDbConn, id: i32) -> Option<Template> {
    let node = repo::get_node(&*conn, id);

    node.map_or(None, |node| {
        let is_node_favorite = repo::is_node_favorite(&*conn, node.id);
        let children = repo::get_nodes(&*conn, Some(node.id), 0, 1000);

        Some(Template::render("nodes", NodesContext{node, is_node_favorite, children}))
    })
}

#[get("/streams/<id>")]
pub fn get_children_streams(conn: StreamiumDbConn, id: i32) -> Option<Template> {
    let node = repo::get_node(&*conn, id);

    node.map_or(None, |node| {
        let is_node_favorite = repo::is_node_favorite(&*conn, node.id);
        let children = repo::get_nodes(&*conn, Some(node.id), 0, 1000);

        Some(Template::render("streams", NodesContext{node, is_node_favorite, children}))
    })
}

#[get("/add_stream/<id>")]
pub fn get_add_stream(conn: StreamiumDbConn, id: i32) -> Option<Template> {
    let node = repo::get_node(&*conn, id);

    if node.is_none() {
        return None;
    }

    Some(Template::render("add_stream", NodeContext { node: node.unwrap() }))
}
