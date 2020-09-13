use crate::auth::get_uid;
use crate::auth::model::Session;
use crate::follow::model::Follow;
use crate::follow::{create_follow, delete_follow, get_following};
use crate::routes::AtomicDB;
use crate::utils::{gen_id, set_timer_days};
use rocket_contrib::json::JsonValue;

#[post("/follow/<id>")]
pub async fn create(id: String, conn: AtomicDB, auth: Session) -> JsonValue {
    let fol = Follow {
        id: gen_id(24),
        source: get_uid(auth.0, &*conn),
        target: id,
        created_on: set_timer_days(0),
    };
    json!({"status": create_follow(fol, &*conn)})
}

#[delete("/follow/delete/<id>")]
pub async fn delete(id: String, conn: AtomicDB, auth: Session) -> JsonValue {
    let fol = Follow {
        id: gen_id(24),
        source: get_uid(auth.0, &*conn),
        target: id,
        created_on: set_timer_days(0),
    };
    json!({"status": delete_follow(fol, &*conn)})
}
