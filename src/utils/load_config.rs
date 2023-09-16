use dotenv::dotenv;
use std::env;
use diesel::{PgConnection};
use diesel::r2d2::ConnectionManager;

pub fn setup_connection_pool() -> ConnectionManager<PgConnection>{
        ConnectionManager::<PgConnection>::new(load_env())
}

fn load_env() -> String{
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB URL NOT SET");
    //TODO: Create other .env
    db_url
}