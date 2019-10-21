use diesel::{PgConnection, prelude::*, QueryDsl, QueryResult, RunQueryDsl};

use crate::models::{Node, Nodetypes, SimpleNode, FileNode};
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

pub fn get_all_artists(conn: &PgConnection) -> Vec<String> {
    use crate::schema::nodes;

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
    use crate::schema::nodes;

    nodes::table.select(artist)
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
    use crate::schema::nodes;

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
    nodes
        .filter(node_type.eq(Nodetypes::File)
            .and(artist.eq(filter_artist))
            .and(parent_id.is_null()))
        .load(conn)
        .expect("Failed to find artist")
}

pub fn create_album(conn: &PgConnection, new_title: &str, new_parent_id: Option<i32>) -> Node {
    create_simple_node(conn, new_title, None, Nodetypes::Album, new_parent_id)
}

pub fn create_artist(conn: &PgConnection, new_title: &str, new_parent_id: Option<i32>) -> Node {
    create_simple_node(conn, new_title, None, Nodetypes::Artist, new_parent_id)
}

pub fn create_stream(conn: &PgConnection, new_title: &str, new_url: Option<&str>, new_parent_id: Option<i32>) -> Node {
    create_simple_node(conn, new_title, new_url, Nodetypes::Stream, new_parent_id)
}

fn create_simple_node(conn: &PgConnection, new_title: &str, new_url: Option<&str>, new_node_type: Nodetypes, new_parent_id: Option<i32>) -> Node {
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
    result.expect("Error saving new Node")
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
    use crate::schema::nodes;

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
    result.expect("Error saving new Node");
}

pub fn update_all_files(conn: &PgConnection, artist_node: &Node, album_node: &Node) {
    let files_to_update: Vec<Node> = nodes
        .filter(artist.eq(artist_node.title.as_str())
            .and(album.eq(album_node.title.as_str()))
            .and(node_type.eq(Nodetypes::File)))
        .load::<Node>(conn)
        .expect("Error loading files for album and artist");

    for file in files_to_update {
        attach_file_to_node(conn, &file, album_node);
    }
}

pub fn attach_file_to_node(conn: &PgConnection, file: &Node, parent: &Node) {
    diesel::update(file)
        .set(parent_id.eq(parent.id))
        .execute(conn)
        .expect("Error updating file");
}
