extern crate dotenv;
extern crate id3;
extern crate streamium_db;

use std::fs;
use std::io;

use diesel::PgConnection;

use streamium_db::repo;
use std::path::Path;
use std::fs::DirEntry;

pub fn import(conn: &PgConnection) {
    let mp3_dir = "/home/juri/Music/";

    repo::delete_all_files(conn);

    visit_dirs(Path::new(mp3_dir), conn, &create_file_for_path)
        .expect("Error on import");

    let artist_root = repo::get_artist_root(conn);
    for artist in repo::get_all_artists(conn) {
        let artist_node = repo::create_artist(conn, artist.as_str(), Some(artist_root.id));
        for album in repo::get_albums_for_artist(conn, artist.as_str()) {
            let album_node = repo::create_album(conn, album.as_str(), Some(artist_node.id));
            repo::update_all_files(conn, &artist_node, &album_node);
        }
    }
}

fn create_file_for_path(path: &DirEntry, conn: &PgConnection) {
    if id3::Tag::read_from_path(path.path()).is_ok() {
        let tag = id3::Tag::read_from_path(path.path()).unwrap();
        if tag.title().is_some() {
            let mut track_number: Option<i32> = None;
            if tag.track().is_some() {
                track_number = Some(tag.track().unwrap() as i32);
            }
            repo::create_file(conn,
                              tag.title().unwrap(), "",
                              tag.artist(), tag.year(), tag.album(),
                              track_number);
        } else {
            println!("Not importig {:?}", path)
        }
    }
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, conn: &PgConnection, cb: &dyn Fn(&DirEntry, &PgConnection)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, conn, cb)?;
            } else {
                cb(&entry, conn);
            }
        }
    }
    Ok(())
}