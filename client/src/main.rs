use dotenv::dotenv;
type Err = Box<dyn std::error::Error>;

fn request_for_room_id() -> String {
    "".to_string()
}

fn request_for_username() -> String {
    "".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Err>{
    dotenv().ok();
    
    let room_id = request_for_room_id();
    let username = request_for_username();
    let base_url = std::env::var("HOST_URL").unwrap_or_else(|e| {
        panic!("The host url has not been set in .env");
    });
    
    let uri = format!("ws/{base_url}/ws/joinroom?room_id={room_id}&username={username}");
    
    println!("Hello, world!");
    
    Ok(())
}
