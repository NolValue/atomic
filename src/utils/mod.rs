use chrono::*;
use rand::Rng;
pub fn gen_id(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, charset.len());
            charset[idx] as char
        })
        .collect()
}

pub fn set_timer_days(amt: i64) -> NaiveDateTime {
    (Utc::now() + Duration::days(amt)).naive_utc()
}
