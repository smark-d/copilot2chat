
use std::time::{Instant, Duration};
use actix_web::{web::head, http::header};
use reqwest::header::HeaderMap;
use serde::{Serialize, Deserialize};

static mut HEADER_CACHE: Option<(Instant, HeaderMap)> = None;

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

pub async fn get_headers() -> HeaderMap {
    unsafe {
        if let Some((time, headers)) = &HEADER_CACHE {
            if time.elapsed() < Duration::from_secs(20 * 60) {
                return headers.clone();
            }
        }
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Authorization", format!("Bearer {}", get_token().await).parse().unwrap());
        headers.insert("Editor-Version", "vscode/1.84.1".parse().unwrap());

        HEADER_CACHE = Some((Instant::now(), headers.clone()));

        headers
    }
}

async fn get_token() -> String {
    // get gho_token from env
    let ghu_token = std::env::var("GHU_TOKEN").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Host", "api.github.com".parse().unwrap());
    headers.insert("authorization", format!("token {}", ghu_token).parse().unwrap());
    headers.insert("editor-version", "JetBrains-IU/232.10203.10".parse().unwrap());
    headers.insert("editor-plugin-version", "copilot-intellij/1.3.3.3572".parse().unwrap());
    headers.insert("user-agent", "GithubCopilot/1.129.0".parse().unwrap());
    headers.insert("accept", "*/*".parse().unwrap());

    let client = reqwest::Client::new();
    let res: TokenResponse = client.get("https://api.github.com/copilot_internal/v2/token")
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json().await.unwrap();
    res.token
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        let now = std::time::SystemTime::now();
        let datetime: chrono::DateTime<chrono::Local> = chrono::DateTime::from(now);
        let time_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("[{}:{}:{} - {}] {} - {}", file!(), line!(), column!(), time_str, module_path!(), format_args!($($arg)*));
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_get_headers() {
        std::env::set_var("GHU_TOKEN", "ghu_xxx");
        let headers = get_headers().await;
        assert_eq!(headers.get(header::CONTENT_TYPE).unwrap(), "application/json");
        assert_eq!(headers.get("Editor-Version").unwrap(), "vscode/1.84.1");
        assert!(headers.get("Authorization").is_some());
        log!("{:?}", headers.get("Authorization").unwrap());
    }
}