use serde::{Serialize};
use std::env;

#[derive(Debug, Serialize)]
struct Request {
    w2g_api_key: String,
    share: String,
    bg_color: String,
    bg_opacity: String,
}

fn make_request(video_url: &str) -> Request {
    let api_key: String = env::var("W2G_API_KEY").expect("Expected an api key in the environment");
    Request {
        w2g_api_key: api_key,
        share: String::from(video_url),
        bg_color: String::from("#000000"),
        bg_opacity: String::from("100"),
    }
}

pub async fn create_room(video_url: &str) -> Result<String, reqwest::Error> {
    let url = "https://w2g.tv/rooms/create.json";
    let request = make_request(video_url);
    
    let client = reqwest::Client::new();
    
    let response = client.post(url)
        .json(&request)
        .send()
        .await
        .expect("failed to post request")
        .text()
        .await
        .expect("failed to get text of response");

    let json_response: serde_json::Value = serde_json::from_str(&response).unwrap();
    let streamkey: String = String::from(json_response.get("streamkey").unwrap().as_str().unwrap());
    Ok(streamkey)
}
