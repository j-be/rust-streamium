#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate strum;
#[macro_use]
extern crate strum_macros;

pub mod schema;
pub mod models;
pub mod repo;
pub mod migration;
