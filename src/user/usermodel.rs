use super::super::schema::users;
use crate::user::{hash_pass, generate_id, UserErrors};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserAuthable{
    email: String,
    password: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct User{
    id: String,
    url: Option<String>,
    nickname: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserInsertable{
    id: String,
    email: String,
    password: String,
}

impl UserInsertable{
    pub fn from_authable(user: UserAuthable) -> UserInsertable{
        UserInsertable {
            id: generate_id(23),
            email: user.email,
            password: hash_pass(user.password),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserAuthable {
    type Error = UserErrors;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("data").collect();
        let user = serde_json::from_str::<UserAuthable>(keys[0].to_string().as_ref()).unwrap();
        Outcome::Success(user)
    }
}