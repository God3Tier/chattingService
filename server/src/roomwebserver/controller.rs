use std::sync::Arc;

use actix_web::{
    HttpRequest, HttpResponse,
    web::{self, Payload, Query},
};

use tokio::sync::{Mutex, mpsc};

use crate::{
    Err, RoomMap, UserMap,
    dto::RoomInfoDTO,
    message::Message,
    roomwebserver::server::Room,
    user::{User},
};

// This function is to establish the connection between the client and the server room
// that is being attempted to join
pub async fn join_room(
    req: HttpRequest,
    stream: Payload,
    details: Query<RoomInfoDTO>,
    rooms: web::Data<RoomMap>,
    users: web::Data<UserMap>,
) -> Result<HttpResponse, Err> {
    let (res, session, receive_session) = match actix_ws::handle(&req, stream) {
        Ok(tuple) => tuple,
        Err(e) => {
            println!("Unable to spawn websocket for user because of {e}");
            return Err("Unable to spawn websocket. Please try again".into());
        }
    };

    println!("Server side connection successgul!");

    let mut guard_room = rooms.lock().await;
    println!("Able to claim room lock");
    let mut guard_user_room = users.lock().await;
    println!("Able to claim the user room");
    let room = match guard_room.get(&details.room_id) {
        Some(room) => {
            println!("Taking room of {:?}", details.room_id);
            Arc::clone(room)
        }
        None => {
            println!("Opening room");
            guard_user_room.insert(details.room_id.to_owned(), Vec::new());
            let (room, room_rx) = Room::spawn_room(details.room_id.to_owned());
            let room = Arc::new(Mutex::new(room));
            let room_clone = Arc::clone(&room);
            tokio::spawn(async move { Room::run(room_clone, room_rx).await });
            room
        }
    };
    guard_room.insert(details.room_id.clone(), Arc::clone(&room));
    drop(guard_room);

    println!("Attempting to claim borrow_room");
    let mut borrow_room = room.lock().await;
    println!("Able to claim the borrow room lock");
    let uuid = rand::random();
    let (user_tx, user_rx) = mpsc::channel::<Arc<Message>>(32);
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    let user = User::new(uuid, details.username.to_owned(), user_tx, shutdown_tx);
    let user = Arc::new(Mutex::new(user));
    borrow_room.add_user(Arc::clone(&user)).await;
    println!("Attempting to drop borrow room");
    drop(borrow_room);
    println!("Dropped borrow room");
    User::spawn_user_threads(
        Arc::clone(&user),
        session,
        receive_session,
        user_rx,
        shutdown_rx,
        Arc::clone(&room),
    )
    .await;

    // Theoretically there should always be a user in this case.
    guard_user_room
        .get_mut(&details.room_id)
        .unwrap()
        .push(user);

    drop(guard_user_room);
    println!("Successfully added user to room!");
    
    Ok(res)
}

pub async fn get_user_connections(
    rooms: web::Data<RoomMap>,
    users: web::Data<UserMap>,
) -> HttpResponse {
    let users = users.lock().await;
    let rooms = rooms.lock().await;
    HttpResponse::Ok().body(format!(
        "User connections: \n{users:?}\n Available rooms: \n {rooms:?}"
    ))
}
