use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::Line, widgets::{Block, Borders, Paragraph, StatefulWidget, Widget}};

use crate::app::app_control::AppAction;

#[derive(Debug)]
pub struct WaitingRoom {
    pub buttons: [Rect; 4],
}

impl WaitingRoom {
    pub fn new() -> WaitingRoom {
        WaitingRoom {
            buttons: [Rect::new(0, 0 ,0, 0); 4],
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent) -> AppAction {
        match key.code {
            KeyCode::Char('1') =>  AppAction::GoToRoom("Room 1".to_string()),
            KeyCode::Char('2') =>  AppAction::GoToRoom("Room 2".to_string()),
            KeyCode::Char('3') =>  AppAction::GoToRoom("Room 3".to_string()),
            KeyCode::Char('4') =>  AppAction::GoToRoom("Room 4".to_string()),
            KeyCode::Char('q') =>  AppAction::Quit,
            _ =>  AppAction::None
        }
    }
}

impl Widget for &WaitingRoom {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Welcome to data leaker room messaging service").bold();
        let instructions = Line::from(
            vec![
                "Select a room to join".into(),
                "Room 1 : 1".black().bold(),
                "Room 2 : 2".blue().bold(),
                "Room 3 : 3".red().bold(),
                "Room 4 : 4".black().bold(),
                "Custom room : Coming soon".into()
            ]
        );

        let paragraph = Paragraph::new(title).block(
            Block::default().title(instructions)
                .borders(Borders::ALL)
        );


        paragraph.render(area, buf);
        
        
    }
}
