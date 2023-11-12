use std::ops::Add;

use chrono::{Duration, Local};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, errors::Error as JwtError, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn encode(
    sub:String
) -> Option<String> {
    let secret = std::env::var("JWT_SECRET").unwrap_or("trian".to_string());
    let exp = Local::now().add(Duration::hours(1)).timestamp();
    let claims = Claims {
        iss: "bluhabit.id".to_string(),
        sub,
        iat: Local::now().timestamp(),
        exp,
    };

    let token = jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ).unwrap();
    Some(token)
}

pub fn decode(
    token: String
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").unwrap_or("trian".to_string());
    let decoded:Result<TokenData<Claims>,JwtError> = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256)
    );
    decoded
}