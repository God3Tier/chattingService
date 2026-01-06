use actix_web::rt;
use actix_ws::{CloseCode, CloseReason, MessageStream, Session};
use mongodb::bson::Uuid;
use std::{fmt::{Display, Formatter}, sync::{Arc, Weak}};
use tokio::sync::{Mutex, mpsc::{self, Receiver, Sender}};

use crate::{Err, message::Message, roomwebserver::server::Room};

#[derive(Debug)]
pub struct User {
    pub user_id: u32,
    pub username: Arc<String>,
    pub room_id: Arc<String>,
    pub user_session_tx: mpsc::Sender<Arc<Message>>,
    room_sender: Option<mpsc::Sender<Arc<Message>>>,
    pub shutdown_tx: tokio::sync::watch::Sender<bool>,
    pub disconnected: bool
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Id = {}, Username: {}", self.user_id, self.username)
    }
}

impl User {
    pub fn new(
        user_id: u32,
        username: String,
        room_id: Arc<String>,
        user_tx: Sender<Arc<Message>>,
        shutdown_tx: tokio::sync::watch::Sender<bool>
    ) -> User {
        // Session is to send messages into a websocket
        // MessageStream is to write messages into a websocket
        let username = Arc::new(username);

       User {
            user_id,
            username: Arc::clone(&username),
            room_id,
            user_session_tx: user_tx,
            room_sender: None,
            shutdown_tx,
            disconnected: false
        }
    }

    /*
     * Asynchronously spawns 2 threads to manage both user sending of message 1
     */
    pub async fn spawn_user_threads(
        user: Arc<Mutex<User>>,
        mut session: Session,
        mut write_session: MessageStream,
        mut user_rx: Receiver<Arc<Message>>,
        shutdown_rx: tokio::sync::watch::Receiver<bool>,
        room: Weak<Mutex<Room>> 
    ) {
        let shutdown_rx_1 = shutdown_rx.clone();
        let shutdown_rx_2 = shutdown_rx.clone();
        // let borrow_username = Arc::clone(&user.username);
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
            
            println!("Closing sender");
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


        rt::spawn(async move {
            let guard_user = user.lock().await;
            let borrow_username = Arc::clone(&guard_user.username);
            let borrow_room_id = Arc::clone(&guard_user.room_id);
            let room_info =  guard_user.room_sender.as_ref().unwrap_or_else(|| panic!("Unable to receive a sender")).clone();
            drop(guard_user);
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
                            let msg = Arc::new(Message::new(Uuid::new(), Arc::clone(&borrow_username), txt, Arc::clone(&borrow_room_id)));
                            room_info
                                .send(msg)
                                .await
                                .unwrap_or_else(|e| println!("Unable to send message {e:?}"));
                            println!("Successful send");
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
            if let Some(room) = room.upgrade() {
                let mut borrow_room = room.lock().await;
                let mut guard_user = user.lock().await;
                borrow_room.disconnect_user(guard_user.user_id).await.unwrap_or_else(|e| {
                    println!("Unable to close disconnect user from room because of {e:?}")
                });
                guard_user.disconnect_user().await.unwrap_or_else(|e| {
                    println!("Unable to close user because of {e:?}")
                });
                drop(borrow_room);
                drop(guard_user);
            }
        });
    }

    pub fn set_room(&mut self, room_sender: mpsc::Sender<Arc<Message>>) {
        self.room_sender = Some(room_sender)
    }

    /*
     * What is the point of this function? Still quite unsure 
     */
    pub async fn send_intiial_messages(&self, msgs: &Vec<Arc<Message>>) -> Result<(), Err> {
        let sender = self.room_sender.as_ref();

        if sender.is_none() {
            return Err("User not connected to the room yet".into());
        }
        
        let sender = sender.unwrap();
        for msg in msgs {
            println!("Sending message {msg:?}");
            if sender.send(Arc::clone(msg)).await.is_err() {
                return Err("Unable to send message".into());
            }
            println!("Succesfully sent message {msg:?}");
        }
        println!("Successuflly sent all initial messages");
        Ok(())
    }

    /*
     * Not sure what else this is supposed to do other than lock it 
     */
    pub async fn disconnect_user(&mut self) -> Result<(), Err> {
        println!("Disconnecting user!");
        self.disconnected = true;
        self.shutdown_tx.send(true).unwrap_or_else(|e|{
            println!("Unable to send shutdown message from the receiver {e:?}");
        });
        Ok(())
    }
}

impl Drop for User {
    fn drop(&mut self) {
        println!("Successfully drop user resource {}", self.username);
    }
}