use super::gen_auth;
use crate::routes::AtomicDB;
use crate::user::get_by_login;
use crate::user::model::UserLogin;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket_contrib::json::JsonValue;
use crate::auth::{validate_auth, get_uid};
use crate::auth::model::Session;

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
        let session = Cookie::build("x-bearer-token", auth.access_token)
            .http_only(true)
            .path("/")
            .finish();
        let refresh = Cookie::build("refresh_token", auth.refresh_token)
            .secure(true)
            .path("/auth/refresh")
            .finish();
        cookies.remove(Cookie::named("x-bearer-token"));
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


#[get("/auth/valid")]
pub fn validate(sess: Session, conn: AtomicDB) -> String {
    get_uid(sess.0, &*conn).to_string()
}
