use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::{self, Sender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{Message};

use crate::response::Response;

pub async fn start_listening(url: String, room_id: String, app_sx: Sender<String>) {
    
    let (ws_stream, response) = connect_async(&url)
        .await
        // Remove this panic for a better way to handle the stream
        .unwrap_or_else(|e| panic!("Unable to connect to provided room \n {e:?}"));
    
    
    println!("{response:?}");
    println!("Successfully connected to room: {room_id}");

    let (terminal_tx, mut terminal_rx) = mpsc::channel::<String>(100);
    let (mut write, read) = ws_stream.split();

    /*
     * Spawn thread to read message from the receiving end of the serVer
     */
    tokio::spawn(async move {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            // This is for the time being until I find a better way to display the information (preferably tui for now)
            match Response::new(data) {
                Ok(response) => {
                    let sender = response.sender.unwrap();
                    let content = response.content.unwrap();
                    
                    // Somehow this needs to be sent to the app information 
                    println!("{sender}: {content}");
                }
                Err(e) => {
                    println!("Unable to parse response\n{e}");
                }
            }
        })
        .await;
    });

    /*
     * To be removed 
     * This receives direct input from the terminal and sends it to the channel for sneding to server
     */
    tokio::spawn(async move { receive_from_buffer(terminal_tx).await });

    
    // This is the thread logic to hold the message from the terminal and send it to the server. Should be modified to accomodate 
    // app state soon. 
    while let Some(res) = terminal_rx.recv().await {
        let msg = Message::from(res);
        // println!("{msg:?}");
        write
            .send(msg)
            .await
            .unwrap_or_else(|e| println!("Unable to send the message {e:?}"))
    }
    
}

/*
 * To be depricated 
 * This handles ability to receive input from the terminal
 */
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
