use std::{fs::OpenOptions, io::Write, sync::Arc};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{
    Mutex,
    mpsc::{Receiver, Sender},
};
use tokio_tungstenite::connect_async;

use crate::{Err, response::Response};

pub async fn start_listening(
    url: String,
    ending_rx: tokio::sync::watch::Receiver<bool>,
    mut user_input_rx: Receiver<String>,
    server_message_sx: Sender<Response>,
) -> Result<(), Err> {
    let file_to_write = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open("./socket_log.txt")
        .unwrap_or_else(|e| panic!("Unable to read file"));

    let file_to_write = Arc::new(Mutex::new(file_to_write));
    let writer_file_mutex = Arc::clone(&file_to_write);

    let connection = connect_async(&url).await;

    if connection.is_err() {
        println!("Cannot connect to server {:?}", connection.err());
        return Err("Unable to connect to the url provided".into());
    }

    let (ws_stream, _) = connection.unwrap();

    // println!("{response:?}");
    // println!("Successfully connected to room: {room_id}");

    // let (terminal_tx, mut terminal_rx) = mpsc::channel::<String>(100);
    let (mut write, read) = ws_stream.split();

    tokio::spawn(async move {
        // This is the thread logic to hold the message from the terminal and send it to the server. Should be modified to accomodate
        // app state soon.
        while let Some(res) = user_input_rx.recv().await {
            if *ending_rx.borrow() {
                let mut file_lock = writer_file_mutex.lock().await;
                file_lock.write("Received cancel command".as_bytes());
                drop(file_lock);
                break;
            }
            let msg = tokio_tungstenite::tungstenite::Message::from(res);
            // println!("{msg:?}");
            match write.send(msg).await {
                Ok(_) => {},
                Err(e) => {
                    let mut file_lock = writer_file_mutex.lock().await;
                    file_lock.write("Unable to send message {e:?}".as_bytes());
                    drop(file_lock)
                }
            }
        }
        
        let mut file_lock = writer_file_mutex.lock().await;
        match write.close().await {
            Ok(_) => {
                file_lock.write("Writer closed successfully".as_bytes());
            }, 
            Err(e) => {
                file_lock.write("Unable to close the write resource {e:?}".as_bytes());
            }
        }
        drop(file_lock)
    });

    read.for_each(|message| async {
        match message {
            Ok(data) => {
                let res = Response::new(data.into_data());
                
                match server_message_sx
                        .send(res)
                        .await
                {
                    Ok(_) => {},
                    Err(e) => {
                        let mut file_lock = file_to_write.lock().await;
                        file_lock.write("Unable to disconnect because of {e:?}".as_bytes());
                        drop(file_lock);
                    }
                }
                
            }
            Err(e) => {
                let mut file_lock = file_to_write.lock().await;
                file_lock.write("Successful disconnect".as_bytes());
                drop(file_lock);
            }
        }
    })
    .await;

    Ok(())
}
