use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
}

pub fn load_config() -> Config {
    dotenv::dotenv().ok();

    Config {
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set in environment"),
    }
}
