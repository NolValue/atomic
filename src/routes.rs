use crate::user::{UserErrors, hash_pass, get, create};
use rocket_contrib::json::*;
use rocket_contrib::databases::diesel;
use crate::user::usermodel::{User, UserAuthable};

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
    json!({"status": 400, "response": status})
}
/** Starts Rocket and Mounts Routes. **/
pub fn gen_routes(){
    rocket::ignite()
        .mount("/", routes![index, user_get, user_get_missing, test_pass, user_create])
        .attach(AtomicDB::fairing())
        .launch();
}