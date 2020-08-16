use crate::auth;
use crate::user;
use crate::utils::{gen_id, set_timer_days, test_replace};
use rocket::config::Environment;
use rocket::response::content::Html;
use rocket::{Config, Rocket, Route};

/** Database Struct **/
#[database("atomic_db")]
pub struct AtomicDB(diesel::PgConnection);

/** Generic Routes. **/
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test_gen_id() -> /*Html<String>*/ String {
    //Html(test_replace())
    test_replace()
}
/** Starts Rocket and Mounts Routes. **/
pub fn gen_routes() {
    let mut routes = routes!(index, test_gen_id);
    routes.append(&mut routes!(user::routes::get, user::routes::create, user::routes::delete));
    routes.append(&mut routes!(auth::routes::login, auth::routes::refresh, auth::routes::validate));
    #[cfg(feature = "communities")]
    routes.append(&mut routes!(/* Unused */));
    #[cfg(feature = "collections")]
    routes.append(&mut routes!(/* Unused */));
    #[cfg(feature = "messages")]
    routes.append(&mut routes!(/* Unused */));
    /*app.mount("/", routes)
    .attach(AtomicDB::fairing())
    .launch();new(Environment::Development)*/
    rocket::ignite()
        .mount("/", routes)
        .attach(AtomicDB::fairing())
        .launch();
}
