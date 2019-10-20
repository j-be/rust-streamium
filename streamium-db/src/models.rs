use super::schema::nodes;

#[derive(DbEnum, Debug, Display)]
pub enum Nodetypes {
    Container,
    File,
    Stream
}

#[derive(Queryable)]
pub struct Node {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub artist: Option<String>,
    pub year: Option<i32>,
    pub node_type: Nodetypes,
}

#[derive(Insertable)]
#[table_name="nodes"]
pub struct SimpleNode<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub node_type: Nodetypes,
}
