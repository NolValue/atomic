pub mod model;
use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use crate::schema::users::dsl::*;
use model::User;

pub fn get_by_id(uid: String, conn: &PgConnection) -> QueryResult<User>{
    users.find(uid).first::<User>(&*conn)
}