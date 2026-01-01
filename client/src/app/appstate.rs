use ratatui::{
    buffer::Buffer, 
    layout::Rect, 
    prelude::{
        Widget, 
    }
};

use crate::app::{
    connected_room::Room, 
    disconnected_room::WaitingRoom
};

#[derive(Debug)]
pub enum AppWidget<'a> {
    Waiting(&'a mut WaitingRoom),
    RoomConnected(&'a mut Room),
    Closed
}


impl<'a> AppWidget<'a> {
    pub fn render(self, rect: Rect, buffer: &mut Buffer) {
        match self {
            // TODO: Will handle proper state change later 
            AppWidget::Waiting(w) => w.render(rect, buffer),
            AppWidget::RoomConnected(w) => w.render(rect, buffer), 
            AppWidget::Closed => return
        }
    }
}