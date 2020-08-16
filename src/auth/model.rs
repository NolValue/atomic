use crate::schema::auths;
use crate::utils::*;
use rocket_contrib::json::JsonValue;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, request, State};
use crate::auth::{get_uid, validate_auth};
use diesel::PgConnection;
use rocket::http::Status;
use crate::routes::AtomicDB;

#[derive(Serialize, Deserialize, Clone)]
pub struct Session(pub String);

#[derive(Serialize, Deserialize)]
pub struct SessionFull {
    pub refresh_token: String,
    pub access_token: String,
}

#[derive(Debug)]
pub enum SessionError {
    BadCount,
    Missing,
    Invalid,
}

#[derive(Insertable, Queryable)]
pub struct Auth {
    id: String,
    uid: String,
    pub refresh_token: String,
    pub access_token: String,
    auth_expiry: chrono::NaiveDateTime,
    nickname: Option<String>,
}

impl Auth {
    pub fn to_auth(&self) -> JsonValue {
        json!({"refresh_token": self.refresh_token, "access_token": self.access_token})
    }
    pub fn from_user(user: String) -> Auth {
        Auth {
            id: gen_id(27),
            uid: user,
            refresh_token: gen_id(32),
            access_token: gen_id(36),
            auth_expiry: set_timer_days(1),
            nickname: None,
        }
    }
}

impl Session{
    pub fn get_uid(self, conn: &PgConnection) -> String{
        get_uid(self.0, &*conn)
    }
    pub fn is_valid(self, conn: &PgConnection) -> bool {
        validate_auth(self.0, conn)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = SessionError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let conn = request.guard::<AtomicDB>().unwrap();
        match cookies.get("x-bearer-token"){
            Some(c) if validate_auth(c.clone().value().to_string(), &*conn) => Outcome::Success(Session(c.value().to_string())),
            Some(c) => Outcome::Failure((Status::Unauthorized, SessionError::Invalid)),
            None => Outcome::Failure((Status::BadRequest, SessionError::Missing)),
            _ => Outcome::Failure((Status::BadRequest, SessionError::Missing))
        }
    }
}
