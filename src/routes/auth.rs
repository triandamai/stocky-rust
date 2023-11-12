extern crate bcrypt;

use actix_web::{Responder, Result, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use validator::Validate;

use crate::AppState;
use crate::common::jwt::encode;
use crate::common::response::{BaseResponse, ErrorResponse};
use crate::entity::sea_orm_active_enums::{AuthProvider, Status};
use crate::models::auth::SignInBasicRequest;
use crate::repositories::auth::AuthRepository;


pub async fn get_token() -> Result<impl Responder> {
    let token = encode(
        "triandamai".to_string()
    ).unwrap();
    return Ok(web::Json(BaseResponse::<String> {
        status_code: 400,
        message: String::from("Success"),
        data: Some(token),
    }));
}

pub async fn sign_in_basic(
    state:web::Data<AppState>,
    body:web::Json<SignInBasicRequest>
)->Result<impl Responder, ErrorResponse>{
    //validate request
    let validate_body = body.validate();
    if body.validate().is_err(){
        return Err(ErrorResponse::bad_request(2000,validate_body.unwrap_err().to_string()));
    }

    //find related account
    let auth_repo = AuthRepository::init(&state);
    let find_user = auth_repo.get_user_by_email(&body.email).await;
    if find_user.is_none(){
        return Err(ErrorResponse::unauthorized("Cannot find user ".to_string()));
    }

    let user = find_user.clone().unwrap();
    //make sure user is authenticated
    let req_password = hash(&body.password,DEFAULT_COST).unwrap_or("".to_string());
    let password_match = verify(user.password, &req_password).unwrap_or(false);
    if !password_match {
        return Err(ErrorResponse::unauthorized("Username or password invalid ".to_string()));
    }

    //make sure account is using basic auth(email&password)
    if user.auth_provider != AuthProvider::Basic{
        return Err(ErrorResponse::forbidden(1000,"Email used by another account".to_string()));
    }
    if user.status == Status::Suspended{
       return  Err(ErrorResponse::forbidden(1002,"Your account suspended".to_string()));
    }
    if user.status != Status::Inactive{
        return Err(ErrorResponse::forbidden(1003,"Your account is inactive".to_string()));
    }

    Ok(web::Json(BaseResponse{
        status_code:200,
        message:"".to_string(),
        data: Some("Tes".to_string())
    }))
}


pub fn auth_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/generate", web::get().to(get_token))
            .route("/sign-in-basic",web::post().to(sign_in_basic))
    );
}
