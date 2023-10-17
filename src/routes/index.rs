use crate::common::response;
use actix_web::{ web,get,Responder,Result,Scope};
use serde::Serialize;
use std::option::Option::Some;
use crate::AppState;

#[derive(Serialize)]
pub struct IndexResponse{
    pub hello:String
}

#[get("/")]
pub async fn hello(
    _state:web::Data<AppState>
)->Result<impl Responder>{
    let obj = response::BaseResponse{
        status_code:200,
        message:String::from("Hehe"),
        data:Some(IndexResponse{
            hello:String::from("Trian")
        })
    };
    Ok(web::Json(obj))
}

pub fn index_handler()->Scope{
 web::scope("/").service(hello)
}