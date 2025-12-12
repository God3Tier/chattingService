use actix_web::rt;
use actix_ws::{CloseCode, CloseReason, MessageStream, Session};
use std::{sync::Arc};
use tokio::{
    sync::mpsc::{self, Receiver}
};

use crate::{Err, message::Message};

pub struct User {
    pub user_id: u32,
    pub username: Arc<String>,
    pub user_session_tx: mpsc::Sender<Arc<Message>>,
    room_sender: Option<mpsc::Sender<Arc<Message>>>,
    pub shutdown_tx: tokio::sync::watch::Sender<bool>,
}

impl User {
    pub fn new(
        user_id: u32,
        username: String,
        session: Session,
        write_session: MessageStream,
    ) -> User {
        // Session is to send messages into a websocket
        // MessageStream is to write messages into a websocket
        let (user_tx, user_rx) = mpsc::channel::<Arc<Message>>(32);
        let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
        let username = Arc::new(username);

        let user = User {
            user_id,
            username: Arc::clone(&username),
            user_session_tx: user_tx,
            room_sender: None,
            shutdown_tx,
        };
        user.spawn_user_threads(session, write_session, user_rx, shutdown_rx);

        user
    }

    fn spawn_user_threads(
        &self,
        mut session: Session,
        mut write_session: MessageStream,
        mut user_rx: Receiver<Arc<Message>>,
        shutdown_rx: tokio::sync::watch::Receiver<bool>,
    ) {
        let shutdown_rx_1 = shutdown_rx.clone();
        let shutdown_rx_2 = shutdown_rx.clone();
        let borrow_username = Arc::clone(&self.username);
        tokio::spawn(async move {
            while let Some(msg) = user_rx.recv().await {
                if shutdown_rx_1.has_changed().unwrap_or_else(|e| {
                    println!("Channel has already been closed err {e:?}!");
                    true
                }) {
                    break;
                }
                let msg = &*msg;
                session
                    .text(serde_json::to_string(msg).unwrap_or("message not found".to_string()))
                    .await
                    .unwrap_or_else(|e| {
                        println!("Channel has been closed! {e:?}");
                    });
            }

            match session
                .close(Some(CloseReason {
                    code: CloseCode::Normal,
                    description: Some(String::from("User has closed the channel!")),
                }))
                .await
            {
                Ok(_) => println!("Closed session successfully!"),
                Err(e) => println!("Unable to disconnect from server! {e:?}"),
            };
        });

        let room_info = self.room_sender.as_ref().unwrap().clone();
        rt::spawn(async move {
            while let Some(msg) = write_session.recv().await {
                if shutdown_rx_2.has_changed().unwrap_or_else(|e| {
                    println!("Channel has already been closed err {e:?}!");
                    true
                }) {
                    break;
                }
                match msg {
                    Ok(msg) => match msg {
                        actix_ws::Message::Text(txt) => {
                            println!("Message received! {txt}");
                            let txt = txt.to_string();
                            // let room_name = room_name.as_deref().unwrap().clone();
                            let msg = Arc::new(Message::new(Arc::clone(&borrow_username), txt));
                            room_info
                                .send(msg)
                                .await
                                .unwrap_or_else(|e| println!("Unable to send message {e:?}"));
                        }
                        actix_ws::Message::Binary(byt) => {}
                        actix_ws::Message::Continuation(cont) => {}
                        actix_ws::Message::Ping(ping) => {}
                        actix_ws::Message::Pong(pong) => {}
                        actix_ws::Message::Close(msg) => {
                            println!("The channel has been closed becasue of {msg:?}");
                            break;
                        }
                        actix_ws::Message::Nop => {}
                    },
                    Err(e) => {
                        println!("Unable to read stream, {e:?}")
                    }
                }
            }
        });
    }

    pub fn set_room(&mut self, room_sender: mpsc::Sender<Arc<Message>>) {
        self.room_sender = Some(room_sender)
    }

    pub async fn send_message(&self, msg: String) -> Result<(), Err> {
        let sender = self.room_sender.as_ref();

        if sender.is_none() {
            return Err("User not connected to the room yet".into());
        }

        let sender = sender.unwrap();
        let msg = Message::new(Arc::clone(&self.username), msg);

        if sender.send(Arc::new(msg)).await.is_err() {
            return Err("Unable to send message".into());
        }

        Ok(())
    }

    pub async fn disconnect_user(&self) -> Result<(), Err> {
        self.user_session_tx.closed().await;
        self.shutdown_tx.closed().await;
        Ok(())
    }
}
