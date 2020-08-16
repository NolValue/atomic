use super::get_by_id;
use super::get_by_login;
use super::model::UserLogin;
use crate::auth::{gen_auth, delete_auth, delete_auth_by_user};
use crate::routes::AtomicDB;
use crate::user::{create_user, delete_user};
use diesel::result::Error;
use rocket::request::Form;
use rocket_contrib::json::JsonValue;
use crate::auth::model::Session;

/** User Routes. **/
#[get("/user/get/<uid>")]
pub fn get(uid: String, conn: AtomicDB) -> Result<JsonValue, Error> {
    get_by_id(uid, &*conn)
        .map(|user| user.to_public())
        .map_err(|error| error)
}

#[post("/user", data = "<ul>")]
pub fn create(ul: Option<Form<UserLogin>>, conn: AtomicDB) -> JsonValue {
    let str = create_user(ul.unwrap().0, &*conn);
    json!({ "status": str })
}

#[delete("/user")]
pub fn delete(sess: Session, conn: AtomicDB) -> String {
    let uid = sess.clone().get_uid(&*conn);
    let rslt = delete_auth_by_user(uid.clone(), &*conn);
    delete_user(uid.clone(), &*conn).to_string()
}
