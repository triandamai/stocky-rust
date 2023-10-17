use std::option::Option;
use serde::Serialize;

#[derive(Serialize,Debug)]
pub struct BaseResponse<T>{
    pub status_code:i32,
    pub message:String,
    pub data:Option<T>
}