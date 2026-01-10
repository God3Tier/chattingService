use std::io::stdout;

use crossterm::{event::EnableMouseCapture, execute};
use dotenv::dotenv;

mod response;
mod websocket_function;
mod app;

type Err = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    // Start ratatui with the websocket function -> 
    let mut terminal = ratatui::init();
    crossterm::terminal::enable_raw_mode()?;
    let app_result = app::app_control::App::new().run(&mut terminal).await;
    // Remove and move elsewhere -> Moved to when room actually is started
    // websocket_function::start_listening(url, room_id, app_sx).await;
    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;
    println!("Clean up complete");
    app_result
}

