pub mod model;
pub mod routes;
use super::schema::users::dsl::*;
use crate::user::model::UserAlterable;
use crate::utils::{gen_id, hash_pass};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use model::{User, UserLogin};

pub fn get_by_id(uid: String, conn: &PgConnection) -> QueryResult<User> {
    users.find(uid).first::<User>(&*conn)
}

pub fn get_by_login(ul: UserLogin, conn: &PgConnection) -> Result<User, String> {
    match users.filter(email.eq(ul.address)).first::<User>(&*conn) {
        Ok(u) if { u.clone().verify_pass(ul.password) } => Ok(u),
        _ => return Err("Failure".to_string()),
    }
}
pub fn create_user(ul: UserLogin, conn: &PgConnection) -> i32 {
    let ul = (
        id.eq(gen_id(23)),
        email.eq(ul.address),
        password.eq(hash_pass(ul.password)),
    );
    match diesel::insert_into(users).values(ul).execute(conn) {
        Ok(_t) => 200,
        Err(_e) => 500,
    }
}

pub fn update_user(usr: UserAlterable, uid: String, conn: &PgConnection) -> i32 {
    return match diesel::update(users.filter(id.eq(uid)))
        .set(usr)
        .execute(conn)
    {
        Ok(_t) => 200,
        Err(_e) => 500,
    };
}
pub fn delete_user(uid: String, conn: &PgConnection) -> i32 {
    match diesel::delete(users.filter(id.eq(uid))).execute(&*conn) {
        Ok(_t) => 200,
        Err(_e) => 500,
    }
}
