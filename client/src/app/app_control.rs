use std::sync::Arc;

use crossterm::event::{self, Event, KeyEvent};
use ratatui::{
    DefaultTerminal,
    widgets::{StatefulWidget, Widget},
};
use tokio::{io, sync::mpsc::Receiver};

use crate::app::{self, appstate::AppWidget, connected_room::Room, disconnected_room::WaitingRoom};

#[derive(Debug)]
enum AppState {
    Waiting,
    RoomConnected,
    Closed,
}

#[derive(Debug, PartialEq)]
pub enum AppAction {
    None,
    GoToWaitingRoom,
    GoToRoom(String),
    Quit,
}

#[derive(Debug)]
pub struct App {
    appstate: AppState,
    waiting: WaitingRoom,
    room: Option<Room>,
    url: String
}

impl App {
    pub fn new() -> App {
        App {
            appstate: AppState::Waiting,
            waiting: WaitingRoom::new(),
            room: None,
            url: std::env::var("BASE_URL").unwrap_or_else(|e| {
                println!("Unable to get base url. defaulting to localhost");
                "127.0.0.1".to_string()
            })
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            match &self.appstate {
                AppState::Closed => {
                    break;
                }
                _ => {
                    terminal.draw(|f| {
                        let area = f.area();
                        let buffer = f.buffer_mut();
                        let widget = self.as_widget();
                        widget.render(area, buffer);
                    }).unwrap();
                    
                    if event::poll(std::time::Duration::from_millis(16))? {
                        if let Event::Key(key) = event::read().unwrap() {
                            let action = self.handle_key(key).await;
                            self.handle_event(action).await; 
                        }
                    }
                    
            
                }
            }
        }
        Ok(())
    }
    
    async fn handle_key(&mut self, key: KeyEvent) -> AppAction {
        match self.appstate {
            AppState::Waiting => {
                return self.waiting.handle_keys(key);
            }, 
            AppState::RoomConnected => {
               let room =  self.room.as_mut().unwrap();
               return room.handle_keys(key).await;
            }, 
            _ => return AppAction::None
        }
    }

    fn as_widget<'a>(&'a mut self) -> AppWidget<'a> {
        match self.appstate {
            AppState::Waiting =>  AppWidget::Waiting(&mut self.waiting),
            AppState::RoomConnected => match &mut self.room {
                Some(room) => {
                     return AppWidget::RoomConnected(room);
                }
                None =>  AppWidget::Waiting(&mut self.waiting),
            },
            AppState::Closed =>  AppWidget::Closed,
        }
    }
    
    async fn handle_event(&mut self, app_action: AppAction) {
        match app_action {
            AppAction::GoToWaitingRoom => {
                self.room = None;
                self.appstate = AppState::Waiting
            }, 
            AppAction::GoToRoom(room_name) => {
                let room = Room::new(room_name, self.url.to_owned()).await;
                
                if room.is_err() {
                    println!("Unable to create new room");
                    return;
                }
                
                let mut room = room.unwrap();
                
                room.read_message().await;
                self.room = Some(room)
            }, 
            AppAction::Quit => {
                self.exit();
            }
            AppAction::None => {}
        }
    }

    fn exit(&mut self) {
        self.appstate = AppState::Closed;
    }
}
