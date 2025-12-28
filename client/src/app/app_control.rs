use ratatui::{
    DefaultTerminal,
    widgets::{StatefulWidget, Widget},
};
use tokio::{io, sync::mpsc::Receiver};

use crate::app::{appstate::AppWidget, connected_room::Room, disconnected_room::WaitingRoom};

#[derive(Debug)]
enum AppState {
    Waiting,
    RoomConnected,
    Closed,
}

pub enum AppAction {
    None,
    GoToWaitingRoom,
    GoToRoom(String),
    Quit,
}

#[derive(Debug)]
pub struct App {
    appstate: AppState,
    room_name: Option<String>,
    waiting: WaitingRoom,
    room: Option<Room>,
}

impl App {
    pub fn new() -> App {
        App {
            appstate: AppState::Waiting,
            room_name: None,
            waiting: WaitingRoom::new(),
            room: None,
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
                    });
                    
                    self.handle_event(); 
                }
            }
        }
        Ok(())
    }

    fn as_widget(&mut self) -> AppWidget {
        match self.appstate {
            AppState::Waiting => return AppWidget::Waiting(&mut self.waiting),
            AppState::RoomConnected => match &mut self.room {
                Some(room) => {
                    return AppWidget::RoomConnected(room);
                }
                None => return AppWidget::Waiting(&mut self.waiting),
            },
            AppState::Closed => return AppWidget::Closed,
        }
    }
    
    fn handle_event(&mut self) {
        // Handle when the room is empty 
        if self.room_name.is_none() {
            
        }
    }

    async fn handle_waiting_room(&self) {}

    fn exit(&mut self) {
        self.appstate = AppState::Closed;
    }
}
