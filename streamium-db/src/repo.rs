use diesel::{PgConnection, prelude::*, QueryDsl, QueryResult, RunQueryDsl};

use crate::models::{Node, Nodetypes, SimpleNode, FileNode, NodeParent, NewParent};
use crate::schema::nodes::dsl::*;
use crate::schema::node_parents::dsl::*;
use crate::schema::{nodes, node_parents};
use diesel::expression::dsl::count;
use diesel::dsl::{select, exists};

pub fn get_order(node: &Node) -> Option<i32>{
    return node.year.map(|y| y * 100 + node.track_number.unwrap_or(0));
}

pub fn get_node(conn: &PgConnection, filter_node_id: i32) -> Option<Node> {
    let node = nodes
        .filter(nodes::id.eq(filter_node_id))
        .first(conn);

    if node.is_ok() {
        return Some(node.unwrap());
    }
    return None;
}

pub fn get_nodes(conn: &PgConnection, parent: Option<i32>, offset: i64, limit: i64) -> Vec<Node> {
    if parent.is_none() {
        return nodes
            .filter(node_type.eq(Nodetypes::Container))
            .order(nodes::id.asc())
            .offset(offset)
            .limit(limit)
            .load::<Node>(conn)
            .expect("Error loading nodes");
    }

    nodes::table.inner_join(node_parents::table)
        .select(nodes::all_columns)
        .filter(parent_id.eq(parent.unwrap()))
        .order((node_order.asc(), title.asc()))
        .offset(offset)
        .limit(limit)
        .load::<Node>(conn)
        .expect("Error loading nodes")
}

pub fn get_node_count(conn: &PgConnection,  parent: Option<i32>) -> i64 {
    if parent.is_none() {
        return nodes
            .select(count(nodes::id))
            .filter(node_type.eq(Nodetypes::Container))
            .first(conn)
            .expect("Error loading nodes");
    }

    nodes::table.inner_join(node_parents::table)
        .select(count(nodes::id))
        .filter(parent_id.eq(parent.unwrap()))
        .first(conn)
        .expect("Error loading nodes")
}

pub fn get_root_nodes(conn: &PgConnection) -> Vec<Node> {
    nodes::table.inner_join(node_parents::table)
        .select(nodes::all_columns)
        .filter(parent_id.is_null())
        .load::<Node>(conn)
        .expect("Error loading nodes")
}

pub fn is_node_favorite(conn: &PgConnection, filter_node_id: i32) -> bool {
    select(exists(node_parents.filter(
        parent_id.eq(-24)
            .and(node_id.eq(filter_node_id)))))
        .get_result(conn)
        .expect("Cannot determine if node is favorite!")
}

pub fn get_all_artists(conn: &PgConnection) -> Vec<String> {
    nodes::table.select(artist)
        .filter(node_type.eq(Nodetypes::File)
            .and(artist.is_not_null()))
        .distinct()
        .load::<Option<String>>(conn)
        .expect("Error loading artists")
        .into_iter()
        .map(|a| a.unwrap())
        .collect()
}

pub fn delete_all_files(conn: &PgConnection) {
    diesel::delete(
        nodes.filter(node_type.ne(Nodetypes::Container)
                .and(node_type.ne(Nodetypes::Stream))))
        .execute(conn)
        .expect("Error deleting files");
}

pub fn get_artist_root(conn: &PgConnection) -> Node {
    nodes
        .filter(title.eq("Artists"))
        .first(conn)
        .expect("Cannot find Artists root")
}

pub fn get_no_album_artists(conn: &PgConnection) -> Vec<String> {
    nodes::table.inner_join(node_parents::table)
        .select(artist)
        .filter(node_type.eq(Nodetypes::File)
            .and(parent_id.is_null()))
        .distinct()
        .load::<Option<String>>(conn)
        .expect("Error loading artists")
        .into_iter()
        .map(|a| a.unwrap())
        .collect()
}

pub fn get_albums_for_artist(conn: &PgConnection, filter_artist: &str) -> Vec<String> {
    nodes::table.select(album)
        .filter(artist.eq(filter_artist)
            .and(node_type.eq(Nodetypes::File))
            .and(album.is_not_null()))
        .distinct()
        .load::<Option<String>>(conn)
        .expect("Error loading album for artists")
        .into_iter()
        .map(|a| a.unwrap())
        .collect()
}

