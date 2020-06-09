use crate::schema::users;
use rocket_contrib::json::JsonValue;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct User{
    pub id: String,
    pub url: Option<String>,
    pub nickname: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
}

impl User{
    pub fn public_json(self) -> JsonValue{
        json!({"id": self.id, "url": self.url, "nickname": self.nickname, "first_name": self.first_name, "last_name": self.last_name})
    }
}