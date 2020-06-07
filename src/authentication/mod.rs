use crate::user::usermodel::UserAuthable;
use diesel::{PgConnection, QueryResult, QueryDsl, ExpressionMethods, RunQueryDsl};
use crate::authentication::authmodel::{Auth, AuthInsertable};
use super::schema::users::{*, dsl::{users}};
use super::schema::users::id;
use super::schema::auths::dsl::*;
use crate::user::{verify_pass, generate_id};
use std::borrow::Borrow;

pub mod authmodel;

pub fn auth_user(user: UserAuthable, connection: &PgConnection) -> Auth {
    let usid = users.filter(email.eq(&user.email)).select(id).first::<String>(connection).unwrap();
    let pass = users.filter(email.eq(&user.email)).select(password).first::<String>(connection).unwrap();
    let mut auth = Auth{
        uid: usid,
        refresh_token: generate_id(32),
        access_token: generate_id(36)
    };
    match verify_pass(user.password, pass){
        true => new_auth(auth, connection),
        false => return Auth{ uid: "failure".to_string(), refresh_token: "failure".to_string(), access_token: "failure".to_string()}
    }
}

pub fn new_auth(user: Auth, connection: &PgConnection) -> Auth {
    let data = diesel::insert_into(auths)
        .values(&AuthInsertable::from_auth(&user))
        .execute(connection).unwrap();
    match data{
        0 => Auth{ uid: "failure".to_string(), refresh_token: "failure".to_string(), access_token: "failure".to_string()},
        1 => user,
        _ => Auth{ uid: "failure".to_string(), refresh_token: "failure".to_string(), access_token: "failure".to_string()}
    }
}