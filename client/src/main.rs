use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt};
use tokio::sync::mpsc::{self, Sender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
type Err = Box<dyn std::error::Error>;

fn request_for_room_id() -> String {
    "room1".to_string()
}

fn request_for_username() -> String {
    "twinkyterror".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    dotenv().ok();

    let room_id = request_for_room_id();
    let username = request_for_username();
    let base_url = std::env::var("HOST_URL").unwrap_or_else(|e| {
        panic!("The host url has not been set in .env \n{e:?}");
    });

    let url = format!("ws://{base_url}/ws/joinroom?room_id={room_id}&username={username}");
    println!("{url}");
    let (ws_stream, response) = connect_async(&url)
        .await
        // Remove this panic for a better way to handle the stream
        .unwrap_or_else(|e| panic!("Unable to connect to provided room \n {e:?}"));
    println!("{response:?}");

    let (terminal_tx, mut terminal_rx) = mpsc::channel::<String>(100);
    let (mut write, read) = ws_stream.split();

    tokio::spawn(async move {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            println!("Response {data:?}");
            // tokio::io::stdout().write_all(&data).await.unwrap();
        })
        .await;
    });

    tokio::spawn(async move { receive_from_buffer(terminal_tx).await });

    while let Some(res) = terminal_rx.recv().await {
        let msg = Message::from(res);
        println!("{msg:?}");
        write
            .send(msg)
            .await
            .unwrap_or_else(|e| println!("Unable to send the message {e:?}"))
    }

    // while terminal_rx.recv().await

    println!("Hello, world!");

    Ok(())
}

async fn receive_from_buffer(terminal_tx: Sender<String>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = [0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };

        let mut res = Vec::from(buf);
        res.truncate(n);
        match String::from_utf8(res) {
            Ok(string) => terminal_tx
                .send(string)
                .await
                .unwrap_or_else(|e| println!("Unable to send message {e:?}")),
            Err(e) => println!("Not a valid string to be passing around {e:?}"),
        }
    }
}
