use axum::response::IntoResponse;
use axum::extract::{State, Json};
use serde_json::json;
use crate::service::{AppContext, TestService, Service};
use serde::{Deserialize, Serialize};
use crate::response::{APIResult, AppError};

#[derive(Serialize, Deserialize)]
pub struct OkResponse{

}

fn check_auth()->Result<String, AppError> {
    Err(AppError::AuthError)
}

pub async fn test_db(State(ctx): State<AppContext>) -> APIResult<Json<OkResponse>>{
    check_auth()?;
    let serv = ctx.get_service::<TestService>();
    let tbs = serv.get_test_tb().await?;
    Ok(Json(OkResponse{}))
}