use reqwest::header::HeaderMap;

use crate::TokenResponse;


pub async fn get_header(headers: &HeaderMap) -> String {

     let client = reqwest::Client::new();
        // Get token
        let response: TokenResponse = client
        .get("https://api.github.com/copilot_internal/v2/token")
        .headers(headers.clone())
        .send()
        .await.unwrap()
        .json()
        .await.unwrap();
    let token = response.token;
    token
}