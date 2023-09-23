use std::env;
use dotenv::dotenv;

pub struct ConfigDB {
    pub db_url: String,
    pub max_connection: usize,
}

pub struct ConfigToken {
    pub secret: String,
    pub exp: usize,
}

pub struct ConfigApp {
    pub db: ConfigDB,
    pub token: ConfigToken,
}

impl ConfigApp {
    pub fn new() -> Self {
        let (db_url, max_connection) = read_db_config();
        let (secret, exp) = read_token_config();
        Self {
            db: ConfigDB { db_url, max_connection },
            token: ConfigToken { secret, exp },
        }
    }
}


fn read_db_config() -> (String, usize) {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB URL NOT SET");
    let max_connection = env::var("MAX_CONNECTION")
        .expect("MAX_CONNECTION not set").parse::<usize>()
        .expect("Value in MAX_CONNECTION not a unsigned integer");

    (db_url, max_connection)
}

fn read_token_config() -> (String, usize) {
    dotenv().ok();

    let secret = env::var("TOKEN_SECRET").expect("TOKEN_SECRET not set");
    let exp = env::var("TOKEN_EXP").expect("TOKEN_EXP not set")
        .parse::<usize>().expect("Value in TOKEN_EXP not a unsigned integer");

    (secret, exp)
}