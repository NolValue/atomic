use crate::schema::posts;
use chrono::NaiveDateTime;
#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: String,
    pub source_type: Option<i16>,
    pub source_id: Option<String>,
    pub public: Option<bool>,
    pub reshares: Option<bool>,
    pub comments: Option<bool>,
    pub poster: String,
    pub content: String,
    pub created_on: NaiveDateTime
}