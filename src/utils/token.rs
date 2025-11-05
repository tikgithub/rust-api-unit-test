use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn create_token(
    user_id: &str,
    secret: &[u8],
    expired_in_second: i64,
) -> Result<String, jsonwebtoken::errors::Error> {

    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(expired_in_second)).timestamp() as usize;
    let claim: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        exp: exp,
        iat: iat,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret)
    )

}
