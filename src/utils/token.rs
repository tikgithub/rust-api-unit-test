use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::modules::error::HttpError;

#[derive(Debug, Serialize, Deserialize,Clone)]
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

pub fn decode_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, HttpError>{
    let decode = decode::<TokenClaims>(
        &token.into(),
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256)
    );

    match decode {
        Ok(token) => Ok(token.claims.sub),
        Err(_) => {}
    }

}
