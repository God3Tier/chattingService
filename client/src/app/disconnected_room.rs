use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{Frame, buffer::Buffer, layout::Rect, style::Stylize, text::Line, widgets::{Block, Borders, Paragraph, StatefulWidget, Widget}};

use crate::app::{app_control::AppAction, widget::big_rectangle::RectangleInstructions};

#[derive(Debug, PartialEq)]
enum WaitingRoomState {
    Normal,
    LoggingIn
}


#[derive(Debug)]
pub struct WaitingRoom {
    pub buttons: [Rect; 4],
    pub waiting_room_state: WaitingRoomState
}

impl WaitingRoom {
    pub fn new() -> WaitingRoom {
        WaitingRoom {
            buttons: [Rect::new(0, 0 ,0, 0); 4],
            waiting_room_state: WaitingRoomState::Normal
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent) -> AppAction {
        match key.code {
            KeyCode::Char('r') => {
                self.waiting_room_state = WaitingRoomState::LoggingIn;
                AppAction::None
            }
            KeyCode::Char('1') =>  AppAction::GoToRoom("Room1".to_string()),
            KeyCode::Char('2') =>  AppAction::GoToRoom("Room2".to_string()),
            KeyCode::Char('3') =>  AppAction::GoToRoom("Room3".to_string()),
            KeyCode::Char('4') =>  AppAction::GoToRoom("Room4".to_string()),
            KeyCode::Char('q') =>  AppAction::Quit,
            _ =>  AppAction::None
        }
    }
    
    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let rectangle_instruction = RectangleInstructions::default();
        rectangle_instruction.render(area, f.buffer_mut());
    }
}
