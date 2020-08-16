pub mod model;
pub mod routes;
use super::schema::auths::dsl::*;
use super::user::model::User;
use crate::utils::{gen_id, set_timer_days};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use model::*;
use diesel::sql_types::Bool;

pub fn gen_auth(user: User, conn: &PgConnection) -> Result<Auth, String> {
    let a = Auth::from_user(user.id);
    match diesel::insert_into(auths).values(&a).execute(conn) {
        Ok(e) => Ok(a),
        Err(_e) => Err("Database Error".to_string()),
    }
}

pub fn get_uid(sess: String, conn: &PgConnection) -> String {
    auths.filter(access_token.eq(sess)).select(uid).first::<String>(conn).unwrap()
}

pub fn update_auth(session: SessionFull, conn: &PgConnection) -> Result<String, SessionError>{
    let newid = gen_id(36);
    match diesel::update(auths.filter(refresh_token.eq(session.refresh_token))).set((
        access_token.eq(newid.clone()),
        auth_expiry.eq(set_timer_days(7)),
    )).execute(&*conn){
        Ok(t) => Ok(newid),
        Err(e) => Err(SessionError::Invalid)
    }
}

pub fn validate_auth(access: String, conn: &PgConnection) -> bool{
    let auth = auths.filter(access_token.eq(access)).first::<Auth>(&*conn);
    match auth {
        Ok(a) => true,
        _ => false
    }
}

pub fn delete_auth(access: String, conn:&PgConnection) -> bool {
    let rslt = diesel::delete(auths.filter(access_token.eq(access))).execute(&*conn).unwrap();
    match rslt{
        1 => true,
        _ => false
    }
}

pub fn delete_auth_by_user(userid: String, conn:&PgConnection) -> bool {
    let rslt = diesel::delete(auths.filter(uid.eq(userid))).execute(&*conn).unwrap();
    match rslt{
        0 => false,
        _ => true
    }
}