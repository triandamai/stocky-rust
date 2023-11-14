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
pub struct SignUpBasicRequest {
    #[validate(email)]
    pub email:String,
    #[validate(length(min=6))]
    pub password:String,
    #[validate(length(min=1))]
    pub full_name:String
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct VerifyOtpRequest {
    #[validate(length(min=6))]
    pub session_id:String,
    #[validate(length(min=4))]
    pub code:String
}
