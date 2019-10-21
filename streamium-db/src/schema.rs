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
        parent_id -> Nullable<Int4>,
    }
}
