use std::ops::Add;
use std::string::ToString;

use chrono::{Duration, Local};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, errors::Error as JwtError, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

pub const JWT_SECRET_KEY: &str = "JWT_SECRET";
pub const JWT_SECRET_KEY_DEFAULT: &str = "triandamai";
pub const ISS: &str = "bluhabit.id";

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    pub fn create(
        sub: String
    ) -> Claims {
        let exp = Local::now().add(Duration::hours(1)).timestamp();

        Claims {
            iss: ISS.to_string(),
            sub,
            iat: Local::now().timestamp(),
            exp,
        }
    }

    pub fn encode(&self) -> Option<String> {
        let secret = std::env::var(JWT_SECRET_KEY)
            .unwrap_or(JWT_SECRET_KEY_DEFAULT.to_string());

        let token = jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &self,
            &EncodingKey::from_secret(secret.as_ref()),
        );
        if token.is_err() {
            return None;
        }
        Some(token.unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct JwtUtils {
    value: String,
}

impl JwtUtils {
    pub fn from_token(token: String) -> JwtUtils {
        JwtUtils {
            value: token
        }
    }
    pub fn create_claim(sub: String) -> Claims {
        Claims::create(sub)
    }
    pub fn decode(
        &self
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let secret = std::env::var(JWT_SECRET_KEY)
            .unwrap_or(JWT_SECRET_KEY_DEFAULT.to_string());
        let decoded: Result<TokenData<Claims>, JwtError> = jsonwebtoken::decode::<Claims>(
            &self.value,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );
        decoded
    }
}
