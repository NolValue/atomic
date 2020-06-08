use crate::user::usermodel::UserAuthable;
use diesel::{PgConnection, QueryDsl, ExpressionMethods, RunQueryDsl, QueryResult};
use crate::authentication::authmodel::{AuthSerializable, AuthInsertable, Auth};
use super::schema::users::{*, dsl::{users}};
use super::schema::users::id;
use super::schema::auths::dsl::*;
use super::schema::auths;
use crate::user::{verify_pass, generate_id};
use crate::routes::AtomicDB;

pub mod authmodel;

pub fn auth_user(user: UserAuthable, connection: &PgConnection) -> AuthSerializable {
    let usid = users.filter(email.eq(&user.email)).select(id).first::<String>(connection).unwrap();
    let pass = users.filter(email.eq(&user.email)).select(password).first::<String>(connection).unwrap();
    let auth = AuthSerializable {
        uid: usid,
        refresh_token: generate_id(32),
        access_token: generate_id(36)
    };
    match verify_pass(user.password, pass){
        true => new_auth(auth, connection),
        false => return AuthSerializable { uid: "failure".to_string(), refresh_token: "failure".to_string(), access_token: "failure".to_string()}
    }
}

pub fn new_auth(user: AuthSerializable, connection: &PgConnection) -> AuthSerializable {
    let data = diesel::insert_into(auths)
        .values(&AuthInsertable::from_auth(&user))
        .execute(connection).unwrap();
    match data{
        0 => AuthSerializable { uid: "failure".to_string(), refresh_token: "failure".to_string(), access_token: "failure".to_string()},
        1 => user,
        _ => AuthSerializable { uid: "failure".to_string(), refresh_token: "failure".to_string(), access_token: "failure".to_string()}
    }
}
pub fn refresh_auth(login: String, connection: &PgConnection) -> AuthSerializable{
    let token = generate_id(36);
    let result = diesel::update(auths.filter(refresh_token.eq(&login)))
        .set(access_token.eq(&token))
        .execute(connection).unwrap();
    AuthSerializable {
        uid: "".to_string(),
        refresh_token: (&login).to_string(),
        access_token: auths.filter(refresh_token.eq(&login)).select(auths::id).first::<String>(connection).unwrap()
    }
}

pub fn validate_auth(cookie: String, connection: &PgConnection) -> bool {
    let data: QueryResult<String> = auths.filter(refresh_token.eq(&cookie)).select(auths::id).limit(1).get_result(connection);
    match data{
        Ok(_e) => true,
        Err(_e) => false
    }
}