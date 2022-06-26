use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::Serialize;
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug, Display, Error, Serialize)]
pub enum UserError {
    #[display(fmt = "Unauthorized.")]
    Unauthorized,
    #[display(fmt = "Invalid credentials.")]
    InvalidCredentials,
    #[display(fmt = "User does not exist.")]
    UserNotFoundError,
    #[display(fmt = "Email {} is already taken.", email)]
    EmailAlreadyExistsError { email: String },
    #[display(fmt = "{}", message)]
    ValidationError { message: String },
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::EmailAlreadyExistsError { .. } => StatusCode::BAD_REQUEST,
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            UserError::Unauthorized => StatusCode::UNAUTHORIZED,
            UserError::UserNotFoundError => StatusCode::NOT_FOUND,
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(json!({
                "error": self.to_string()
            }))
    }
}

impl From<ValidationErrors> for UserError {
    fn from(err: ValidationErrors) -> Self {
        UserError::ValidationError {
            message: err.to_string(),
        }
    }
}
