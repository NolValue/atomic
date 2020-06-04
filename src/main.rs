#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate argon2;
mod schema;
mod routes;
mod user;
fn main() {
    routes::gen_routes();
}