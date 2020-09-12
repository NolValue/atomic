pub mod routes;
pub mod model;
use super::schema::posts::dsl::*;
use model::{Post, PostAlterable};
use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::utils::gen_id;

pub fn get_uid(pid: String, conn: &PgConnection) -> String {
    posts
        .filter(id.eq(pid))
        .select(poster)
        .first::<String>(conn)
        .unwrap()
}

pub fn create_post(mut post: Post, conn: &PgConnection) -> i32 {
    post.id = gen_id(21);
    match diesel::insert_into(posts).values(post).execute(conn) {
        Ok(_t) => 200,
        Err(_e) => 500,
    }
}

pub fn delete_post(pid: String, conn: &PgConnection) -> i32 {
    match diesel::delete(posts.filter(id.eq(pid))).execute(&*conn) {
        Ok(_t) => 200,
        Err(_e) => 500,
    }
}

pub fn update_post(pst: PostAlterable, pid: String,conn: &PgConnection) -> i32 {
    return match diesel::update(posts.filter(id.eq(pid)))
        .set(pst)
        .execute(conn)
    {
        Ok(_t) => 200,
        Err(_e) => 500,
    };
}