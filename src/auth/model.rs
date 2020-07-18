use crate::schema::auths;
use crate::utils::*;
use rocket_contrib::json::JsonValue;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub refresh_token: String,
    pub access_token: String,
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
