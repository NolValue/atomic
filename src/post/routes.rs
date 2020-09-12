use crate::post::model::{Post, PostAlterable};
use crate::routes::AtomicDB;
use crate::auth::model::Session;
use rocket_contrib::json::{JsonValue, Json};
use crate::post::{create_post, update_post, delete_post};
use crate::post;
use crate::auth;

#[post("/post", data = "<post>")]
pub async fn create(mut post: Post, conn: AtomicDB, auth: Session) -> JsonValue {
    json!({"status": create_post(post, &*conn)})
}

#[post("/post/update", format = "json", data = "<post>")]
pub async fn update(mut post: Json<PostAlterable>, conn: AtomicDB, auth: Session) -> JsonValue {
    let mut post = post.0;
    let pid = post.id.clone();
    if post::get_uid(pid.clone(), &*conn) == auth::get_uid(auth.0, &*conn){
        post.sanitize();
        json!({ "status": update_post(post, pid,  &*conn) })
    }
    else{
        json!({ "status": 500 })
    }
}

#[post("/post/delete/<pid>")]
pub async fn delete(pid: String, conn: AtomicDB, auth: Session) -> JsonValue{
    if post::get_uid(pid.clone(), &*conn) == auth::get_uid(auth.0, &*conn) {
        json!({ "status": delete_post(pid, &*conn) })
    }
    else{
        json!({ "status": 500 })
    }
}