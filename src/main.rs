#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate argon2;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
mod auth;
mod routes;
mod schema;
mod user;
mod utils;
mod posts;
use routes::gen_routes;

fn main() {
    gen_routes();
}
