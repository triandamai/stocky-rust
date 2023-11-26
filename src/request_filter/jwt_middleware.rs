use std::future::{ready, Ready};

use actix_web::{FromRequest, http, HttpRequest};
use actix_web::dev::Payload;
use jsonwebtoken::errors::Error;
use jsonwebtoken::TokenData;

use migration::async_trait::async_trait;

use crate::{common, common::response::ErrorResponse};
use crate::common::jwt::JwtUtils;

pub struct JwtMiddleware {
    pub session_id: String,
}

#[async_trait]
impl FromRequest for JwtMiddleware {
    //this should type to actix::web::Error but the return is string text/plain
    // so that we create custom error see response::ErrorResponse for the implementation
    type Error = ErrorResponse;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return ready(Err(ErrorResponse::unauthorized(
                "You are not logged in, please provide token".to_string(),
            )));
        }

        let claims = match JwtUtils::from_token(token.unwrap().to_string())
            .decode(){
            Ok(claims) => claims.claims,
            Err(error) => return ready(Err(ErrorResponse::unauthorized(error.to_string())))
        };

        ready(Ok(JwtMiddleware { session_id: claims.sub }))
    }
}
