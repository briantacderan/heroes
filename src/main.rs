#![feature(proc_macro_hygiene, decl_macro, trait_alias)]
//#![feature(associated_type_bounds)]

/* Our extern crates */
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

extern crate dotenv;

/* Importing functions */
use diesel::{
    connection::Connection,
    pg::PgConnection,
};

use dotenv::dotenv;
use std::env;
use rocket_contrib::templates::Template;

/* Static files imports */
use std::path::{ Path, PathBuf };
use rocket::response::NamedFile;

/* Declaring a module, just for separating things better */
pub mod heroes;

/* Will hold our data structs */
pub mod models;

/* auto-generated table macros */
pub mod schema;

/* This will return our pg connection to use with diesel */
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/*
pub fn parse_hero(header: &ConnectionResult<T>) -> Result<Vec<T>, ConnectionError> {
    match header.get(0) {
        None => Err(E),
        Some(Vec) => Ok(),
        Some(_) => Err("Invalid Hero"),
    }
}
*/

/* Static files Handler, will give back our heroes images */ 
#[get("/imgs/<file..>")]
fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("imgs/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![
        assets,
        heroes::list, 
        heroes::new, 
        heroes::insert,
        heroes::update,
        heroes::process_update,
        heroes::delete
        ]).attach(Template::fairing()).launch();
}