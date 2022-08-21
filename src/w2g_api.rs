use serde::Serialize;
use std::env;

#[derive(Debug, Serialize)]
struct CreateRoomRequest {
    w2g_api_key: String,
    share: String,
    bg_color: String,
    bg_opacity: String,
}
#[derive(Debug, Serialize)]
struct UpdateRoomRequest {
    w2g_api_key: String,
    item_url: String,
}

pub struct W2GRoom {
    _streamkey: String,
    _client: reqwest::Client,
}

impl W2GRoom {
    pub fn new() -> Self {
        W2GRoom {
            _streamkey: String::from(""),
            _client: reqwest::Client::new(),
        }
    }

    pub async fn create_room(&mut self, video_url: &str) -> Result<(), reqwest::Error> {
        let url = "https://w2g.tv/rooms/create.json";
        let request = make_request(video_url);

        let response = self
            ._client
            .post(url)
            .json(&request)
            .send()
            .await
            .expect("failed to post request")
            .text()
            .await
            .expect("failed to get text of response");

        let json_response: serde_json::Value = serde_json::from_str(&response).unwrap();
        let streamkey: String =
            String::from(json_response.get("streamkey").unwrap().as_str().unwrap());
        self._streamkey = streamkey;
        Ok(())
    }

    pub fn get_room_url(&self) -> String {
        format!("https://w2g.tv/rooms/{}", self._streamkey)
    }
}

fn make_request(video_url: &str) -> CreateRoomRequest {
    let api_key: String = env::var("W2G_API_KEY").expect("Expected an api key in the environment");
    CreateRoomRequest {
        w2g_api_key: api_key,
        share: String::from(video_url),
        bg_color: String::from("#000000"),
        bg_opacity: String::from("100"),
    }
}

pub async fn make_room(video_url: &str) -> Result<W2GRoom, reqwest::Error> {
    let mut room: W2GRoom = W2GRoom::new();
    room.create_room(video_url).await?;
    Ok(room)
}
