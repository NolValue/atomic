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
use crate::utils::{set_timer_days, gen_id};
use rocket::futures::io::BufWriter;
use rocket::tokio;
use std::iter::once;
use std::convert::Infallible;
use crate::auth::model::Session;
use crate::routes::AtomicDB;
use std::str::FromStr;

pub enum PostError{
    InvalidData,
    TooManyFields,
    TooFewFields,
    WrongType,
    MissingBoundary,
    Unsupported
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
        let mut multipart = Multipart::with_reader(buf, boundary);
        let auth = request.guard::<Session>().await.unwrap();
        let conn = request.guard::<AtomicDB>().await.unwrap();
        let mut post = Post{
            id: gen_id(21),
            source_type: None,
            source_id: None,
            public: None,
            reshares: None,
            comments: None,
            poster: auth.get_uid(&*conn),
            content: "".to_string(),
            created_on: set_timer_days(0)
        };
        while let Some(field) = multipart.next_field().await.unwrap() {
            let name = field.name();
            match name{
                Some("source_type") => post.source_type = Some(i16::from_str(field.text().await.unwrap().as_str()).unwrap()),
                Some("source_id") => post.source_id = Some(field.text().await.unwrap()),
                Some("content") => post.content = field.text().await.unwrap(),
                Some("public") => post.public = Some(bool::from_str(field.text().await.unwrap().as_str()).unwrap()),
                Some("reshares") => post.reshares = Some(bool::from_str(field.text().await.unwrap().as_str()).unwrap()),
                Some("comments") => post.comments = Some(bool::from_str(field.text().await.unwrap().as_str()).unwrap()),
                _ => return Outcome::Failure((Status::BadRequest, PostError::Unsupported))
            }
        }
        return Success(post)
    }
}