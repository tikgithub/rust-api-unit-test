#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_max_age: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET must be set");
        let jwt_max_age = std::env::var("JWT_MAX_AGE")
            .unwrap_or("3600".to_string())
            .parse::<i64>()
            .expect("JWT_MAX_AGE must be a valid integer");

        return Config {
            database_url,
            jwt_secret,
            jwt_max_age,
            port: 8080,
        };
    }
}
