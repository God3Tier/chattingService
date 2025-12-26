use ratatui::{DefaultTerminal, widgets::{ StatefulWidget, Widget}};
use tokio::{io, sync::mpsc::Receiver};

use crate::app::{
    appstate::AppWidget, connected_room::Room, disconnected_room::WaitingRoom
};

#[derive(Debug)]
enum AppState {
    Waiting,
    RoomConnected,
    Closed,
}

#[derive(Debug)]
pub struct App {
    appstate: AppState,
    app_rx: Receiver<String>,
    room_name: Option<String>,
}

impl App {
    pub fn new(app_rx: Receiver<String>) -> App {
        App {
            appstate: AppState::Waiting,
            app_rx,
            room_name: None,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            match &self.appstate {
                AppState::Waiting => {}
                AppState::RoomConnected => {}
                AppState::Closed => {
                    break;
                }
            }
        }
        Ok(())
    }

    fn as_widget(&self) -> AppWidget {
        match self.appstate {
            AppState::Waiting => return AppWidget::Waiting(WaitingRoom::new()),
            AppState::RoomConnected => return AppWidget::RoomConnected(Room::new()),
            AppState::Closed => return AppWidget::Closed
        }
    }

    async fn handle_waiting_room(&self) {}

    fn exit(&mut self) {
        self.appstate = AppState::Closed;
    }
}
