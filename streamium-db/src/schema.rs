table! {
    node_parents (id) {
        id -> Int4,
        node_id -> Int4,
        parent_id -> Int4,
        node_order -> Nullable<Int4>,
    }
}

table! {
    use crate::models::NodetypesMapping;
    use diesel::sql_types::{Int4, Nullable, Varchar};
    nodes (id) {
        id -> Int4,
        title -> Varchar,
        url -> Nullable<Varchar>,
        artist -> Nullable<Varchar>,
        year -> Nullable<Int4>,
        album -> Nullable<Varchar>,
        track_number -> Nullable<Int4>,
        node_type -> NodetypesMapping,
    }
}

allow_tables_to_appear_in_same_query!(
    node_parents,
    nodes,
);

joinable!{ node_parents -> nodes(node_id) }
