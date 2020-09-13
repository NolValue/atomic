use super::gen_auth;
use crate::auth::model::{Session, SessionFull};
use crate::auth::{delete_auth, get_uid, update_auth};
use crate::routes::AtomicDB;
use crate::user::get_by_login;
use crate::user::model::UserLogin;
use rocket::http::{Cookie, CookieJar};
use rocket_contrib::json::{Json, JsonValue};

#[post("/auth/login", format = "json", data = "<user>")]
pub async fn login(
    user: Json<UserLogin>,
    conn: AtomicDB,
    mut cookies: &CookieJar<'_>,
) -> JsonValue {
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
            .http_only(true)
            .path("/auth")
            .finish();
        cookies.add(session);
        cookies.add(refresh);
        status = 200
    }
    json!({ "status": status })
}

#[post("/auth/logout")]
pub async fn logout(session: Session, conn: AtomicDB, mut cookies: &CookieJar<'_>) {
    let access = Cookie::build("x-bearer-token", "null")
        .http_only(true)
        .path("/")
        .finish();
    let refresh = Cookie::build("refresh_token", "null")
        .http_only(true)
        .path("/auth")
        .finish();
    cookies.remove(access.clone());
    cookies.remove(refresh.clone());
    delete_auth(session.0, &*conn);
}

#[post("/auth/refresh")]
pub async fn refresh(session: SessionFull, conn: AtomicDB, mut cookies: &CookieJar<'_>) {
    /*Unused!*/
    let token = update_auth(session, &*conn);
    let access = Cookie::build("x-bearer-token", token)
        .http_only(true)
        .path("/")
        .finish();
    cookies.add(access);
}

#[get("/auth/valid")]
pub async fn validate(sess: Session, conn: AtomicDB) -> String {
    get_uid(sess.0, &*conn).to_string()
}
