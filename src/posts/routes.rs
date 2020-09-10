use crate::posts::model::Post;
use crate::routes::AtomicDB;
use crate::auth::model::Session;
use rocket_contrib::json::JsonValue;

#[post("/post", data = "<post>")]
pub async fn test(mut post: Post, conn: AtomicDB) -> JsonValue {
    json!({ "status": "Check Console" })
}