use crate::schema::users;
use crate::schema::users::columns::password;
use crate::utils::hash_pass;
use argon2::{hash_encoded, verify_encoded, Config};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket_contrib::json::JsonValue;

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub address: String,
    pub password: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub url: Option<String>,
    pub nickname: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset, Serialize, Deserialize, Clone)]
#[table_name = "users"]
pub struct UserAlterable {
    pub url: Option<String>,
    pub nickname: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl UserAlterable {
    pub fn sanitize(&mut self) {
        if self.password != None {
            self.password = Some(hash_pass(self.password.clone().unwrap()))
        }
    }
}
impl User {
    pub fn to_public(self) -> JsonValue {
        json!({"id": self.id, "url": self.url, "nickname": self.nickname, "first_name": self.first_name, "last_name": self.last_name})
    }
    pub fn verify_pass(self, pwd: String) -> bool {
        verify_encoded(self.password.as_str(), pwd.as_ref()).unwrap()
    }
}
