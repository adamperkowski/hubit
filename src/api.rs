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
    reqwest::Client::new()
}

fn assign_headers(token: SecretBox<String>) -> HeaderMap {
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

    headers
}

fn match_statuscode(status_code: StatusCode) -> Result<(), StatusCode> {
    match status_code {
        StatusCode::OK => Ok(()),
        _ => {
            eprintln!("{}: {}", REQ_ERROR, status_code);
            Err(status_code)
        }
    }
}

pub async fn create_issue(
    client: reqwest::Client,
    token: SecretBox<String>,
    _args: Vec<&str>,
) -> Result<(), StatusCode> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues",
        _args[0], _args[1]
    );
    let headers = assign_headers(token);
    let body = json!(
        {
            "title":"test issue",
            "body":"opened by Hubit",
            "assignees":[""],
            "milestone":"",
            "labels":["bug"],
        }
    );

    let request = client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await
        .expect(REQ_ERROR);
    let response = request.status();

    match_statuscode(response)
}
