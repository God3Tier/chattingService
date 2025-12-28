use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{response::Response};

pub async fn start_listening(url: String, room_id: String, mut user_input_rx: Receiver<String>, server_message_sx: Sender<Response>) {
    
    let (ws_stream, response) = connect_async(&url)
        .await
        // Remove this panic for a better way to handle the stream
        .unwrap_or_else(|e| panic!("Unable to connect to provided room \n {e:?}"));
    
    
    println!("{response:?}");
    println!("Successfully connected to room: {room_id}");

    // let (terminal_tx, mut terminal_rx) = mpsc::channel::<String>(100);
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
                    server_message_sx.send(response);
                    
                }
                Err(e) => {
                    println!("Unable to parse response\n{e}");
                }
            }
        })
        .await;
    });

    
    // This is the thread logic to hold the message from the terminal and send it to the server. Should be modified to accomodate 
    // app state soon. 
    while let Some(res) = user_input_rx.recv().await {
        let msg = tokio_tungstenite::tungstenite::Message::from(res);
        // println!("{msg:?}");
        write
            .send(msg)
            .await
            .unwrap_or_else(|e| println!("Unable to send the message {e:?}"))
    }
    
}