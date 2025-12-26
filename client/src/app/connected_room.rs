use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Debug)]
pub struct Room {
    roomname: String, 
    messages: Vec<String>
}

impl Room {
    pub fn new() -> Room {
        Room {
            roomname: "placeholder".to_string(),
            messages: Vec::new()
        }
    }
}

impl Widget for &Room {
    fn render(self, area: Rect, buf: &mut Buffer) {
        
    }
}