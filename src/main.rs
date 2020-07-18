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
mod auth;
mod routes;
mod schema;
mod user;
mod utils;
use routes::gen_routes;

fn main() {
    gen_routes();
}