pub fn get_artist(conn: &PgConnection, filter_artist: &str) -> Node {
    nodes
        .filter(node_type.eq(Nodetypes::Artist)
            .and(title.eq(filter_artist)))
        .first(conn)
        .expect("Failed to find artist")
}

pub fn get_no_album_tracks_for_artist(conn: &PgConnection, filter_artist: &str) -> Vec<Node> {
    nodes::table.inner_join(node_parents::table)
        .select(nodes::all_columns)
        .filter(node_type.eq(Nodetypes::File)
            .and(artist.eq(filter_artist))
            .and(parent_id.is_null()))
        .load(conn)
        .expect("Failed to find artist")
}

pub fn create_album(conn: &PgConnection, new_title: &str, new_node_order: Option<i32>, new_parent_id: Option<i32>) -> Node {
    create_simple_node(conn, new_title, None, Nodetypes::Album, new_node_order, new_parent_id)
}

pub fn create_artist(conn: &PgConnection, new_title: &str, new_parent_id: Option<i32>) -> Node {
    create_simple_node(conn, new_title, None, Nodetypes::Artist, None, new_parent_id)
}

pub fn create_stream(conn: &PgConnection, new_title: &str, new_url: Option<&str>, new_parent_id: Option<i32>) -> Node {
    create_simple_node(conn, new_title, new_url, Nodetypes::Stream, None,new_parent_id)
}

fn create_simple_node(conn: &PgConnection, new_title: &str, new_url: Option<&str>, new_node_type: Nodetypes, new_node_order: Option<i32>, new_parent_id: Option<i32>) -> Node {
    let new_simple_node = SimpleNode {
        title: new_title,
        url: new_url,
        node_type: new_node_type,
    };

    let result = diesel::insert_into(nodes::table)
        .values(&new_simple_node)
        .get_result(conn) as QueryResult<Node>;
    let node = result.expect("Error saving new Node");

    if new_parent_id.is_some() {
        attach_node_to_parent(conn, node.id, new_parent_id.unwrap(), new_node_order);
    }

    return node;
}

pub fn create_file(
    conn: &PgConnection,
    new_title: &str,
    new_url: &str,
    new_artist: Option<&str>,
    new_year: Option<i32>,
    new_album: Option<&str>,
    new_track_number: Option<i32>)
{
    let new_file_node = FileNode {
        title: new_title,
        url: new_url,
        artist: new_artist,
        year: new_year,
        node_type: Nodetypes::File,
        album: new_album,
        track_number: new_track_number,
    };

    let result = diesel::insert_into(nodes::table)
    .values(&new_file_node)
    .get_result(conn) as QueryResult<Node>;

    match result {
        Ok(node) => println!("Created: {:?}", node),
        Err(error) => println!("Cannot create: {:?}, {:?}", new_file_node, error),
    }
}

pub fn update_all_files(conn: &PgConnection, artist_node: &Node, album_node: &Node) {
    let files_to_update: Vec<Node> = nodes
        .filter(artist.eq(artist_node.title.as_str())
            .and(album.eq(album_node.title.as_str()))
            .and(node_type.eq(Nodetypes::File)))
        .load::<Node>(conn)
        .expect("Error loading files for album and artist");

    for file in files_to_update {
        attach_node_to_parent(conn, file.id, album_node.id, get_order(&file));
    }
}

pub fn attach_node_to_parent(conn: &PgConnection, filter_node_id: i32, new_parent_id: i32, new_node_order: Option<i32>) {
    let new_parent_node = NewParent {
        node_id: filter_node_id,
        parent_id: new_parent_id,
        node_order: new_node_order,
    };
    let result = diesel::insert_into(node_parents::table)
        .values(&new_parent_node)
        .get_result(conn) as QueryResult<NodeParent>;
    result.expect("Error saving parent");
}

pub fn detach_node_from_parent(conn: &PgConnection, filter_node_id: i32, filter_parent_id: i32) {
    diesel::delete(node_parents.filter(
        node_id.eq(filter_node_id)
            .and(parent_id.eq(filter_parent_id))
    )).execute(conn)
        .expect("Error while detaching node from parent!");
}

pub fn delete_node(conn: &PgConnection, delete_node: &Node) {
    diesel::delete(delete_node)
        .execute(conn)
        .expect("Error deleting node");
}
