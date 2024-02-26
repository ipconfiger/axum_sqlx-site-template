use serde::{Deserialize, Serialize};
use crate::http_wraper::api_impl;
use crate::response::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub openid: String,
    pub session_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub rid: String,
    pub errcode: u16,
    pub errmsg: String
}

pub async fn code_2_token(app_id: &str, secret: &str, code: &str) -> Result<TokenResponse, AppError> {
    let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={app_id}&secret={secret}&js_code={code}&grant_type=authorization_code");
    api_impl(url.as_str(), false, |req| req).await
}
