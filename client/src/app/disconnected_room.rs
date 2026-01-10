use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::app::{
    app_control::AppAction,
    widget::{display_text_box::DisplayTextInput, filling_buttons::FillingButtons},
};

#[derive(Debug, PartialEq)]
pub enum WaitingRoomState {
    Normal,
    LoggingIn,
}

#[derive(Debug)]
pub struct WaitingRoom {
    pub waiting_room_state: WaitingRoomState,
    input: String,
    character_indx: usize,
    username: String,
}

impl WaitingRoom {
    pub fn new() -> WaitingRoom {
        WaitingRoom {
            waiting_room_state: WaitingRoomState::Normal,
            username: "Guest".to_string(),
            input: "".to_string(),
            character_indx: 0,
        }
    }

    // I really dont have a better way to not have this as a cloned value since having it as a shared reference
    // forces this and room lifetimes to be frankenstined together
    pub fn handle_keys(&mut self, key: KeyEvent) -> AppAction {
        match self.waiting_room_state {
            WaitingRoomState::Normal => match key.code {
                KeyCode::Char('r') => {
                    self.input.clear();
                    self.waiting_room_state = WaitingRoomState::LoggingIn;
                    AppAction::None
                }
                KeyCode::Char('1') => {
                    AppAction::GoToRoom("Room1".to_string(), self.username.clone())
                }
                KeyCode::Char('2') => {
                    AppAction::GoToRoom("Room2".to_string(), self.username.clone())
                }
                KeyCode::Char('3') => {
                    AppAction::GoToRoom("Room3".to_string(), self.username.clone())
                }
                KeyCode::Char('4') => {
                    AppAction::GoToRoom("Room4".to_string(), self.username.clone())
                }
                KeyCode::Char('q') => AppAction::Quit,
                _ => AppAction::None,
            },
            WaitingRoomState::LoggingIn => {
                match key.code {
                    KeyCode::Char(new_char) => {
                        self.enter_char(new_char);
                    }
                    KeyCode::Enter => {
                        self.waiting_room_state = WaitingRoomState::Normal;
                        self.submit_message()
                    },
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Esc => {
                        self.input.clear();
                        self.reset_cursor();
                        self.waiting_room_state = WaitingRoomState::Normal
                    },
                    _ => {}
                }
                AppAction::None
            }
        }
    }

    // I dont know how to not just abstract this elsewhere. No point making it a trait for now cause
    // that will add more unnecessary complication
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_indx.saturating_sub(1);
        self.character_indx = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_left = self.character_indx.saturating_add(1);
        self.character_indx = self.clamp_cursor(cursor_moved_left);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_indx)
            .unwrap_or(self.input.len())
    }

    fn enter_char(&mut self, new_char: char) {
        let indx = self.byte_index();
        self.input.insert(indx, new_char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        if self.character_indx != 0 {
            let current_indx = self.character_indx;
            let new_pos = current_indx - 1;

            let new_word = self.input.chars().take(new_pos);
            let after_char_to_delete = self.input.chars().skip(current_indx);

            self.input = new_word.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn reset_cursor(&mut self) {
        self.character_indx = 0;
    }

    fn submit_message(&mut self) {
        self.username = self.input.clone();
        self.input.clear();
        self.reset_cursor();
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        if self.waiting_room_state == WaitingRoomState::LoggingIn {
            let layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([Constraint::Length(6), Constraint::Min(1)]) // Title takes 3 lines, buttons the rest
                .split(area);
            
            let instruction_box = DisplayTextInput::new("Please enter your username and password (coming soon!)");
            instruction_box.render(layout[0], f.buffer_mut());
            
            let username_text_box = DisplayTextInput::new(&self.input);
            username_text_box.render(layout[1], f.buffer_mut());
            return;
        }

        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)]) // Title takes 3 lines, buttons the rest
            .split(area);

        let title_area = layout[0];
        let buttons_area = layout[1];

        let rectangle_instruction =
            DisplayTextInput::new("Welcome to data leak chatbot\nPlease pick a function to do");
        rectangle_instruction.render(title_area, f.buffer_mut());

        let instructions: Vec<String> = vec![
            "Room 1 : 1".into(),
            "Room 2 : 2".into(),
            "Room 3 : 3".into(),
            "Room 4 : 4".into(),
            "Select r to sign in".into(),
            "Custom room : Coming soon".into(),
        ];

        let num_rows = 3;
        let num_cols = 2;

        let filling_buttons = FillingButtons::new(instructions, num_rows, num_cols);
        filling_buttons.render(buttons_area, f.buffer_mut());
    }
}
