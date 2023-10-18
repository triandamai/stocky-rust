use actix_web::{get, post, web, Responder, Result, Scope};
use chrono::Local;
use sea_orm::ActiveValue::Set;

use entity::user;
use uuid::Uuid;
use validator::Validate;

use crate::{
    common::response::BaseResponse, common::utils::get_validation_message, models, AppState,
};

#[get("/get")]
pub async fn get_users(state: web::Data<AppState>) -> Result<impl Responder> {
    let users = state.user_repository.get_users().await;

    Ok(web::Json(users))
}

#[get("/get-user-info/{info_id}")]
pub async fn get_user_info_by_id(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let info_id = path.into_inner();
    let users = state.user_repository.get_user_info_by_id(&info_id).await;

    if users.is_err() {
        return Ok(web::Json(BaseResponse {
            status_code: 400,
            data: None,
            message: String::from("Muehehe"),
        }));
    }
    return Ok(web::Json(BaseResponse {
        status_code: 400,
        data: Some(users.unwrap()),
        message: String::from("Muehehe"),
    }));
}

#[post("/insert-user-info")]
pub async fn insert_user_info(
    state: web::Data<AppState>,
    req: web::Json<models::user::AddUserInfoRequest>,
) -> Result<impl Responder> {
    let result = state
        .user_repository
        .insert_user_info(&req.user_id, &req.user_info)
        .await;

    if result.is_err() {
        return Ok(web::Json(BaseResponse {
            status_code: 400,
            data: None,
            message: String::from("Muehehe"),
        }));
    }

    return Ok(web::Json(BaseResponse {
        status_code: 200,
        data: Some(result.unwrap()),
        message: String::from("Muehehe"),
    }));
}

#[get("/insert")]
pub async fn insert_user(state: web::Data<AppState>) -> Result<impl Responder> {
    let current_date = Local::now().naive_local();

    let id = Uuid::new_v4().as_simple().to_string();
    let data = user::ActiveModel {
        id: Set(String::from(id)),
        full_name: Set(String::from("Trian Damai")),
        email: Set(String::from("triandamai@gmail.com")),
        password: Set(String::from("12345678")),
        created_at: Set(Some(current_date)),
        updated_at: Set(Some(current_date)),
    };

    let user = state.user_repository.insert_user(data).await;

    return Ok(web::Json(BaseResponse {
        status_code: 200,
        data: user,
        message: String::from("Muehehe"),
    }));
}

#[post("/sign_in_with_email")]
pub async fn sign_in_with_email(
    state: web::Data<AppState>,
    req: web::Json<models::auth::SignRequest>,
) -> Result<impl Responder> {
    let is_valid = req.validate();
    if is_valid.is_err() {
        return Ok(web::Json(BaseResponse {
            status_code: 400,
            message: get_validation_message(&is_valid.unwrap_err()),
            data: None,
        }));
    }

    let find_user = state.user_repository.get_user_by_email(&req.email).await;

    if find_user.is_err() {
        return Ok(web::Json(BaseResponse {
            status_code: 400,
            message: String::from(&find_user.unwrap_err().to_string()),
            data: None,
        }));
    }

    return match find_user.unwrap() {
        None => Ok(web::Json(BaseResponse {
            status_code: 400,
            message: String::from("Cannot found user"),
            data: None,
        })),
        Some(value) => {
            if !value.password.eq(&req.password) {
                return Ok(web::Json(BaseResponse {
                    status_code: 400,
                    message: String::from("Password or username wrong"),
                    data: None,
                }));
            }
            return Ok(web::Json(BaseResponse {
                status_code: 400,
                message: String::from(""),
                data: Some(value),
            }));
        }
    };
}

pub fn user_handler() -> Scope {
    web::scope("/user")
        .service(get_users)
        .service(insert_user)
        .service(sign_in_with_email)
        .service(insert_user_info)
        .service(get_user_info_by_id)
}
