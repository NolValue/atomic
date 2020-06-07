use super::super::schema::users;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User{
    id: String,
    url: Option<String>,
    nickname: String,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserPublic{
    id: String,
    url: Option<String>,
    nickname: String,
    first_name: Option<String>,
    last_name: Option<String>
}