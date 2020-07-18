use crate::schema::users;
use argon2::{hash_encoded, verify_encoded, Config};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket_contrib::json::JsonValue;

#[derive(FromForm)]
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

impl User {
    pub fn to_public(self) -> JsonValue {
        json!({"id": self.id, "url": self.url, "nickname": self.nickname, "first_name": self.first_name, "last_name": self.last_name})
    }
    pub fn hash_pass(self) -> String {
        let salt: String = thread_rng().sample_iter(&Alphanumeric).take(64).collect();
        let config = Config::default();
        hash_encoded(self.password.as_ref(), salt.as_ref(), &config).unwrap()
    }
    pub fn verify_pass(self, pwd: String) -> bool {
        verify_encoded(self.password.as_str(), pwd.as_ref()).unwrap()
    }
}
