use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct DBClient {
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> DBClient {
        DBClient { pool }
    }
}
