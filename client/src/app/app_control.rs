use ratatui::{DefaultTerminal, widgets::Widget};
use tokio::{io, sync::mpsc::Receiver};

use crate::app::appstate::AppState;

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

    fn as_widget(&self) -> impl Widget + '_ {
        match self.appstate {
            AppState::Waiting => return WaitingWidget,
            AppState::RoomConnected => {}
            AppState::Closed => {}
        }
    }

    async fn handle_waiting_room(&self) {}

    fn exit(&mut self) {
        self.appstate = AppState::Closed;
    }
}
