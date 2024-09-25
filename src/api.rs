use std::any::Any;

use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
    StatusCode,
};
use secrecy::{ExposeSecret, SecretBox};
use serde_json::json;

const ACCEPT_VND: &str = "application/vnd.github+json";
const HUBIT_USER_AGENT: &str = "Hubit CLI";
const GITHUB_API_NAME: &str = "X-GitHub-Api-Version";
const GITHUB_API_VERSION: &str = "2022-11-28";
const HUBIT_AUTHORIZATION: &str = "Bearer";
const REQ_ERROR: &str = "Request failed";

pub fn init() -> reqwest::Client {
    let client = reqwest::Client::new();
    client
}

/* pub async fn list_assigned_issues(
    client: reqwest::Client,
    token: SecretBox<String>,
) -> Result<String, StatusCode> {
    let url = "https://api.github.com/issues";
    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT, HeaderValue::from_static(ACCEPT_VND));
    headers.insert(USER_AGENT, HeaderValue::from_static(HUBIT_USER_AGENT));
    headers.insert(
        GITHUB_API_NAME,
        HeaderValue::from_static(GITHUB_API_VERSION),
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(
            format!("{} {}", HUBIT_AUTHORIZATION, token.expose_secret()).as_str(),
        )
        .unwrap(),
    );

    let request = client.get(url).headers(headers).send().await.expect(REQ_ERROR);
    let response = request.status();

    match response {
        StatusCode::OK => {
            Ok(request.text().await.unwrap())
        }
        _ => {
            eprintln!("{}: {}", REQ_ERROR, response);
            Err(response)
        }
    }
} */
