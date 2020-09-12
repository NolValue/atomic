pub mod model;
pub mod routes;
use model::Follow;
use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods, BoolExpressionMethods};
use crate::schema::follows::dsl::*;

pub fn create_follow(mut fol: Follow, conn: &PgConnection) -> i32 {
    match diesel::insert_into(follows).values(fol).execute(conn) {
        Ok(_t) => 200,
        Err(_e) => 500,
    }
}

pub fn delete_follow(mut fol: Follow, conn: &PgConnection) -> i32 {
    match diesel::delete(follows.filter(source.eq(fol.source).and(target.eq(fol.target)))).execute(&*conn) {
        Ok(_t) => 200,
        Err(_e) => 500,
    }
}