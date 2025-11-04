use crate::modules::error::ErrorMessage;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

const MAX_PASSWORD_LENGTH: usize = 64;

pub fn hash(password: impl Into<String>) -> Result<String, ErrorMessage> {
    let password: String = password.into();
    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExeedMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ErrorMessage::HasingError)?
        .to_string();

    Ok(hash_password)
}

pub fn compare(password: &str, password_hash_string: &str) -> Result<bool, ErrorMessage> {
    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExeedMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }

    let password_hash =
        PasswordHash::new(password_hash_string).map_err(|_| ErrorMessage::InvalidHasFormat)?;

    let password_match = Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .map_or(false, |_| true);

    Ok(password_match)
}
