use std::{collections::HashMap, sync::Arc, time::Duration};

use actix_web::{App, HttpServer, web};
use tokio::{sync::Mutex};

use crate::{
    roomwebserver::{controller, server::Room},
    user::User,
};

mod dto;
mod message;
mod roomwebserver;
mod user;

type RoomMap = Arc<Mutex<HashMap<String, Arc<Mutex<Room>>>>>;
type UserMap = Arc<Mutex<HashMap<String, Vec<Arc<Mutex<User>>>>>>;
type Err = Box<dyn std::error::Error>;

async fn delog_rooms(rooms: &RoomMap) {
    let mut rooms = rooms.lock().await;

    let to_remove: Vec<String> = {
        let mut v = Vec::new();
        for (name, room) in rooms.iter() {
            let borrow_room = room.lock().await;
            if borrow_room.is_closed {
                v.push(name.clone());
            }
        }
        v
    };

    for name in to_remove {
        rooms.remove(&name);
    }
}

async fn delog_user(users: &UserMap) {
    let mut remove_user = Vec::new();

    let mut user_lock = users.lock().await;
    let get_room_keys = user_lock.clone();
    let room_names: Vec<&String> = get_room_keys.keys().collect();

    for room_name in room_names {
        for (indx, user) in user_lock.get(room_name).unwrap().iter().enumerate() {
            println!("Attempting to take the lock during usewr sweep");
            let temp_lock = user.lock().await;
            println!("Successfully taken the lock");
            if temp_lock.disconnected {
                println!("Deleting user: {:?}", users);
                remove_user.push((room_name, indx));
            }
        }
    }

    for (room_name, indx) in remove_user {
        let removing_vec = user_lock
            .get_mut(room_name)
            // Get rid of this unwrap later
            .unwrap();
        removing_vec.remove(indx);
        
        if removing_vec.is_empty() {
            user_lock.remove(room_name);
        }
    }

    // println!("User Mapping state : {user_lock:?}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rooms: RoomMap = Arc::new(Mutex::new(HashMap::new()));
    let users: UserMap = Arc::new(Mutex::new(HashMap::new()));

    let room_deloger = Arc::clone(&rooms);
    let user_deloger = Arc::clone(&users);

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            // println!("Sweeping rooms");
            delog_rooms(&room_deloger).await;
            interval.tick().await;
        }
    });

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            // println!("Sweeping users");
            delog_user(&user_deloger).await;
            interval.tick().await;
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&rooms)))
            .app_data(web::Data::new(Arc::clone(&users)))
            .route("/ws/joinroom", web::get().to(controller::join_room))
            .route("/users", web::get().to(controller::get_user_connections))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
