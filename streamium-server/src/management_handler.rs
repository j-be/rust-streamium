use rocket::http::Status;
use rocket_contrib::json::Json;

use streamium_db::models::Node;
use streamium_db::repo;
use streamium_importer::import;

use crate::StreamiumDbConn;

#[get("/nodes")]
pub fn all_nodes(conn: StreamiumDbConn) -> Json<Vec<Node>> {
    Json(repo::get_root_nodes(&*conn))
}

#[get("/import")]
pub fn import_files(conn: StreamiumDbConn) -> Status {
    import(&*conn);
    Status::NoContent
}
