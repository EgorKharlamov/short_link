use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub api_port: String,
    pub db_host: String,
    pub db_port: String,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
    pub database_url: String,
    pub client_url: String,
}

impl Config {
    pub fn default() -> Config {
        dotenv().ok();
        Config {
            api_port: dotenvy::var("API_PORT").expect("Where is your api_port?"),
            db_host: dotenvy::var("DB_HOST").expect("Where is your db_host?"),
            db_port: dotenvy::var("DB_PORT").expect("Where is your db_port?"),
            db_username: dotenvy::var("DB_USERNAME").expect("Where is your db_username?"),
            db_password: dotenvy::var("DB_PASSWORD").expect("Where is your db_password?"),
            db_name: dotenvy::var("DB_NAME").expect("Where is your db_name?"),
            database_url: dotenvy::var("DATABASE_URL").expect("Where is your DATABASE_URL?"),
            client_url: dotenvy::var("CLIENT_URL").expect("Where is your CLIENT_URL?"),
        }
    }
}
