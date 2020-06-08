use super::super::schema::users;
use crate::user::{hash_pass, generate_id, UserErrors};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::http::Status;
use rocket_contrib::json::JsonValue;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserAuthable{
    pub(crate) email: String,
    pub(crate) password: String,
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
pub fn valid_json(json: String) -> bool {
    match serde_json::from_str::<JsonValue>(json.as_ref()){
        Ok(_e) => true,
        Err(_e) => false
    }
}
impl<'a, 'r> FromRequest<'a, 'r> for UserAuthable {
    type Error = UserErrors;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("data").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest,UserErrors::MissingData)),
            1 if valid_json(keys[0].to_string()) => Outcome::Success(serde_json::from_str::<UserAuthable>(keys[0].to_string().as_ref()).unwrap()),
            1 => Outcome::Failure((Status::BadRequest, UserErrors::InvalidData)),
            _ => Outcome::Failure((Status::BadRequest, UserErrors::IncorrectSize))
        }
    }
}