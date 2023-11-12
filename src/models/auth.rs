use serde::{Serialize,Deserialize};
use validator::Validate;

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct SignInBasicRequest {
    #[validate(email)]
    pub email:String,
    #[validate(length(min=6))]
    pub password:String
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct SignUpRequest {
    #[validate(email)]
    pub email:String,
    #[validate(length(min=6))]
    pub password:String,
    pub full_name:String
}
