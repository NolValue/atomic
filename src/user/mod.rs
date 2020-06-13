pub mod model;
use crate::schema::users::dsl::*;
use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use model::User;

pub fn get_by_id(uid: String, conn: &PgConnection) -> QueryResult<User> {
    users.find(uid).first::<User>(&*conn)
}
