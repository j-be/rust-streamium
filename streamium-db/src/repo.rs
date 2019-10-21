use diesel::{PgConnection, prelude::*, QueryDsl, QueryResult, RunQueryDsl};

use crate::models::{Node, Nodetypes, SimpleNode};
use crate::schema::nodes::dsl::*;
use diesel::expression::dsl::count;

pub fn get_nodes(conn: &PgConnection, parent: i32, offset: i64, limit: i64) -> Vec<Node> {
    if parent < 0 {
        return nodes
            .filter(parent_id.is_null())
            .offset(offset)
            .limit(limit)
            .load::<Node>(conn)
            .expect("Error loading nodes");
    }

    nodes
        .filter(parent_id.eq(parent))
        .offset(offset)
        .limit(limit)
        .load::<Node>(conn)
        .expect("Error loading nodes")
}

pub fn get_node_count(conn: &PgConnection,  parent: i32) -> i64 {
    if parent < 0 {
        return nodes
            .select(count(id))
            .filter(parent_id.is_null())
            .first(conn)
            .expect("Error loading nodes");
    }

    nodes
        .select(count(id))
        .filter(parent_id.eq(parent))
        .first(conn)
        .expect("Error loading nodes")
}

pub fn get_root_nodes(conn: &PgConnection) -> Vec<Node> {
    nodes
        .filter(parent_id.is_null())
        .load::<Node>(conn)
        .expect("Error loading nodes")
}

pub fn get_nodes_by_parent(conn: &PgConnection, parent: i32) -> Vec<Node> {
    nodes
        .filter(parent_id.eq(parent))
        .load::<Node>(conn)
        .expect("Error loading nodes")
}

pub fn create_container(conn: &PgConnection, new_title: &str, new_url: &str, new_parent_id: Option<i32>) {
    create_simple_node(conn, new_title, new_url, Nodetypes::Container, new_parent_id)
}

pub fn create_stream(conn: &PgConnection, new_title: &str, new_url: &str, new_parent_id: Option<i32>) {
    create_simple_node(conn, new_title, new_url, Nodetypes::Stream, new_parent_id)
}

fn create_simple_node(conn: &PgConnection, new_title: &str, new_url: &str, new_node_type: Nodetypes, new_parent_id: Option<i32>) {
    use crate::schema::nodes;

    let new_simple_node = SimpleNode {
        title: new_title,
        url: new_url,
        node_type: new_node_type,
        parent_id: new_parent_id,
    };
    let result = diesel::insert_into(nodes::table)
        .values(&new_simple_node)
        .get_result(conn) as QueryResult<Node>;
    result.expect("Error saving new Node");
}
