use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use crate::common::response::ErrorResponse;

mod common;
// mod entity;
mod entity;
mod models;
mod repositories;
mod request_filter;
mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
    secret: String,
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_BACKTRACE", "full".to_string());
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }
    let url = match std::env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => String::from("postgres://admin:password123@localhost:5432/actix"),
    };
    let secret = match std::env::var("JWT_SECRET") {
        Ok(val) => val,
        Err(_) => String::from("thisisverysecret"),
    };
    let db = Database::connect(url).await.unwrap();

    let state = AppState { db: db.clone(), secret };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .app_data(web::Data::new(String::from("secret")))
            .app_data(web::JsonConfig::default().error_handler(|err, req| {
                InternalError::from_response("",
                                             HttpResponse::build(StatusCode::BAD_REQUEST)
                                                 .json(ErrorResponse::bad_request(1000, err.to_string())))
                    .into()
            }))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %t %p %{User-Agent}i"))
            .configure(init)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

pub fn init(cfg: &mut web::ServiceConfig) {
    routes::user::user_handler(cfg);
    routes::auth::auth_handler(cfg);
    routes::index::index_handler(cfg);
}
