use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Deserialize, Serialize, Clone, Default)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        email(message = "Email is invalid"),
        length(min = 1, message = "Email is required")
    )]
    pub email: String,
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters"),
        length(min = 1, message = "Password is required")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Password confirm is required"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    pub password_confirm: String,
}
