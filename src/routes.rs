use crate::auth;
use crate::user;
use crate::utils::test_replace;

/** Database Struct **/
#[database("atomic_db")]
pub struct AtomicDB(diesel::PgConnection);

/** Generic Routes. **/
#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
async fn test_gen_id() -> String {
    //Html(test_replace())
    test_replace()
}
/** Starts Rocket and Mounts Routes. **/
pub async fn gen_routes() {
    let mut routes = routes!(index, test_gen_id);
    routes.append(&mut routes!(
        user::routes::get,
        user::routes::create,
        user::routes::delete,
        user::routes::update
    ));
    routes.append(&mut routes!(
        auth::routes::login,
        auth::routes::refresh,
        auth::routes::validate,
        auth::routes::logout
    ));
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
