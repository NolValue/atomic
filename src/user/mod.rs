use argon2::{self, Config, verify_encoded};
use rand::{thread_rng, Rng, distributions::Alphanumeric};
use diesel::{PgConnection, QueryResult, prelude::*};
use super::schema::users::{*, dsl::{users}};
pub mod usermodel;
use usermodel::User;
use crate::user::usermodel::{UserInsertable, UserAuthable};

pub fn get(uid: String, connection: &PgConnection) -> QueryResult<User>{
    users.find(&uid).select((id, url, nickname, first_name, last_name)).first::<User>(connection)
}

pub fn create(user: UserAuthable, connection: &PgConnection) -> usize {
    diesel::insert_into(users)
        .values(&UserInsertable::from_authable(user))
        .execute(connection).unwrap()
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UserErrors {
    MissingData,
    InvalidData,
    NoEmailSet,
    NoPasswordSet,
    IncorrectSize
}

pub fn generate_id(length: i64) -> String{
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             1234567890";
    let mut rng = rand::thread_rng();
    let pass: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    pass
}

pub fn verify_pass(pass: String, hash: String) -> bool{
    verify_encoded(hash.as_ref(), pass.as_ref()).unwrap()
}

pub fn hash_pass(pass: String) -> String{
    let salt: String = thread_rng().sample_iter(&Alphanumeric).take(64).collect();
    let config= Config::default();
    argon2::hash_encoded(pass.as_ref(), salt.as_ref(), &config).unwrap()
}
