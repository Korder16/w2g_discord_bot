#[path = "w2g_api_models.rs"]
mod w2g_api_models;

use w2g_api_models::{CreateRoomRequest, CreateRoomResponse};
use std::env;

fn make_request(video_url: &str) -> CreateRoomRequest {
    let api_key: String = env::var("W2G_API_KEY").expect("Expected an api key in the environment");
    CreateRoomRequest {
        w2g_api_key: api_key,
        share: String::from(video_url),
        bg_color: String::from("#000000"),
        bg_opacity: String::from("100"),
    }
}

pub async fn create_room(video_url: &str) -> Result<CreateRoomResponse, reqwest::Error> {
    let url = "https://api.w2g.tv/rooms/create.json";
    let request = make_request(video_url);

    let client = reqwest::Client::new();
    let response: CreateRoomResponse = client
        .post(url)
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
