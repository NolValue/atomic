use argon2::{hash_encoded, Config};
use chrono::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;

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

pub fn hash_pass(password: String) -> String {
    let salt: String = thread_rng().sample_iter(&Alphanumeric).take(64).collect();
    let config = Config::default();
    hash_encoded(password.as_ref(), salt.as_ref(), &config).unwrap()
}

pub fn test_replace() -> String {
    let str = r#"<This> &is \**A* _test_"#;
    md_parse(html_parse(str).as_str())
}

/** Escape and Process HTML **/
fn html_parse(str: &str) -> String {
    escape_forward(escape_single(escape_quote(escape_lt(escape_gt(
        escape_amp(str),
    )))))
}
fn escape_amp(str: &str) -> String {
    lazy_static! {
        static ref escapeamp: Regex = Regex::new(r"&").unwrap();
    }
    escapeamp.replace_all(str, "&amp;").to_string()
}

fn escape_gt(str: String) -> String {
    lazy_static! {
        static ref escapegt: Regex = Regex::new(r">").unwrap();
    }
    escapegt.replace_all(&str, "&gt;").to_string()
}

fn escape_lt(str: String) -> String {
    lazy_static! {
        static ref escapelt: Regex = Regex::new(r"<").unwrap();
    }
    escapelt.replace_all(&str, "&lt;").to_string()
}
fn escape_quote(str: String) -> String {
    lazy_static! {
        static ref escapequote: Regex = Regex::new(r#"""#).unwrap();
    }
    escapequote.replace_all(&str, "&quot;").to_string()
}

fn escape_single(str: String) -> String {
    lazy_static! {
        static ref escapesingle: Regex = Regex::new(r"'").unwrap();
    }
    escapesingle.replace_all(&str, "&#x27;").to_string()
}
fn escape_forward(str: String) -> String {
    lazy_static! {
        static ref escapesingle: Regex = Regex::new(r"/").unwrap();
    }
    escapesingle.replace_all(&str, "&#x2F;").to_string()
}

/** Escape and Process Markdown **/
fn md_parse(str: &str) -> String {
    parse_strike(parse_italics(parse_bold(escape_strike(escape_italics(
        escape_bold(str),
    )))))
}
fn escape_bold(str: &str) -> String {
    lazy_static! {
        static ref escapebold: Regex = Regex::new(r"(\\\*)+").unwrap();
    }
    escapebold.replace_all(str, "&ast;").to_string()
}
fn escape_italics(str: String) -> String {
    lazy_static! {
        static ref escapeitalics: Regex = Regex::new(r"(\\_)+").unwrap();
    }
    escapeitalics.replace_all(&str, "&lowbar;").to_string()
}
fn escape_strike(str: String) -> String {
    lazy_static! {
        static ref escapestrike: Regex = Regex::new(r"(\\~)+").unwrap();
    }
    escapestrike.replace_all(&str, "&tilde").to_string()
}
fn parse_bold(str: String) -> String {
    lazy_static! {
        static ref bold: Regex = Regex::new(r"(?s)(\*(?P<inner>.*?)\*)").unwrap();
    }
    bold.replace_all(&str, "<b>$inner</b>").to_string()
}
fn parse_italics(str: String) -> String {
    lazy_static! {
        static ref italics: Regex = Regex::new(r"(?s)(_(?P<inner>.*?)_)").unwrap();
    }
    italics.replace_all(&str, "<em>$inner</em>").to_string()
}
fn parse_strike(str: String) -> String {
    lazy_static! {
        static ref strike: Regex = Regex::new(r"(?s)(~(?P<inner>.*?)~)").unwrap();
    }
    strike.replace_all(&str, "<s>$inner</s>").to_string()
}
