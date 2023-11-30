use super::uuid;
use actix_web::http::header;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use once_cell::sync::Lazy;
use std::time::{Duration, Instant, SystemTime};

struct CachedToken {
    token: String,
    fetched_at: SystemTime,
}

static mut TOKEN: Lazy<Option<CachedToken>> = Lazy::new(|| None);

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

pub async fn get_headers() -> HeaderMap {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", get_token().await).parse().unwrap(),
    );
    headers.insert("X-Request-Id", uuid::get_request_id().parse().unwrap());
    headers.insert("Vscode-Sessionid", uuid::get_session_id().parse().unwrap());
    headers.insert("vscode-machineid", uuid::get_machine_id().parse().unwrap());
    headers.insert("Editor-Version", "vscode/1.84.2".parse().unwrap());
    headers.insert(
        "Editor-Plugin-Version",
        "copilot-chat/0.10.2".parse().unwrap(),
    );
    headers.insert("Openai-Organization", "github-copilot".parse().unwrap());
    headers.insert("Openai-Intent", "conversation-panel".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("User-Agent", "GitHubCopilotChat/0.10.2".parse().unwrap());
    headers.insert("Accept", "*/*".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
    headers
}

async fn get_token() -> String {
    // Check if token is valid and was fetched less than 15 minutes ago
    if let Some(ref cached_token) = unsafe {TOKEN.as_ref()} {
        if cached_token.fetched_at.elapsed().unwrap() < Duration::from_secs(15 * 60) {
            return cached_token.token.clone();
        }
    }

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
    let res: TokenResponse = client
        .get("https://api.github.com/copilot_internal/v2/token")
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    // Update the cached token
    unsafe {
        *TOKEN = Some(CachedToken {
            token: res.token.clone(),
            fetched_at: SystemTime::now(),
        });
    }

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
        assert_eq!(
            headers.get(header::CONTENT_TYPE).unwrap(),
            "application/json"
        );
        assert_eq!(headers.get("Editor-Version").unwrap(), "vscode/1.84.1");
        assert!(headers.get("Authorization").is_some());
        log!("{:?}", headers.get("Authorization").unwrap());
    }

    #[actix_web::test]
    async fn test_get_token() {
        std::env::set_var("GHU_TOKEN", "ghu_xxx");
        let token = get_token().await;
        println!("token: {}", token);
        assert_eq!(token, get_token().await);
    }
}
