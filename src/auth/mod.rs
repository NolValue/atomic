pub mod model;
use super::schema::auths::dsl::*;
use super::user::model::User;
use crate::utils::{gen_id, set_timer_days};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use model::*;

pub fn gen_auth(user: User, conn: &PgConnection) -> Result<Auth, String> {
    let a = Auth::from_user(user.id);
    match diesel::insert_into(auths).values(&a).execute(conn) {
        Ok(e) => Ok(a),
        Err(_e) => Err("Database Error".to_string()),
    }
}

pub fn update_auth(session: Session, conn: &PgConnection) {
    diesel::update(auths.filter(refresh_token.eq(session.refresh_token))).set((
        access_token.eq(gen_id(36)),
        auth_expiry.eq(set_timer_days(7)),
    ));
}
