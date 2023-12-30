use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct CreateRoomRequest {
    pub w2g_api_key: String,
    pub share: String,
    pub bg_color: String,
    pub bg_opacity: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoomResponse {
    pub streamkey: String
}
