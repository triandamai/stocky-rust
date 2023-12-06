use std::string::ToString;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::Result as WebResult;
use actix_web::web::{Data, Json};
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use dotenv::dotenv;
use redis::Client;
use sea_orm::{Database, DatabaseConnection};

use crate::common::response::{BaseResponse, ErrorResponse};
use crate::server_sent_event::sse_emitter::SseBroadcaster;

mod common;
// mod entity;
mod entity;
mod models;
mod repositories;
mod request_filter;
mod routes;
mod server_sent_event;

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
    cache: Client,
    sse_emitter: Arc<SseBroadcaster>,
}

const DB_URL_KEY: &str = "DATABASE_URL";
const REDIS_URL_KEY: &str = "REDIS_URL";
const DB_URL_DEFAULT_VALUE: &str = "postgres://user:password@host:port/db";
const REDIS_URL_DEFAULT_VALUE: &str = "redis://user:password@host:port";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    std::env::set_var("RUST_BACKTRACE", "full".to_string());
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }
    let postgres_url: String = std::env::var(DB_URL_KEY)
        .unwrap_or(DB_URL_DEFAULT_VALUE.to_string());

    let redis_url: String = std::env::var(REDIS_URL_KEY)
        .unwrap_or(REDIS_URL_DEFAULT_VALUE.to_string());

    let db: DatabaseConnection = Database::connect(postgres_url)
        .await
        .expect("failed to connect postgres");

    let cache: Client = Client::open(redis_url)
        .expect("Invalid connection Url");

    let sse_emitter = SseBroadcaster::create();

    let state: AppState = AppState {
        db: db.clone(),
        cache: cache.clone(),
        sse_emitter: sse_emitter.clone(),
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                // let origin = origin.as_bytes();
                // origin.ends_with(b".bluhabit.id")
                true
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE);

        App::new()
            .wrap(cors)
            // .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new(
                "%a %r %s %b %{Referer}i %{User-Agent}i %T",
            ))
            .app_data(Data::new(state.clone()))
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                InternalError::from_response(
                    format!("cause {}", err.to_string()),
                    HttpResponse::build(StatusCode::BAD_REQUEST)
                        .json(
                            ErrorResponse::bad_request(
                                1000,
                                err.to_string(),
                            )
                        ),
                ).into()
            }))
            .configure(init)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

async fn event_stream(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let key = req.match_info().get("key")
        .unwrap();
    state.sse_emitter.new_client(key).await
}

async fn send(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> WebResult<impl Responder> {
    let key = req.match_info().get("key")
        .unwrap();
    state.sse_emitter.send_to("databaru",key.to_string(), "Tes").await;
    return Ok(Json(BaseResponse::success(
        200,
        Some(""),
        "".to_string(),
    )));
}

async fn broadcast(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> WebResult<impl Responder> {
    state.sse_emitter.broadcast( "topic baru","Tes").await;
    return Ok(Json(BaseResponse::success(
        200,
        Some(""),
        "".to_string(),
    )));
}


pub fn init(cfg: &mut web::ServiceConfig) {
    routes::user::user_handler(cfg);
    routes::auth::auth_handler(cfg);
    routes::index::index_handler(cfg);
    cfg.service(
        web::scope("/sse")
            .route("/events/{key}", web::get().to(event_stream))
            .route("/send/{key}", web::get().to(send))
            .route("/broadcast", web::get().to(broadcast))
    );
}
