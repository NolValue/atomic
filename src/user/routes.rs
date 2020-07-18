use super::get_by_id;
use super::get_by_login;
use super::model::UserLogin;
use crate::auth::gen_auth;
use crate::routes::AtomicDB;
use diesel::result::Error;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket_contrib::json::JsonValue;

/** User Routes. **/
#[get("/user/get/<uid>")]
pub fn get(uid: String, conn: AtomicDB) -> Result<JsonValue, Error> {
    get_by_id(uid, &*conn)
        .map(|user| user.to_public())
        .map_err(|error| error)
}

#[post("/user/login", data = "<user>")]
pub fn login(user: Form<UserLogin>, conn: AtomicDB, mut cookies: Cookies) -> JsonValue {
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
