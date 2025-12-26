use ratatui::{buffer::Buffer, layout::Rect};

use crate::app::{connected_room::Room, disconnected_room::WaitingRoom};

#[derive(Debug)]
pub enum AppWidget {
    Waiting(WaitingRoom),
    RoomConnected(Room),
    Closed
}


impl AppWidget {
    fn render(self, rect: Rect, buffer: &mut Buffer) {
        match self {
            AppWidget::Waiting(w) => w.render(rect, buffer),
            AppWidget::RoomConnected(w) => w.render(rect, buffer), 
            AppWidget::Closed => return
        }
    }
}