use std::{collections::HashMap, sync::Arc};

use tokio::sync::{self, Mutex, mpsc::{self, Receiver, Sender}};

use crate::{Err, message::Message, user::{User}};

#[derive(Debug)]
pub struct Room {
    room_id: String, 
    members: HashMap<u32, (mpsc::Sender<Arc<Message>>, sync::watch::Sender<bool>)>,
    sender: Sender<Arc<Message>>, 
    pub is_closed: bool
}

impl Room {
    pub fn spawn_room(room_id: String) -> (Room, Receiver<Arc<Message>>) {
        let (room_tx, room_rx) = mpsc::channel::<Arc<Message>>(100);
        let room = Room {
            room_id,
            members: HashMap::new(),
            sender: room_tx,
            is_closed: false
        };
        
        (room, room_rx)
    }
    
    pub async fn add_user(&mut self, user: Arc<Mutex<User>>) {
        println!("Attempting to lock user");
        let mut user = user.lock().await;
        println!("Able to unlock user");
        user.set_room(self.sender.clone());
        self.members.insert(user.user_id, (user.user_session_tx.clone(), user.shutdown_tx.clone()));
        drop(user);
        println!("Successfully dropped the user");
    }
    
    pub async fn run (room: Arc<Mutex<Room>>, mut room_rx: Receiver<Arc<Message>>) {
        while let Some( msg) = room_rx.recv().await {
            let temp_read = msg.as_ref();
            println!("Received room message: {temp_read:?}");
            let borrow_room = room.lock().await;
            let members = &borrow_room.members;
            for (id , user_session_tx) in members {
                println!("Sending to user {id}");
                let send_to = user_session_tx.0.clone();
                send_to.send(Arc::clone(&msg)).await.unwrap_or_else(|_| {
                    println!("User {id} is unable to send message")
                });
            }
            drop(borrow_room);
        }
    }
    
    pub async fn disconnect_user(&mut self, user_id: u32)  -> Result<(), Err>{
        let user = self.members.remove(&user_id);
        if user.is_none() {
            println!("Nothing inside");
            return Ok(())
        }
        
        let user = user.unwrap();
        drop(user.0);
        println!("Successfully dropped the sender");
        user.1.send(true).unwrap_or_else(|e| {
            println!("Unable to send to user close {e:?}")
        });
        println!("Succesfully assigned false sender");
        
        if self.members.is_empty() {
            println!("Room will close now from Room struct");
            // self.sender.closed().await;
            self.is_closed = true;
        }
        
        println!("All is fine in paradise");
        Ok(())
    }
}