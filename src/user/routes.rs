use super::get_by_id;
use super::get_by_login;
use super::model::UserLogin;
use crate::auth::model::Session;
use crate::auth::{delete_auth, delete_auth_by_user, gen_auth};
use crate::routes::AtomicDB;
use crate::user::{create_user, delete_user, update_user};
use diesel::result::Error;
use rocket::request::Form;
use rocket_contrib::json::{JsonValue, Json};
use crate::user::model::UserAlterable;

/** User Routes. **/
#[get("/user/get/<uid>")]
pub fn get(uid: String, conn: AtomicDB) -> Result<JsonValue, Error> {
    get_by_id(uid, &*conn)
        .map(|user| user.to_public())
        .map_err(|error| error)
}

#[post("/user",  format = "json", data = "<ul>")]
pub fn create(ul: Json<UserLogin>, conn: AtomicDB) -> JsonValue {
    let str = create_user(ul.0, &*conn);
    json!({ "status": str })
}

#[post("/user/update", format = "json", data = "<user>")]
pub fn update(mut user: Json<UserAlterable>, conn: AtomicDB, auth: Session) -> JsonValue {
    user.sanitize();
    json!({ "status": update_user(user.0, auth.get_uid(&*conn), &*conn) })
}

#[delete("/user")]
pub fn delete(sess: Session, conn: AtomicDB) -> JsonValue {
    let uid = sess.clone().get_uid(&*conn);
    let rslt = delete_auth_by_user(uid.clone(), &*conn);
    delete_user(uid.clone(), &*conn).to_string();
    json!({ "status": delete_user(uid.clone(), &*conn) })
}
