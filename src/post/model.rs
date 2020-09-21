use crate::auth::model::Session;
use crate::routes::AtomicDB;
use crate::schema::posts;
use crate::utils::{gen_id, parse_all, set_timer_days};
use chrono::NaiveDateTime;
use multer::Multipart;
use regex::internal::Input;
use rocket::data::ByteUnit;
use rocket::data::{FromData, Outcome, ToByteUnit};
use rocket::futures::io::BufWriter;
use rocket::futures::TryFutureExt;
use rocket::http::Status;
use rocket::outcome::Outcome::Success;
use rocket::response::Stream;
use rocket::tokio;
use rocket::{Data, Request};
use std::convert::Infallible;
use std::io::{BufReader, Bytes};
use std::iter::once;
use std::str::FromStr;
use crate::media::model::{MediaType, Poll};

pub enum PostError {
    InvalidData,
    TooManyFields,
    TooFewFields,
    WrongType,
    MissingBoundary,
    Unsupported,
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
    pub created_on: NaiveDateTime,
}

#[derive(AsChangeset, Serialize, Deserialize, Clone)]
#[table_name = "posts"]
pub struct PostAlterable {
    pub id: String,
    pub public: Option<bool>,
    pub reshares: Option<bool>,
    pub comments: Option<bool>,
    pub content: Option<String>,
}

#[rocket::async_trait]
impl FromData for Post {
    type Error = PostError;
    async fn from_data(request: &Request<'_>, data: Data) -> Outcome<Self, Self::Error> {
        let cont = request.content_type().unwrap();
        let (_, boundary) = cont.params().find(|&(k, _)| k == "boundary").unwrap();
        let buf = data.open(8.megabytes());
        let mut multipart = Multipart::with_reader(buf, boundary);
        let auth = request.guard::<Session>().await.unwrap();
        let conn = request.guard::<AtomicDB>().await.unwrap();
        let mut post = Post {
            id: gen_id(21),
            source_type: None,
            source_id: None,
            public: None,
            reshares: None,
            comments: None,
            poster: auth.get_uid(&*conn),
            content: "".to_string(),
            created_on: set_timer_days(0),
        };
        //let mut media: Vec<Box<dyn MediaType>>;
        while let Some(field) = multipart.next_field().await.unwrap() {
            let name = field.name();
            //let mut poll  = false;
            let content_type = field.content_type().unwrap();
            if content_type.type_() == "image"{
                //Parse and Store -- soon(tm)
            }
            match name {
                Some("id") => post.id = field.text().await.unwrap(),
                Some("source_type") => {
                    post.source_type =
                        Some(i16::from_str(field.text().await.unwrap().as_str()).unwrap())
                }
                Some("source_id") => post.source_id = Some(field.text().await.unwrap()),
                Some("content") => post.content = parse_all(field.text().await.unwrap().as_str()),
                Some("public") => {
                    post.public =
                        Some(bool::from_str(field.text().await.unwrap().as_str()).unwrap())
                }
                Some("reshares") => {
                    post.reshares =
                        Some(bool::from_str(field.text().await.unwrap().as_str()).unwrap())
                }
                Some("comments") => {
                    post.comments =
                        Some(bool::from_str(field.text().await.unwrap().as_str()).unwrap())
                }
                /*Some("poll") => {
                    if !poll {
                        let p: Poll = serde_json::from_str(field.text().await.unwrap().as_str()).unwrap();
                        //Max Options of 4
                        if p.options.len() == 0 || p.options.len() > 4{
                            return Outcome::Failure((Status::BadRequest, PostError::InvalidData))
                        }
                        media.push(
                            Box::new(p)
                        );
                        poll = true;
                    }
                }*/
                _ => println!("Unsupported Type: {}", name.unwrap())
            }
        }
        if post.content == "".to_string() {
            return Outcome::Failure((Status::BadRequest, PostError::InvalidData));
        }
        Outcome::Success(post)
    }
}

impl PostAlterable {
    pub fn sanitize(&mut self) {
        let content = self.content.as_ref().unwrap();
        if self.content != None {
            self.content = Some(parse_all(content.as_str()))
        }
    }
}
