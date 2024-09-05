use std::net::SocketAddr;
use std::time::Duration;
use crate::conf::Configuration;
use axum::{extract::{Json, State, path::Path, Query}, Router, routing::{post, get}, response::IntoResponse};
use axum::response::Html;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use crate::handlers::test_db;
use crate::service::AppContext;

pub async fn start_serve(cfg: &Configuration) {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&cfg.db_uri.clone())
        .await
        .expect("can't connect to database");
    let client = redis::Client::open(cfg.redis_uri.clone()).unwrap();

    let state = AppContext{ db: pool, redis: client };

    let app = Router::new()
        .route("/status", get(status))
        .route("/404", get(not_found))
        .route("/test/test_tbs", get(test_db))
        .with_state(state);

    // 启动HTTP服务器
    println!("server will start at 0.0.0.0:{}", cfg.port);
    let serv = axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], cfg.port)))
        .serve(app.into_make_service())
        .await;
    match serv {
        Ok(_)=>{
            println!("server stopped normally");
        }
        Err(err)=>{
            println!("server fault with err:{}", err);
        }
    }
}

async fn not_found() -> Html<String> {
    Html(include_str!("../templates/404.html").to_string())
}
async fn status()->impl IntoResponse {
    Json(json!({"status": "it works!"}))
}