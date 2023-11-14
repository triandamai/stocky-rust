use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use dotenv::dotenv;
use redis::{Client, Commands};
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
    db: DatabaseConnection,
    cache: Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_BACKTRACE", "full".to_string());
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }
    let postgres_url = match std::env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => String::from("postgres://admin:password123@localhost:5432/actix"),
    };
    let jwt_secret = match std::env::var("JWT_SECRET") {
        Ok(val) => val,
        Err(_) => String::from("thisisverysecret"),
    };
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(val) => val,
        Err(_) => String::from("redis//:eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81@localhost:6379")
    };
    let db = Database::connect(postgres_url).await
        .expect("failed to connect postgres");
    let cache = Client::open(redis_url)
        .expect("Invalid connection Url");

    let state = AppState { db: db.clone(), cache:cache.clone() };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                InternalError::from_response(format!("cause {}",err.to_string()), HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(ErrorResponse::bad_request(1000, err.to_string())))
                    .into()
            }))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
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
