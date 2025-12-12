use std::{collections::HashMap, sync::Arc};

use tokio::sync::{self, Mutex, mpsc::{self, Receiver, Sender}};

use crate::{Err, message::Message, user::{User}};

// Currently placed T as a placeholder before I figure out how to use it 
pub struct Room {
    room_id: String, 
    members: HashMap<u32, (mpsc::Sender<Arc<Message>>, sync::watch::Sender<bool>)>,
    receiver: Receiver<Arc<Message>>,
    sender: Sender<Arc<Message>>
}


impl Room {
    pub fn spawn_room(room_id: String) -> Room {
        let (room_tx, room_rx) = mpsc::channel::<Arc<Message>>(100);
        Room {
            room_id,
            members: HashMap::new(),
            receiver: room_rx,
            sender: room_tx
        }
    }
    
    pub async fn add_user(&mut self, user: Arc<Mutex<User>>) {
        let mut user = user.lock().await;
        user.set_room(self.sender.clone());
        self.members.insert(user.user_id, (user.user_session_tx.clone(), user.shutdown_tx.clone()));
    }
    
    pub async fn run (room: Arc<Mutex<Room>>) {
        let mut borrow_room = room.lock().await;
        while let Some( msg) = borrow_room.receiver.recv().await {
            let members = &borrow_room.members;
            for (id , user_session_tx) in members {
                let send_to = user_session_tx.0.clone();
                send_to.send(Arc::clone(&msg)).await.unwrap_or_else(|_| {
                    println!("User {id} is unable to send message")
                });
            }

        }
    }
    
    pub async fn disconnect_user(&mut self, user_id: u32)  -> Result<(), Err>{
        let user = self.members.remove(&user_id);
        if user.is_none() {
            return Ok(())
        }
        
        let user = user.unwrap();
        user.0.closed().await;
        user.1.send(true);
        
        if self.members.is_empty() {
            println!("Room will close now from Room struct");
            self.receiver.close();
            self.sender.closed().await;
        }
        
        
        Ok(())
    }
}