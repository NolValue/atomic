use super::get_by_id;
use super::model::UserLogin;
use crate::auth::model::Session;
use crate::auth::delete_auth_by_user;
use crate::routes::AtomicDB;
use crate::user::model::UserAlterable;
use crate::user::{create_user, delete_user, update_user};
use diesel::result::Error;
use rocket_contrib::json::{Json, JsonValue};

/** User Routes. **/
#[get("/user/get/<uid>")]
pub async fn get(uid: String, conn: AtomicDB) -> Result<JsonValue, JsonValue> {
    get_by_id(uid, &*conn)
        .map(|user| user.to_public())
        .map_err(|error| json!({"status": error.to_string()}))
}

#[post("/user", format = "json", data = "<ul>")]
pub async fn create(ul: Json<UserLogin>, conn: AtomicDB) -> JsonValue {
    let str = create_user(ul.0, &*conn);
    json!({ "status": str })
}

#[post("/user/update", format = "json", data = "<user>")]
pub async fn update(mut user: Json<UserAlterable>, conn: AtomicDB, auth: Session) -> JsonValue {
    user.sanitize();
    json!({ "status": update_user(user.0, auth.get_uid(&*conn), &*conn) })
}

#[delete("/user")]
pub async fn delete(sess: Session, conn: AtomicDB) -> JsonValue {
    let uid = sess.clone().get_uid(&*conn);
    let _rslt = delete_auth_by_user(uid.clone(), &*conn);
    delete_user(uid.clone(), &*conn).to_string();
    json!({ "status": delete_user(uid.clone(), &*conn) })
}
