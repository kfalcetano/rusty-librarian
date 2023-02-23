use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum BasicError {
    #[display(fmt = "Internal error")]
    InternalError,

    #[display(fmt = "Bad request")]
    BadClientData,

    #[display(fmt = "Timeout")]
    Timeout,
}

impl error::ResponseError for BasicError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            BasicError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            BasicError::BadClientData => StatusCode::BAD_REQUEST,
            BasicError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[derive(Debug, Display, Error)]
pub enum DataError {
    #[display(fmt = "User already exists")]
    DuplicateUser,
    #[display(fmt = "Book already exists")]
    DuplicateBook
}

impl error::ResponseError for DataError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            DataError::DuplicateUser => StatusCode::CONFLICT,
            DataError::DuplicateBook => StatusCode::CONFLICT,
        }
    }
}