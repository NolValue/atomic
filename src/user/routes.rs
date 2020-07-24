use super::get_by_id;
use super::get_by_login;
use super::model::UserLogin;
use crate::auth::gen_auth;
use crate::routes::AtomicDB;
use crate::user::create_user;
use diesel::result::Error;
use rocket::request::Form;
use rocket_contrib::json::JsonValue;

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
