use std::env;

pub struct Config {
    pub db_file_path: String,
}

pub fn init() -> Config {
    Config {
        db_file_path: env::var("DB_FILE").unwrap_or(String::from("./db.json")),
    }
}
