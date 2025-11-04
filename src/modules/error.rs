use std::fmt;

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorReponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorReponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 1. Convert the ErrorReponse struct into a JSON String.
        let json_string = serde_json::to_string(self).unwrap();

        // 2. Write the resulting JSON string to the formatter 'f'.
        write!(f, "{}", json_string)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExeedMaxPasswordLength(usize),
    HasingError,
    InvalidHasFormat,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvide,
    PermissionDenied,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl Into<String> for ErrorMessage {
    fn into(self) -> String {
        self.to_string()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Server Error, Please try again".to_string(),
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::ExeedMaxPasswordLength(max_length) => "Password Max Length".to_string(),
            ErrorMessage::HasingError => "Encrypting error, please try again".to_string(),
            ErrorMessage::InvalidHasFormat => "Invalid hashing format".to_string(),
            ErrorMessage::InvalidToken => "Invalid token provided".to_string(),
            ErrorMessage::WrongCredentials => "Wrong credentials provided".to_string(),
            ErrorMessage::EmailExist => "Email already exist".to_string(),
            ErrorMessage::UserNoLongerExist => "User no longer exist".to_string(),
            ErrorMessage::TokenNotProvide => "Token not provided".to_string(),
            ErrorMessage::PermissionDenied => "Permission denied".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: u16,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: u16) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }
    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError::new(message, 500)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError::new(message, 400)
    }

    pub fn unique_contraint_voilation(message: impl Into<String>) -> Self {
        HttpError::new(message, 409)
    }

    pub fn un_authorize(message: impl Into<String>) -> Self {
        HttpError::new(message, 401)
    }
    pub fn not_found(message: impl Into<String>) -> Self {
        HttpError::new(message, 404)
    }

    pub fn into_http_response(self) -> HttpResponse {
        match self.status {
            400 => HttpResponse::BadRequest().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            401 => HttpResponse::Unauthorized().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            409 => HttpResponse::Conflict().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            500 => HttpResponse::InternalServerError().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            _ => {
                eprintln!(
                    "Warning missing pattern match, status: {} code",
                    self.status
                );
                HttpResponse::InternalServerError().json(Response {
                    status: "fail",
                    message: self.message.into(),
                })
            }
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Http Server Error {} {}", self.message, self.status)
    }
}

impl std::error::Error for HttpError {}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let cloned = self.clone();
        cloned.into_http_response()
    }
}
