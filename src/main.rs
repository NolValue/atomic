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
mod post;
mod follow;
use routes::gen_routes;

#[rocket::main]
async fn main() {
    if let Err(e) = gen_routes().launch().await {
        println!("Failure to launch!");
        println!("Error: {:?}", e);
    };
}
