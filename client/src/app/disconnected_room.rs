use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::app::{app_control::AppAction, widget::big_rectangle::RectangleInstructions};

#[derive(Debug, PartialEq)]
enum WaitingRoomState {
    Normal,
    LoggingIn,
}

#[derive(Debug)]
pub struct WaitingRoom {
    waiting_room_state: WaitingRoomState,
}

impl WaitingRoom {
    pub fn new() -> WaitingRoom {
        WaitingRoom {
            waiting_room_state: WaitingRoomState::Normal,
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent) -> AppAction {
        match key.code {
            KeyCode::Char('r') => {
                self.waiting_room_state = WaitingRoomState::LoggingIn;
                AppAction::None
            }
            KeyCode::Char('1') => AppAction::GoToRoom("Room1".to_string()),
            KeyCode::Char('2') => AppAction::GoToRoom("Room2".to_string()),
            KeyCode::Char('3') => AppAction::GoToRoom("Room3".to_string()),
            KeyCode::Char('4') => AppAction::GoToRoom("Room4".to_string()),
            KeyCode::Char('q') => AppAction::Quit,
            _ => AppAction::None,
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)]) // Title takes 3 lines, buttons the rest
            .split(area);

        let title_area = layout[0];
        let buttons_area = layout[1];
        let rectangle_instruction = RectangleInstructions::default();
        rectangle_instruction.render(title_area, f.buffer_mut());

        let instructions = [
            "Room 1 : 1",
            "Room 2 : 2",
            "Room 3 : 3",
            "Room 4 : 4",
            "Select r to sign in",
            "Custom room : Coming soon",
        ];

        let num_rows = 3;
        let num_cols = 2;
        let row_constraints: Vec<Constraint> =
            (0..num_rows).map(|_| Constraint::Length(3)).collect(); // Each row 3 lines tall
        let row_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(row_constraints)
            .split(buttons_area);

        // Render buttons in the grid
        for i in 0..instructions.len() {
            let row = i / num_cols;
            let col = i % num_cols;

            // Split the current row into 2 columns
            let col_constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
            let col_layout = Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints(col_constraints)
                .split(row_layout[row]);

            // Render the button in the appropriate cell
            let button_block = Block::default().borders(Borders::ALL);
            let button_paragraph = Paragraph::new(instructions[i]).block(button_block);
            button_paragraph.render(col_layout[col], f.buffer_mut());
        }
    }
}
