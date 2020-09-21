pub mod model;
pub mod routes;
use crate::schema::follows::dsl::*;
use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use model::Follow;

pub fn create_follow(mut fol: Follow, conn: &PgConnection) -> i32 {
    if !is_following(fol.clone(), &*conn) {
        return match diesel::insert_into(follows).values(fol).execute(conn) {
            Ok(_t) => 200,
            Err(_e) => 500,
        }
    }
    500
}

pub fn delete_f_by_uid(mut fol: String, conn: &PgConnection) -> i32 {
    match diesel::delete(follows.filter(source.eq(fol.clone())).or_filter(target.eq(fol))).execute(&*conn){
        Ok(_t) => 200,
        Err(_e) => 500,
    }
}

pub fn is_following(mut fol: Follow, conn: &PgConnection) -> bool{
    let count: Vec<String> = follows
        .filter(source.eq(fol.source).and(target.eq(fol.target)))
        .select(id)
        .load(&*conn).unwrap();
    match count.len(){
        0 => false,
        _ => true
    }
}

pub fn delete_follow(mut fol: Follow, conn: &PgConnection) -> i32 {
    match diesel::delete(follows.filter(source.eq(fol.source).and(target.eq(fol.target)))).execute(&*conn) {
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
