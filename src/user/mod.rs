use argon2::{self, Config};
use rand::{thread_rng, Rng, distributions::Alphanumeric};
use diesel::{PgConnection, QueryResult, prelude::*};
use super::schema::users::{*, dsl::{users}};
pub mod usermodel;
use usermodel::UserPublic;

pub fn get(uid: String, connection: &PgConnection) -> QueryResult<UserPublic>{
    users.find(&uid).select((id, url, nickname, first_name, last_name)).first::<UserPublic>(connection)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserErrors {
    MissingUID,
    InvalidUID,
    UnknownInformation,
    NoEmailSet,
}

pub fn hash_pass(pass: String) -> String{
    let salt: String = thread_rng().sample_iter(&Alphanumeric).take(64).collect();
    let config= Config::default();
    argon2::hash_encoded(pass.as_ref(), salt.as_ref(), &config).unwrap()
}
