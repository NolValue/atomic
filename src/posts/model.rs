use crate::schema::posts;
use chrono::NaiveDateTime;
use rocket::data::{FromData, Outcome, ToByteUnit};
use rocket::response::Stream;
use rocket::{Request, Data};
use std::io::{BufReader, Bytes};
use rocket::http::Status;
use multer::Multipart;
use rocket::futures::TryFutureExt;
use rocket::data::ByteUnit;
use regex::internal::Input;
use rocket::outcome::Outcome::Success;
use crate::utils::set_timer_days;
use rocket::futures::io::BufWriter;
use rocket::tokio;
use std::iter::once;
use std::convert::Infallible;

pub enum PostError{
    InvalidData,
    TooManyFields,
    TooFewFields,
    WrongType,
    MissingBoundary
}
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

#[rocket::async_trait]
impl FromData for Post{
    type Error = PostError;
    async fn from_data(request: &Request<'_>, data: Data) -> Outcome<Self, Self::Error> {
        let cont = request.content_type().unwrap();
        let (_, boundary) = cont.params().find(|&(k, _)| k == "boundary").unwrap();
        let buf = data.open(8.megabytes());
        let multipart = Multipart::with_reader(buf, boundary);
        return Success(Post{
            id: "0515".to_string(),
            source_type: None,
            source_id: None,
            public: None,
            reshares: None,
            comments: None,
            poster: "1851".to_string(),
            content: "Your mom".to_string(),
            created_on: set_timer_days(0)
        })
    }
}