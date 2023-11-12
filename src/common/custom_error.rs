use std::fmt::Debug;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub enum CustomErrorType {
    SeaOrmError,
    ValidationError,
    Serialize
}

#[derive(Debug,Serialize,Deserialize)]
pub struct CustomError {
    pub message: Option<String>,
    pub err_type: CustomErrorType,
}

impl CustomError {
    pub fn message(&self) -> String {
        match &self.message {
            Some(c) => c.clone(),
            None => String::from(""),
        }
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<validator::ValidationErrors> for CustomError {
    fn from(err: validator::ValidationErrors) -> CustomError {
        let message = err.field_errors().into_iter()
            .map(|v|{
                let message:String = v.1.into_iter().map(|er|{
                    let message = match er.clone().message{
                        Some(val)=>val.to_string(),
                        None=>String::from("<no message>")
                    };
                    return format!("{}",message)
                }).collect();
                return format!("Field {} {} ",v.0,message);
            }).collect();
        CustomError {
            message: Some(message),
            err_type: CustomErrorType::ValidationError,
        }
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self.err_type {
            CustomErrorType::SeaOrmError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrorType::ValidationError => StatusCode::BAD_REQUEST,
            CustomErrorType::Serialize => StatusCode::BAD_REQUEST
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}