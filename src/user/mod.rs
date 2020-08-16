pub mod model;
pub mod routes;
use super::schema::users::dsl::*;
use crate::utils::{gen_id, hash_pass};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use model::{User, UserLogin};

pub fn get_by_id(uid: String, conn: &PgConnection) -> QueryResult<User> {
    users.find(uid).first::<User>(&*conn)
}

pub fn get_by_login(ul: UserLogin, conn: &PgConnection) -> Result<User, String> {
    match users.filter(email.eq(ul.address)).first::<User>(&*conn){
        Ok(u) if { u.clone().verify_pass(ul.password) } => Ok(u),
        _ => return Err("Failure".to_string()),
    }
}
pub fn create_user(ul: UserLogin, conn: &PgConnection) -> String {
    let ul = (
        id.eq(gen_id(23)),
        email.eq(ul.address),
        password.eq(hash_pass(ul.password)),
    );
    match diesel::insert_into(users).values(ul).execute(conn) {
        Ok(a) => "success".to_string(),
        Err(e) => e.to_string(),
    }
}

pub fn delete_user(uid: String, conn:&PgConnection) -> bool {
    let rslt = diesel::delete(users.filter(id.eq(uid))).execute(&*conn).unwrap();
    match rslt{
        1 => true,
        _ => false
    }
}