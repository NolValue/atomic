#[derive(Debug, Serialize, Deserialize)]
pub enum UserErrors {
    MissingUID,
    InvalidUID,
}