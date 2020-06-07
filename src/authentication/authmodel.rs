use chrono::{NaiveDateTime, DateTime, Local, Duration};
use crate::user::generate_id;
use std::ops::Add;
use super::super::schema::auths;
pub enum Scopes{
    Unimplemented
}

#[derive(Serialize, Deserialize)]
pub struct Auth{
    pub uid: String,
    pub refresh_token: String,
    pub access_token: String,
}

#[derive(Insertable)]
#[table_name = "auths"]
pub struct AuthInsertable{
    id: String,
    uid: String,
    refresh_token: String,
    access_token: String,
    auth_expiry: NaiveDateTime,
}

pub fn week() -> NaiveDateTime {
    let dt: DateTime<Local> = Local::now();
    let dt = dt.naive_local();
    let dt = dt + Duration::days(7);
    dt
}

impl AuthInsertable{
    pub fn from_auth(auth: &Auth) -> AuthInsertable{
        AuthInsertable{
            id: generate_id(27),
            uid: (&auth.uid).parse().unwrap(),
            refresh_token: (&auth.refresh_token).parse().unwrap(),
            access_token: (&auth.access_token).parse().unwrap(),
            auth_expiry: week(),
        }
    }
}