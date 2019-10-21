use super::schema::nodes;

#[derive(DbEnum, Debug, Display, Serialize, Deserialize)]
pub enum Nodetypes {
    Container,
    File,
    Stream
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Node {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub artist: Option<String>,
    pub year: Option<i32>,
    pub node_type: Nodetypes,
    pub album: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name="nodes"]
pub struct SimpleNode<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub node_type: Nodetypes,
    pub parent_id: Option<i32>,
}
