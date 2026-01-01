use crate::{Err, app::app_control::AppAction, response::Response, websocket_function};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug)]
pub struct Room {
    room_id: String,
    messages: Vec<String>,
    // This handles the user input and cursor movement to accurately depict what the user is going to do
    character_indx: usize,
    input_mode: InputMode,
    input: String,
    user_input_sx: Sender<String>,
    server_message_rx: Receiver<Response>,
    pub app_action: AppAction
}

#[derive(Debug)]
enum InputMode {
    Normal,
    Editing,
}

impl Room {
    pub async fn new(room_id: String, url: String) -> Result<Room, Err> {
        let (user_input_sx, user_input_rx) = mpsc::channel(100);
        let (server_message_sx, server_message_rx) = mpsc::channel::<Response>(100);
        // TODO: Fix this later
        websocket_function::start_listening(url, room_id.clone(), user_input_rx, server_message_sx)
            .await;
        
        Ok(Room {
            room_id,
            messages: Vec::new(),
            character_indx: 0,
            input_mode: InputMode::Normal,
            input: "".to_string(),
            user_input_sx,
            server_message_rx,
            app_action: AppAction::None
        })
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_indx.saturating_sub(1);
        self.character_indx = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_left = self.character_indx.saturating_add(1);
        self.character_indx = self.clamp_cursor(cursor_moved_left);
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

    pub async fn read_message(&mut self) {
        while let Some(msg) = self.server_message_rx.recv().await {
            // Display the derived message here
            let sender = msg.sender.unwrap();
            let message = msg.content.unwrap();
            self.messages.push(format!("{sender}:{message}"));
        }
    }

    async fn submit_message(&mut self) {
        // I cba deal with th lifetimes clone for now
        self.user_input_sx.send(self.input.clone())
            .await.unwrap_or_else(|e| println!("Unable to send message because of {e}"));
        self.input.clear();
        self.reset_cursor();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_indx)
            .unwrap_or(self.input.len())
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub async fn handle_keys(&mut self, key: KeyEvent) -> AppAction {
        // Careful with this one if it is unable to re-read the terminal
            match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => self.input_mode = InputMode::Editing,
                    KeyCode::Char('q') => return AppAction::GoToWaitingRoom,
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Char(new_char) => {
                        self.enter_char(new_char);
                    }
                    KeyCode::Enter => self.submit_message().await,
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Esc => self.input_mode = InputMode::Normal,
                    _ => {}
                },
            }

        AppAction::None
    }
}

impl Widget for &Room {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical({
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
        });
        
        let [help_area, input_area, message_area] = vertical.areas(area);
        
        Paragraph::new(match self.input_mode {
            InputMode::Editing => "Press escape to return to normal mode",
            InputMode::Normal => "Press e to edit. Press q to join a different room"
        }).render(help_area, buf);
        
        Paragraph::new(self.messages.join("\n")).block(
            Block::default().borders(Borders::ALL)
                .title(self.room_id.as_str())
        ).wrap(ratatui::widgets::Wrap { trim: false })
        .render(message_area, buf);
        
        Paragraph::new(self.input.as_str())
            .block(Block::default().borders(Borders::ALL).title("Input")).render(input_area, buf);
    }
}
