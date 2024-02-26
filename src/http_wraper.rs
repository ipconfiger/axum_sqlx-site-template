use reqwest::{Client, RequestBuilder};
use serde::Deserialize;
use serde_json::from_str;
use crate::response::AppError;

pub async fn api_impl<T:for<'a> Deserialize<'a>, F:Fn(RequestBuilder)->RequestBuilder>(url: &str, is_post: bool, f: F) -> Result<T, AppError>{
    let client = Client::new();
    let request = f(if is_post {client.post(url)} else {client.get(url)});
    let response = request.send().await?;
    let resp_txt = response.text().await?;
    match from_str::<T>(resp_txt.as_str()) {
        Ok(r)=>{Ok(r)},
        Err(e)=>Err(AppError::JsonError(e))
    }
}