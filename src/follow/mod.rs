pub mod model;
pub mod routes;
use crate::schema::follows::dsl::*;
use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use model::Follow;

pub fn create_follow(mut fol: Follow, conn: &PgConnection) -> i32 {
    match diesel::insert_into(follows).values(fol).execute(conn) {
        Ok(_t) => 200,
        Err(_e) => 500,
    }
}

pub fn delete_follow(mut fol: Follow, conn: &PgConnection) -> i32 {
    match diesel::delete(follows.filter(source.eq(fol.source).and(target.eq(fol.target))))
        .execute(&*conn)
    {
        Ok(_t) => 200,
        Err(_e) => 500,
    }
}

pub fn get_following(fid: String, conn: &PgConnection) -> Vec<String> {
    follows
        .select(target)
        .filter(source.eq(fid))
        .order_by(created_on.desc())
        .load(&*conn)
        .unwrap()
}

pub fn get_followers(fid: String, conn: &PgConnection) -> Vec<String> {
    follows
        .filter(target.eq(fid))
        .select(source)
        .order_by(created_on.desc())
        .load::<String>(&*conn)
        .unwrap()
}
