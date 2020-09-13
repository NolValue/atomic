use crate::auth;
use crate::follow;
use crate::post;
use crate::user;

/** Database Struct **/
#[database("atomic_db")]
pub struct AtomicDB(diesel::PgConnection);

/** Generic Routes. **/
#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

/** Starts Rocket and Mounts Routes. **/
pub fn gen_routes() -> rocket::Rocket {
    let mut routes = routes!(index);
    routes.append(&mut routes!(
        user::routes::get,
        user::routes::create,
        user::routes::delete,
        user::routes::update,
        user::routes::get_fling,
        user::routes::get_fler
    ));
    routes.append(&mut routes!(
        auth::routes::login,
        auth::routes::refresh,
        auth::routes::validate,
        auth::routes::logout
    ));
    routes.append(&mut routes!(
        post::routes::create,
        post::routes::update,
        post::routes::delete
    ));
    routes.append(&mut routes!(follow::routes::create, follow::routes::delete,));
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
}
