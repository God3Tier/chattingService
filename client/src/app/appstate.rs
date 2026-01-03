use ratatui::{Frame, layout::Rect, prelude::Widget};

use crate::app::{connected_room::Room, disconnected_room::WaitingRoom};

#[derive(Debug)]
pub enum AppWidget<'a> {
    Waiting(&'a mut WaitingRoom),
    RoomConnected(&'a mut Room),
    Closed,
    None,
}

impl<'a> AppWidget<'a> {
    pub fn render(self, f: &mut Frame, rect: Rect) {
        match self {
            // TODO: Will handle proper state change later
            AppWidget::Waiting(w) => w.render(f, rect),
            AppWidget::RoomConnected(w) => {
                w.render(f, rect);
            }
            _ => return,
        }
    }
}
