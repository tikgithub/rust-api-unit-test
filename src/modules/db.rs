use async_trait::async_trait;
use sqlx::{Error, Pool, Postgres};
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
    async fn get_users(&self,page:u32,limit:u32)->Result<Vec<User>,sqlx::Error>;
    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    )-> Result<User, sqlx::Error>;
    async fn save_admin_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password:T,
    )->Result<User, sqlx::Error>;


    #[async_trait]
    impl UserExt for DBClient{
        async fn get_user() -> Result<Option<User>, Error> {
            todo!()
        }

        async fn get_users(&self, page: u32, limit: u32) -> Result<Vec<User>, Error> {
            todo!()
        }

        async fn save_user<T: Into<String> + Send>(&self, name: T, email: T, password: T) -> Result<User, Error> {
            todo!()
        }

        async fn save_admin_user<T: Into<String> + Send>(&self, name: T, email: T, password: T) -> Result<User, Error> {
            todo!()
        }
    }
}