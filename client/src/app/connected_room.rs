use std::{sync::Arc};

use crate::{
    Err,
    app::{app_control::AppAction, widget::messages::Messages},
    response::Response,
    websocket_function,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, widgets::Widget};
use tokio::{
    sync::{
        Mutex,
        mpsc::{self, Sender}
    },
    task,
};

#[derive(Debug)]
pub struct Room {
    room_id: String,
    messages: Arc<Mutex<Vec<String>>>,
    // This handles the user input and cursor movement to accurately depict what the user is going to do
    character_indx: usize,
    input_mode: InputMode,
    input: String,
    user_input_sx: Sender<String>,
    closing_room_sx: tokio::sync::watch::Sender<bool>,
}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

impl Room {
    pub fn new(
        room_id: String,
        url: String,
        username: String,
        closing_room_rx: tokio::sync::watch::Receiver<bool>,
        closing_room_sx: tokio::sync::watch::Sender<bool>,
    ) -> Result<Room, Err> {
        let (user_input_sx, user_input_rx) = mpsc::channel(100);
        let (server_message_sx, mut server_message_rx) = mpsc::channel::<Response>(100);
        let url = format!("ws://{url}/ws/joinroom?room_id={room_id}&username={username}");

        // println!("Connecting to {}", url);

        tokio::spawn(async move {
            websocket_function::start_listening(
                url,
                closing_room_rx,
                user_input_rx,
                server_message_sx,
            )
            .await.unwrap();
        });

        let messages = Arc::new(Mutex::new(Vec::new()));
        let clone_messsages = Arc::clone(&messages);
        tokio::spawn(async move {
            while let Some(msg) = server_message_rx.recv().await {
                // Display the derived message here
                let sender = msg.sender.unwrap();
                let message = msg.content.unwrap();
                let mut lock_message = clone_messsages.lock().await;
                lock_message.push(format!("{sender}:{message}"));
                drop(lock_message);
            }
        });

        let room = Room {
            room_id,
            messages,
            character_indx: 0,
            input_mode: InputMode::Normal,
            input: "".to_string(),
            user_input_sx,
            closing_room_sx,
        };

        // if startup_rx.is_empty() {
        //     return Err("Unable to start room".into());
        // }

        Ok(room)
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

    async fn submit_message(&mut self) {
        // I cba deal with th lifetimes clone for now
        self.user_input_sx
            .send(self.input.clone())
            .await
            .unwrap_or_else(|e| println!("Unable to send message because of {e}"));
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
    pub fn render(&self, f: &mut Frame, rect: Rect) {
        let msg = task::block_in_place(|| {
            let guard = tokio::runtime::Handle::current().block_on(self.messages.lock());
            guard.clone()
        });
        let messages = Messages::new(&self.input_mode, &msg, &self.room_id, &self.input);
        messages.render(rect, f.buffer_mut());
        drop(msg);
    }
}

impl Drop for Room {
    fn drop(&mut self) {
        println!("Dropping room");
        // self.closing_room_sx.send(true).unwrap();
    }
}
