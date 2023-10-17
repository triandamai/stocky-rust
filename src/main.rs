use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::Database;

use crate::repositories::user::UserRepository;

mod common;
mod models;
mod repositories;
mod routes;

async fn hi() -> impl Responder {
    HttpResponse::Ok().body("Hi there")
}

#[derive(Debug, Clone)]
pub struct AppState {
    user_repository: UserRepository,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = match std::env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => String::from("postgres://triandamai:triandamai@localhost:5432/actix"),
    };

    let db = Database::connect(url).await.unwrap();

    let user_repository = UserRepository {
        db_conn: db.clone(),
    };

    let state = AppState { user_repository };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::index::index_handler())
        .service(routes::user::user_handler())
        .route("/hi", web::get().to(hi));
}
