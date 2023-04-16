use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum DataError {
    #[display(fmt = "User already exists")]
    DuplicateUser,
    #[display(fmt = "Book already exists")]
    DuplicateBook,
    #[display(fmt = "User not found")]
    UserNotFound,
    #[display(fmt = "<h1>404 Error</h1>Book not found")]
    BookNotFound,
}

impl error::ResponseError for DataError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            DataError::UserNotFound => StatusCode::NOT_FOUND,
            DataError::DuplicateUser => StatusCode::CONFLICT,
            DataError::DuplicateBook => StatusCode::CONFLICT,
            DataError::BookNotFound => StatusCode::NOT_FOUND
        }
    }
}