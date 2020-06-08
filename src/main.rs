#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate chrono;

extern crate argon2;
mod schema;
mod routes;
mod user;
mod authentication;
use routes::gen_routes;


fn main() {
    gen_routes();
}