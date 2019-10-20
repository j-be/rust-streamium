use diesel::{PgConnection, prelude::*, QueryDsl, QueryResult, RunQueryDsl};

use crate::models::{Node, Nodetypes, SimpleNode};
use crate::schema::nodes::dsl::*;

pub fn get_nodes(conn: &PgConnection, offset: i64, limit: i64) -> Vec<Node> {
    nodes
        .offset(offset)
        .limit(limit)
        .load::<Node>(conn)
        .expect("Error loading nodes")
}

pub fn create_container(conn: &PgConnection, new_title: &str, new_url: &str) {
    create_simple_node(conn, new_title, new_url, Nodetypes::Container)
}

pub fn create_stream(conn: &PgConnection, new_title: &str, new_url: &str) {
    create_simple_node(conn, new_title, new_url, Nodetypes::Stream)
}

fn create_simple_node(conn: &PgConnection, new_title: &str, new_url: &str, new_node_type: Nodetypes) {
    use crate::schema::nodes;

    let new_simple_node = SimpleNode{
        title: new_title,
        url: new_url,
        node_type: new_node_type,
    };
    let result = diesel::insert_into(nodes::table)
        .values(&new_simple_node)
        .get_result(conn) as QueryResult<Node>;
    result.expect("Error saving new Node");
}
