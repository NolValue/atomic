use super::gen_auth;
use crate::routes::AtomicDB;
use crate::user::get_by_login;
use crate::user::model::UserLogin;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket_contrib::json::JsonValue;

#[post("/auth/login", data = "<user>")]
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
            .path("/auth/refresh")
            .finish();
        cookies.add(session);
        cookies.add(refresh);
        status = 200
    }
    json!({ "status": status })
}

#[post("/auth/refresh")]
pub fn refresh() {
    /*Unused!*/
}
