use crate::user;
use crate::utils::gen_id;
use rocket::config::Environment;
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
fn test_gen_id() -> String {
    gen_id(17)
}

/** Starts Rocket and Mounts Routes. **/
pub fn gen_routes() {
    let mut config = Config::new(Environment::Production);
    #[cfg(feature = "prod")]
    {
        config = Config::new(Environment::Production);
    }
    #[cfg(feature = "staging")]
    {
        config = Config::new(Environment::Staging);
    }
    let app = rocket::custom(config);
    let mut routes = routes!(index, test_gen_id);
    routes.append(&mut routes!(user::routes::get, user::routes::login));
    #[cfg(feature = "communities")]
    routes.append(&mut routes!(UNUSED!));
    #[cfg(feature = "collections")]
    routes.append(&mut routes!(UNUSED!));
    #[cfg(feature = "messages")]
    routes.append(&mut routes!(UNUSED!));
    app.mount("/", routes).launch();
}
