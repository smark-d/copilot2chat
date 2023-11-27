use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;

pub mod util;



#[derive(Serialize, Deserialize)]
struct JsonData {
    stream: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Model {
    id: String,
    object: String,
    created: i64,
    owned_by: String,
    root: String,
    parent: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ModelsResponse {
    object: String,
    data: Vec<Model>,
}

async fn forward_request(gho_token: &str, stream: bool, json_data: &JsonData) -> Result<String, reqwest::Error> {
    // Init reqwest client
    let client = reqwest::Client::new();

    // Prepare headers
    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_static("api.github.com"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", gho_token)).unwrap());
    headers.insert("Editor-Version", HeaderValue::from_static("JetBrains-IU/232.10203.10"));
    headers.insert("Editor-Plugin-Version", HeaderValue::from_static("copilot-intellij/1.3.3.3572"));
    headers.insert(USER_AGENT, HeaderValue::from_static("GithubCopilot/1.129.0"));

    // Get token
    let response: TokenResponse = client
        .get("https://api.github.com/copilot_internal/v2/token")
        .headers(headers.clone())
        .send()
        .await?
        .json()
        .await?;
    let token = response.token;

    // Prepare headers with access token
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
    headers.insert("Editor-Version", HeaderValue::from_static("vscode/1.83.1"));

    // Post request
    let resp = client
        .post("https://api.githubcopilot.com/chat/completions")
        .headers(headers)
        .json(json_data)
        .send()
        .await?
        .text()
        .await?;

    Ok(resp)
}

async fn proxy(json_data: web::Json<JsonData>, gho_token: String) -> impl Responder {
    let response = forward_request(&gho_token, json_data.stream.unwrap_or(false), &json_data).await;
    match response {
        Ok(text) => HttpResponse::Ok().body(text),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn models() -> impl Responder {
    let data = ModelsResponse {
        object: "list".to_string(),
        data: vec![
            Model {
                id: "gpt-4-0314".to_string(),
                object: "model".to_string(),
                created: 1687882410,
                owned_by: "openai".to_string(),
                root: "gpt-4-0314".to_string(),
                parent: None,
            },
            // Add other models here...
        ],
    };
    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/v1/chat/completions", web::post().to(proxy))
            .route("/v1/models", web::get().to(models))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
