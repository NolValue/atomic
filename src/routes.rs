use rocket_contrib::json::*;
use rocket_contrib::databases::diesel;
use super::user;
use diesel::result::Error;

/** Database Struct **/
#[database("atomic_db")]
pub struct AtomicDB(diesel::PgConnection);

/** Generic Routes. **/
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test/<pass>")]
fn test_pass(pass: String) -> String{
    pass
}

/** User Routes. **/
#[get("/user/get/<uid>")]
fn user_get(uid: String, conn: AtomicDB) -> Result<JsonValue, Error> {
    user::get_by_id(uid, &*conn)
        .map(|user| user.public_json())
        .map_err(|error| error)
}

#[get("/user/get")]
fn user_get_missing() -> JsonValue{
    json!({"status": 400})
}


/** Starts Rocket and Mounts Routes. **/
pub fn gen_routes(){
    rocket::ignite()
        .mount("/", routes![index, user_get, user_get_missing, test_pass])
        .attach(AtomicDB::fairing())
        .launch();
}