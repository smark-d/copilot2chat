use actix_web::{web, App, HttpResponseBuilder, HttpServer, Responder};
use reqwest::StatusCode;
use std::io::Error;

pub mod util;
pub mod uuid;

use once_cell::sync::Lazy;
use reqwest::Client;

static CLIENT: Lazy<Client> = Lazy::new(Client::new); // lazy static client instance

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/v1/chat/completions", web::post().to(forward)))
        .bind(("0.0.0.0", 2088))?
        .run()
        .await
}

async fn forward(body: web::Bytes) -> Result<impl Responder, Box<dyn std::error::Error>> {
    log!("forwarding request to copilot， the request body is: {:?}", body);
    let url = "https://api.githubcopilot.com/chat/completions";

    let headers = util::get_headers().await;
    let res = CLIENT
        .post(url)
        .headers(headers)
        .body(body.to_vec())
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        Ok(HttpResponseBuilder::new(StatusCode::OK)
            .content_type("text/event-stream; charset=utf-8")
            .streaming(res.bytes_stream()))
    } else {
        Err(Box::new(Error::new(
            std::io::ErrorKind::Other,
            "request to copilot failed",
        )))
    }
}
