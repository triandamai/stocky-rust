use actix_web::{Responder, Result, web};
use actix_web::web::Json;

use crate::{AppState, common::response::BaseResponse, models, request_filter};
use crate::common::response::ErrorResponse;
use crate::repositories::auth::AuthRepository;

pub async fn get_users(
    state: web::Data<AppState>,
    jwt: request_filter::jwt_middleware::JwtMiddleware,
) -> Result<impl Responder, ErrorResponse> {
    let auth_repo = AuthRepository::init(&state);
    let user = auth_repo.get_current_sign_in(
        jwt.user_id.to_string()
    ).await;

    if user.is_none() {
        return Err(ErrorResponse::create(401, "cannot found user".to_string()));
    }

    let result = user.unwrap();
    Ok(Json(models::auth::SignInBasicRequest {
        email: result.0.id,
        password: result.0.token.unwrap(),
    }))
}

pub async fn sign_in_basic(
    _: web::Data<AppState>,
    _: web::Json<models::auth::SignInBasicRequest>,
) -> Result<impl Responder> {
    return Ok(Json(BaseResponse::success(
        200,
        Some(""),
        "".to_string()
    )));
}

pub fn user_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/get", web::get().to(get_users))
            .route("/sign-in-basic", web::post().to(sign_in_basic)),
    );
}
