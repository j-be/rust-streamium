extern crate dotenv;
extern crate id3;
extern crate streamium_db;

use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

use diesel::PgConnection;
use streamium_db::repo;
use streamium_db::repo::get_order;

pub fn import(conn: &PgConnection, mp3_dir: &str) {
    repo::delete_all_files(conn);

    // Create file entries
    visit_dirs(Path::new(mp3_dir), conn, &create_file_for_path, mp3_dir)
        .expect("Error on import");

    // Attach files to their artist and album
    let artist_root = repo::get_artist_root(conn);
    for artist in repo::get_all_artists(conn) {
        let artist_node = repo::create_artist(conn, artist.as_str(), Some(artist_root.id));
        for album in repo::get_albums_for_artist(conn, artist.as_str()) {
            let album_node = repo::create_album(conn, album.as_str(), None, Some(artist_node.id));
            repo::update_all_files(conn, &artist_node, &album_node);
        }
    }

    // Create "Unknown Album" entries where necessary
    for artist in repo::get_no_album_artists(conn) {
        let artist_node = repo::get_artist(conn, artist.as_str());
        let album_node = repo::create_album(conn, "<Unknown Album>", Some(9999), Some(artist_node.id));
        for file in repo::get_no_album_tracks_for_artist(conn, artist.as_str()) {
            repo::attach_node_to_parent(conn, file.id, album_node.id, get_order(&file));
        }
    }
}

fn create_file_for_path(path: &DirEntry, conn: &PgConnection, mp3_dir: &str) {
    if id3::Tag::read_from_path(path.path()).is_ok() {
        let tag = id3::Tag::read_from_path(path.path()).unwrap();
        if tag.title().is_some() && tag.artist().is_some() {
            let mut track_number: Option<i32> = None;
            if tag.track().is_some() {
                track_number = Some(tag.track().unwrap() as i32);
            }

            repo::create_file(conn,
                              tag.title().unwrap(), get_url(path.path().to_str().unwrap(), mp3_dir).as_str(),
                              tag.artist(), tag.year(), tag.album(),
                              track_number);
        } else {
            println!("Not importig {:?}", path)
        }
    }
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, conn: &PgConnection, cb: &dyn Fn(&DirEntry, &PgConnection, &str), mp3_dir: &str) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, conn, cb, mp3_dir)?;
            } else {
                cb(&entry, conn, mp3_dir);
            }
        }
    }
    Ok(())
}

fn get_url(path: &str, mp3_dir: &str) -> String {
    let raw_path = path.to_string()
        .split(mp3_dir)
        .collect::<String>();
    raw_path
        .split("/")
        .map(|p| urlencoding::encode(p.as_ref()))
        .collect::<Vec<String>>()
        .join("/")
}
