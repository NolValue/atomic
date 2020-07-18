use super::auth;
use super::user;
use super::utils::gen_id;
use crate::auth::gen_auth;
use auth::model::Auth;
use diesel::result::Error;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::*;
use serde::Serialize;
use user::get_by_login;
use user::model::UserLogin;

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

/** User Routes. **/
#[get("/user/get/<uid>")]
fn user_get(uid: String, conn: AtomicDB) -> Result<JsonValue, Error> {
    user::get_by_id(uid, &*conn)
        .map(|user| user.to_public())
        .map_err(|error| error)
}

#[get("/user/get")]
fn user_get_missing() -> JsonValue {
    json!({"status": 400})
}

#[post("/user/login", data = "<user>")]
fn user_login(user: Form<UserLogin>, conn: AtomicDB, mut cookies: Cookies) -> JsonValue {
    let usr = match get_by_login(user.0, &*conn) {
        Ok(u) => u,
        Err(e) => return json!({ "error": e }),
    };
    let mut status = 400;
    let auth = gen_auth(usr, &*conn);
    if auth.is_ok() {
        let auth = auth.unwrap();
        let session = Cookie::build("session_token", auth.access_token)
            .secure(true)
            .finish();
        let refresh = Cookie::build("refresh_token", auth.refresh_token)
            .secure(true)
            .finish();
        cookies.add(session);
        cookies.add(refresh);
        status = 200
    }
    json!({ "status": status })
}

/** Starts Rocket and Mounts Routes. **/
pub fn gen_routes() {
    rocket::ignite()
        .mount("/", routes![index, user_get, user_get_missing, test_gen_id])
        .attach(AtomicDB::fairing())
        .launch();
}
