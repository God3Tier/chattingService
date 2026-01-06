use std::sync::Arc;

use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    id: Uuid,
    #[serde(with = "arc_string_serde")]
    pub sender: Arc<String>,
    #[serde(with = "arc_string_serde")]
    room_id: Arc<String>,
    content: String,
}

impl Message {
    pub fn new(id: Uuid, sender: Arc<String>, content: String, room_id: Arc<String>) -> Message {
        Message {
            id,
            sender,
            room_id,
            content,
        }
    }
}

mod arc_string_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::sync::Arc;

    pub fn serialize<S>(value: &Arc<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Arc<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Arc::new(s))
    }
}
