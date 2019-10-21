table! {
    use crate::models::NodetypesMapping;
    use diesel::sql_types::{Int4, Nullable, Varchar};
    nodes (id) {
        id -> Int4,
        title -> Varchar,
        url -> Varchar,
        artist -> Nullable<Varchar>,
        year -> Nullable<Int4>,
        node_type -> NodetypesMapping,
        album -> Nullable<Varchar>,
    }
}
