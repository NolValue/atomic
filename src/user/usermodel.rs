use argon2::{self, Config};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User{
    id: String,
    url: String,
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl User{
    fn hash_pass(password: String) -> String{
        let salt: String = thread_rng().sample_iter(&Alphanumeric).take(64).collect();
        let config= Config::default();
        argon2::hash_encoded(password.as_ref(), salt.as_ref(), &config).unwrap()
    }
    fn verify_pass(&self, password: String) -> bool{
        argon2::verify_encoded(&self.password, password.as_ref()).unwrap()
    }
}