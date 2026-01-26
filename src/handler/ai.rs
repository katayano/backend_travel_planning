use axum::extract::Query;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize)]
pub struct ChatQuery {
    pub question: String,
}

#[derive(Serialize)]
pub struct GenerateContentRequest {
    contents: Vec<RequestContent>,
}

#[derive(Serialize)]
pub struct RequestContent {
    role: String,
    parts: Vec<RequestContentPart>,
}

#[derive(Serialize)]
pub struct RequestContentPart {
    text: String,
}

#[derive(Deserialize)]
pub struct GenerateContentResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
pub struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
pub struct ResponseContent {
    role: String,
    parts: Vec<ResponseContentPart>,
}

#[derive(Deserialize)]
pub struct ResponseContentPart {
    text: String,
}

pub async fn chat(query: Query<ChatQuery>) -> String {
    let question = &query.question;

    let url = get_url();

    let request_body = GenerateContentRequest {
        contents: vec![RequestContent {
            role: "user".to_string(),
            parts: vec![RequestContentPart {
                text: question.to_string(),
            }],
        }],
    };

    let client = reqwest::Client::new();

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .unwrap();

    let response_body = response.text().await.unwrap();

    // レスポンスを整形
    let response_json: Vec<GenerateContentResponse> =
        serde_json::from_str(&response_body).expect("Failed to parse response JSON");

    let answer = response_json
        .iter()
        .flat_map(|res| &res.candidates)
        .flat_map(|candidate| &candidate.content.parts)
        .map(|part| part.text.clone())
        .collect::<Vec<String>>()
        .join("");

    answer
}

fn get_url() -> String {
    dotenv().ok();
    let url = env::var("VERTEX_API_ENDPOINT").expect("VERTEX_API_ENDPOINT must be set");
    let key = env::var("VERTEX_API_KEY").expect("VERTEX_API_KEY must be set");

    format!("{}&key={}", url, key)
}
