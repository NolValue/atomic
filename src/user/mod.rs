pub mod model;
pub mod routes;
use super::schema::users::dsl::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use model::{User, UserLogin};

pub fn get_by_id(uid: String, conn: &PgConnection) -> QueryResult<User> {
    users.find(uid).first::<User>(&*conn)
}

pub fn get_by_login(ul: UserLogin, conn: &PgConnection) -> Result<User, String> {
    let usr: QueryResult<User> = users.filter(email.eq(ul.address)).first::<User>(&*conn);
    match usr {
        Ok(u) if { u.clone().verify_pass(ul.password) } => Ok(u),
        _ => return Err("Failure".to_string()),
    }
}
