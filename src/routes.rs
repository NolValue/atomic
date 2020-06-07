use crate::user::{UserErrors, hash_pass, get};
use rocket_contrib::json::*;
use rocket_contrib::databases::diesel;
use crate::user::usermodel::UserPublic;

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
fn user_get(uid: String, conn: AtomicDB) -> Result<Json<UserPublic>, String> {
    get(uid, &*conn)
        .map(|user| Json(user))
        .map_err(|error| format!("Error: {}", error))
}
#[get("/user/get")]
fn user_get_missing() -> JsonValue{
    json!({"status": 400, "response": UserErrors::MissingUID})
}

/** Starts Rocket and Mounts Routes. **/
pub fn gen_routes(){
    rocket::ignite()
        .mount("/", routes![index, user_get, user_get_missing, test_pass])
        .attach(AtomicDB::fairing())
        .launch();
}