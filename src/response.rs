use axum::Json;
use axum::response::IntoResponse;
use sqlx::Error;
use serde_json::json;
use redis::RedisError;
use reqwest::Error as HttpError;
use log::error;
use serde_json::Error as JsonError;


#[derive(Debug)]
pub enum AppError {
    DbError(Error),
    RedisError(RedisError),
    LogicError(String),
    AuthError,
    ForbiddenError,
    HttpError(HttpError),
    JsonError(JsonError)
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        Self::DbError(value)
    }
}

impl From<String> for AppError {
    fn from(value: String) -> Self {
        Self::LogicError(value)
    }
}

impl From<RedisError> for AppError {
    fn from(value: RedisError) -> Self {
        Self::RedisError(value)
    }
}

impl From<HttpError> for AppError {
    fn from(value: HttpError) -> Self {
        Self::HttpError(value)
    }
}

impl From<JsonError> for AppError {
    fn from(value: JsonError) -> Self {
        Self::JsonError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DbError(de)=>{
                error!("数据库错误:{:?}", de);
                Json(json!({"code":500, "detail": format!("{de:?}")})).into_response()
            },
            Self::LogicError(msg)=>{
                error!("业务错误:{}", msg);
                 Json(json!({"code":500, "detail": format!("{msg}")})).into_response()
            },
            Self::AuthError=>{
                Json(json!({"code":401, "detail": "错误的用户名或者密码"})).into_response()
            },
            Self::ForbiddenError=>{
                Json(json!({"code":403, "detail": "没有访问权限"})).into_response()
            },
            Self::RedisError(re)=>{
                error!("Redis错误:{:?}", re);
                Json(json!({"code":500, "detail": format!("{re:?}")})).into_response()
            },
            Self::HttpError(he)=>{
                error!("HTTP请求错误:{:?}", he);
                Json(json!({"code":500, "detail": format!("{he:?}")})).into_response()
            },
            Self::JsonError(je)=>{
                error!("Json反序列化错误:{:?}", je);
                Json(json!({"code":500, "detail": format!("{je:?}")})).into_response()
            }
        }

    }
}

pub type APIResult<T> = Result<T, AppError>;
