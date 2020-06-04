use crate::user::UserErrors;
use rocket::http::RawStr;
use rocket_contrib::json::*;
/** Generic Routes. **/
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/** User Routes. **/
#[get("/user/get/<uid>")]
fn user_get(uid: Result<i64, &RawStr>) -> JsonValue {
    match uid {
        Ok(id) => json!({"id": id}),
        Err(_e) => json!({"status": 400, "response": UserErrors::InvalidUID})
    }
}
#[get("/user/get")]
fn user_get_missing() -> JsonValue{
    json!({"status": 400, "response": UserErrors::MissingUID})
}


/** Starts Rocket and Mounts Routes. **/
pub fn gen_routes(){
    rocket::ignite().mount("/", routes![index, user_get, user_get_missing]).launch();
}