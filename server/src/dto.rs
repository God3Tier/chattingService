use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct RoomInfoDTO {
    pub room_id: String,
    pub username: String
}