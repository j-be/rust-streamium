extern crate dotenv;
extern crate id3;
extern crate streamium_db;

use std::fs;

use diesel::PgConnection;

use streamium_db::repo;

pub fn import(conn: &PgConnection) {
    let paths = fs::read_dir("/home/juri/Music/Tool/2006 - 10,000 Days").unwrap();

    let artist_root = repo::get_artist_root(conn);

    repo::delete_all_files(conn);
    for path in paths {
        let tag = id3::Tag::read_from_path(path.unwrap().path()).unwrap();
        repo::create_file(conn, tag.title().unwrap(), "", tag.artist(), tag.year(), tag.album(), None);
    }

    for artist in repo::get_all_artists(conn) {
        let artist_node = repo::create_container(conn, artist.as_str(), Some(artist_root.id));
        for album in repo::get_albums_for_artist(conn, artist.as_str()) {
            let album_node = repo::create_container(conn, album.as_str(), Some(artist_node.id));
            repo::update_all_files(conn, &artist_node, &album_node);
        }
    }
}