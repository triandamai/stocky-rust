use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct SignRequest {
    pub email:String,
    pub password:String
}
