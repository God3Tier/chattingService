use std::sync::Arc;

use crossterm::event::{self, Event, KeyEvent};
use ratatui::{
    DefaultTerminal,
    widgets::{Clear},
};
use tokio::{io, sync::{Mutex, mpsc::Receiver}};

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
    GoToRoom(String, String),
    Quit,
}

#[derive(Debug)]
pub struct App {
    appstate: AppState,
    waiting: WaitingRoom,
    room: Option<Room>,
    url: String,
}

impl App {
    pub fn new() -> App {
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|e| {
            println!("Unable to get base url. defaulting to localhost");
            "127.0.0.1".to_string()
        });
        App {
            appstate: AppState::Waiting,
            waiting: WaitingRoom::new(),
            room: None,
            url: base_url,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            match &self.appstate {
                AppState::Closed => {
                    break;
                }
                _ => {
                    // println!("{:?}", self.appstate);
                    let widget = self.as_widget();
                    terminal
                        .draw(|f| {
                            let area = f.area();
                            widget.render(f, area);
                        })
                        .unwrap();

                    if event::poll(std::time::Duration::from_millis(16))?
                        && let Event::Key(key) = event::read().unwrap()
                    {
                        let action = self.handle_key(key).await;
                        self.handle_event(action).await;
                    }
                }
            }
        }
        Ok(())
    }

    async fn handle_key(&mut self, key: KeyEvent) -> AppAction {
        match self.appstate {
            AppState::Waiting => {
                self.waiting.handle_keys(key)
            }
            AppState::RoomConnected => {
                let room = self.room.as_mut().unwrap();
                return room.handle_keys(key).await;
            }
            _ => AppAction::None,
        }
    }

    fn as_widget<'a>(&'a mut self) -> AppWidget<'a> {
        match self.appstate {
            AppState::Waiting => AppWidget::Waiting(&mut self.waiting),
            AppState::RoomConnected => match &mut self.room {
                Some(room) => {
                    AppWidget::RoomConnected(room)
                }
                None => {
                    AppWidget::None
                },
            },
            AppState::Closed => AppWidget::Closed,
        }
    }

    async fn handle_event(&mut self, app_action: AppAction) {
        match app_action {
            AppAction::GoToWaitingRoom => {
                self.room = None;
                self.appstate = AppState::Waiting
            }
            AppAction::GoToRoom(room_name, username) => {
                // println!("Going to a new room post connection");
                let room = Room::new(room_name, self.url.to_owned(), username);

                if room.is_err() {
                    // println!("Unable to create new room");
                    // self.appstate = AppState::Waiting;
                    return;
                }

                let room = room.unwrap();
                self.room = Some(room);
                self.appstate = AppState::RoomConnected;
            }
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
