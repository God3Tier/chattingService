use dotenv::dotenv;
use tokio::sync::mpsc;

mod response;
mod websocket_function;
mod app;

type Err = Box<dyn std::error::Error>;

fn request_for_room_id() -> String {
    println!("Please select a room to join");
    "room1".to_string()
}


fn request_for_username() -> String {
    "agonypain".to_string()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    
    let (app_sx, app_rx) = mpsc::channel(100);
    
    let mut terminal = ratatui::init();
    let app_result = app::app_control::App::new(app_rx).run(&mut terminal).await;
    
    let room_id = request_for_room_id();
    let username = request_for_username();
    let base_url = std::env::var("HOST_URL").unwrap_or_else(|e| {
        panic!("The host url has not been set in .env \n{e:?}");
    });

    let url = format!("ws://{base_url}/ws/joinroom?room_id={room_id}&username={username}");
    println!("{url}");
    
    // websocket_function::start_listening(url, room_id, app_sx).await;
    
    ratatui::restore();
    app_result
}

