use super::schema::{nodes, node_parents};

#[derive(DbEnum, Debug, Display, Serialize, Deserialize, PartialEq, Eq)]
pub enum Nodetypes {
    Container,
    Artist,
    Album,
    File,
    Stream
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: i32,
    pub title: String,
    pub url: Option<String>,
    pub artist: Option<String>,
    pub year: Option<i32>,
    pub album: Option<String>,
    pub track_number: Option<i32>,
    pub node_type: Nodetypes,
}

#[derive(Insertable)]
#[table_name="nodes"]
pub struct SimpleNode<'a> {
    pub title: &'a str,
    pub url: Option<&'a str>,
    pub node_type: Nodetypes,
}

#[derive(Insertable, Debug)]
#[table_name="nodes"]
pub struct FileNode<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub artist: Option<&'a str>,
    pub year: Option<i32>,
    pub node_type: Nodetypes,
    pub album: Option<&'a str>,
    pub track_number: Option<i32>,
}

#[derive(Queryable)]
pub struct NodeParent {
    pub id: i32,
    pub node_id: i32,
    pub parent_id: i32,
    pub node_order: Option<i32>,
}

#[derive(Insertable)]
#[table_name="node_parents"]
pub struct NewParent {
    pub node_id: i32,
    pub parent_id: i32,
    pub node_order: Option<i32>,
}
