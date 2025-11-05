use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use crate::modules::models::User;

#[derive(Debug, Clone)]
pub struct DBClient {
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> DBClient {
        DBClient { pool }
    }
}

#[async_trait]
pub trait UserExt{
    async fn get_user()-> Result<Option<User>,sqlx::Error>;
    
}