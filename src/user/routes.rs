use super::get_by_id;
use super::model::UserLogin;
use crate::auth::delete_auth_by_user;
use crate::auth::model::Session;
use crate::follow::{get_followers, get_following, delete_f_by_uid};
use crate::routes::AtomicDB;
use crate::user::model::UserAlterable;
use crate::user::{create_user, delete_user, update_user};
use diesel::result::Error;
use rocket_contrib::json::{Json, JsonValue};
use crate::post::delete_by_uid;

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
    delete_auth_by_user(uid.clone(), &*conn);
    delete_by_uid(uid.clone(), &*conn);
    delete_f_by_uid(uid.clone(), &*conn);
    json!({ "status": delete_user(uid.clone(), &*conn) })
}

#[get("/user/following/<id>")]
pub async fn get_fling(id: String, conn: AtomicDB) -> JsonValue {
    json!({"following": get_following(id, &*conn)})
}

#[get("/user/followers/<id>")]
pub async fn get_fler(id: String, conn: AtomicDB) -> JsonValue {
    json!({"following": get_followers(id, &*conn)})
}
