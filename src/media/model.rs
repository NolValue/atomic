use crate::schema::media;
use crate::utils::gen_id;
use serde_json::Value;

pub trait MediaType {
    fn to_media(&self, post: String) -> Media;
}

pub struct Media{
    id: String,
    post: String,
    content_type: i16,
    content: Value
}

#[derive(Serialize, Deserialize)]
pub struct Image{
    source: String //IE: cdn.hexasphere.com/images/test.jpg
}

#[derive(Serialize, Deserialize)]
pub struct Poll{
    pub options: Vec<String> //Vec of Option Names
}
impl MediaType for Poll{
    fn to_media(&self, post: String) -> Media {
        Media{
            id: gen_id(25),
            post: post,
            content_type: 1,
            content: serde_json::to_value(self).unwrap()
        }
    }
}
impl MediaType for Image{
    fn to_media(&self, post: String) -> Media {
        Media{
            id: gen_id(25),
            post: post,
            content_type: 1,
            content: serde_json::to_value(self).unwrap()
        }
    }
}