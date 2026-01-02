use serde::{Deserialize};
use tokio_tungstenite::tungstenite::Bytes;

use crate::Err;

#[derive(Deserialize)]
pub struct Response {
    pub sender: Option<String>,
    pub content: Option<String>
}
impl Response {
    pub fn new(json_bytes: Bytes) -> Response {
        let json_string = json_bytes.trim_ascii();
        match serde_json::from_slice::<Response>(json_string) {
            Ok(res) => {
                return res;
            }, 
            Err(e) => {
                println!("Unable to Read message");
                Response { sender: None, content: None }
            }
        }
    }
}