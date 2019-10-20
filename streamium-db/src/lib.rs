#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
extern crate dotenv;
extern crate strum;
#[macro_use]
extern crate strum_macros;

pub mod schema;
pub mod models;
pub mod repo;
