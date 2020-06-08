use crate::user::{UserErrors, hash_pass, get, create};
use rocket_contrib::json::*;
use rocket_contrib::databases::diesel;
use crate::user::usermodel::{User, UserAuthable};
use crate::authentication::{auth_user, refresh_auth, validate_auth};
use rocket::http::{Cookie, Cookies};
use chrono::Duration;
use std::borrow::Borrow;
use crate::authentication::authmodel::AuthSerializable;

/** Database Struct **/
#[database("atomic_db")]
pub struct AtomicDB(diesel::PgConnection);

/** Generic Routes. **/
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
//Just hashes text. Used for testing.
#[get("/test/<pass>")]
fn test_pass(pass: String) -> String{
    hash_pass(pass)
}

/** User Routes. **/
#[get("/user/get/<uid>")]
fn user_get(uid: String, conn: AtomicDB) -> Result<Json<User>, JsonValue> {
    get(uid, &*conn)
        .map(|user| Json(user))
        .map_err(|error| json!({"status": 400, "response": error.to_string()}))
}

#[get("/user/get")]
fn user_get_missing() -> JsonValue{
    json!({"status": 400, "response": UserErrors::MissingData})
}

#[post("/user/create")]
fn user_create(account: UserAuthable, conn: AtomicDB) -> JsonValue {
    let status = create(account, &*conn);
    match status{
        0 => json!({"status": 400, "success": status}),
        1 => json!({"status": 200, "success": status}),
        _ => json!({"status": 400, "success": "unexpected result"})
    }
}

/** Auth Routes **/
#[post("/auth/login")]
pub fn auth_login(mut cookies: Cookies, account: UserAuthable, conn: AtomicDB) -> JsonValue{
    let mut auth;
    match cookies.get("session-token"){
        Some(cookie) => match validate_auth(cookie.value().to_string(), &*conn){
            true => auth = AuthSerializable{ uid: "".to_string(), access_token: cookies.get("session-token").unwrap().value().to_string(), refresh_token: cookies.get("refresh-token").unwrap().value().to_string() },
            false => auth = refresh_auth(cookie.value().to_string(), &*conn)
        },
        None => auth = auth_user(account, &*conn)
    }
    let session = Cookie::build("session-token", auth.access_token).secure(false).max_age(Duration::days(7)).http_only(true).finish();
    let refresh = Cookie::build("refresh-token", auth.refresh_token).secure(false).max_age(Duration::max_value()).http_only(true).finish();
    cookies.add(session);
    cookies.add(refresh);
    json!({"status": 200})
}

/** Starts Rocket and Mounts Routes. **/
pub fn gen_routes(){
    rocket::ignite()
        .mount("/", routes![index, user_get, user_get_missing, test_pass, user_create, auth_login])
        .attach(AtomicDB::fairing())
        .launch();
}