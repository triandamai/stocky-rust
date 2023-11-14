use actix_web::{http::StatusCode, Error as ActixWebError, HttpResponse, ResponseError};
use core::fmt;
use serde::Serialize;
use std::option::Option;

#[derive(Serialize, Debug)]
pub struct BaseResponse<T> {
    pub status_code: u16,
    pub error_code: u16,
    pub data: Option<T>,
    pub message: String,
}

impl<T> BaseResponse<T> {
    pub fn created(status_code:u16,data:T,message:String)->BaseResponse<T>{
        BaseResponse::<T>{
            status_code,
            message,
            data:Some(data),
            error_code: status_code,
        }
    }

    pub fn success(status_code:u16,data:T,message:String)->BaseResponse<T>{
        BaseResponse::<T>{
            status_code,
            message,
            data:Some(data),
            error_code: status_code,
        }
    }

    pub fn not_found(status_code:u16,data:T,message:String)->BaseResponse<T>{
        BaseResponse::<T>{
            status_code,
            message,
            data:Some(data),
            error_code: status_code,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status_code: u16,
    pub error_code:u16,
    pub data: Option<String>,
    pub message: String,
}

impl ErrorResponse {
    pub fn create(status_code: u16, message: String) -> ErrorResponse {
        ErrorResponse {
            status_code,
            error_code:status_code,
            message,
            data: None,
        }
    }
    pub fn unauthorized( message: String) -> ErrorResponse {
        ErrorResponse {
            status_code:401,
            error_code:401,
            message,
            data: None,
        }
    }

    pub fn forbidden(error_code: u16, message: String) -> ErrorResponse {
        ErrorResponse {
            status_code:403,
            error_code,
            message,
            data: None,
        }
    }
    pub fn bad_request(error_code: u16, message: String) -> ErrorResponse {
        ErrorResponse {
            status_code:400,
            error_code,
            message,
            data: None,
        }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::json!(&self))
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        return StatusCode::from_u16(self.status_code).unwrap();
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl From<ActixWebError> for ErrorResponse {
    fn from(value: ActixWebError) -> ErrorResponse {
        return ErrorResponse::create(
            value.as_response_error().status_code().as_u16(),
            value.to_string(),
        );
    }
}
