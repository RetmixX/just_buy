use crate::DBPool;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone)]
pub struct Database{
   pub(in self::super) pool_connection: DBPool
}

impl Database{
    pub fn new(pool: DBPool) -> Self {
        Database{
            pool_connection: pool
        }
    }
}