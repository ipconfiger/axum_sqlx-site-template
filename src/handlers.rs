use axum::response::IntoResponse;
use axum::extract::{State, Json};
use serde_json::json;
use crate::service::{AppContext, TestService, Service};

pub async fn test_db(State(ctx): State<AppContext>) ->impl IntoResponse{
    let serv = ctx.get_service::<TestService>();
    let tbs = serv.get_test_tb().await;
    let mps:Vec<_> = tbs.iter().map(|mp| json!({"id": mp.id, "name": mp.name, "ts": mp.ts})).collect();
    Json(json!({"status": true, "tbs": mps}))
}