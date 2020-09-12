use crate::routes::AtomicDB;
use crate::auth::model::Session;
use rocket_contrib::json::JsonValue;
use crate::follow::model::Follow;
use crate::utils::{gen_id, set_timer_days};
use crate::follow::{delete_follow, create_follow};

#[post("/follow/<id>")]
pub async fn create(id: String, conn: AtomicDB, auth: Session) -> JsonValue {
    let fol = Follow{
        id: gen_id(24),
        source: auth.0,
        target: id,
        created_on: set_timer_days(0)
    };
    json!({"status": create_follow(fol, &*conn)})
}

#[delete("/follow/delete/<id>")]
pub async fn delete(id: String, conn: AtomicDB, auth: Session) -> JsonValue {
    let fol = Follow{
        id: gen_id(24),
        source: auth.0,
        target: id,
        created_on: set_timer_days(0)
    };
    json!({"status": delete_follow(fol, &*conn)})
}