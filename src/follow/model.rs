use crate::schema::follows;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
pub struct Follow {
    pub id: String,
    pub source: String,
    pub target: String,
    pub created_on: NaiveDateTime
}

