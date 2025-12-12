use std::sync::Arc;

use actix_web::{
    HttpRequest, HttpResponse,
    web::{self, Payload, Query},
};

use tokio::sync::Mutex;

use crate::{Err, RoomMap, UserMap, dto::RoomInfoDTO, roomwebserver::server::Room, user::User};

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

    let mut guard_room = rooms.lock().await;
    let mut guard_user_room = users.lock().await;
    let room = match guard_room.get(&details.room_id) {
        Some(room) => Arc::clone(room),
        None => {
            println!("Opening room");
            guard_user_room.insert(details.room_id.to_owned(), Vec::new());
            let room = Arc::new(Mutex::new(Room::spawn_room(details.room_id.to_owned())));
            let room_clone = Arc::clone(&room);
            tokio::spawn(async move { Room::run(room_clone).await });
            room
        }
    };

    guard_room.insert(details.room_id.clone(), Arc::clone(&room));
    let mut borrow_room = room.lock().await;
    let uuid = rand::random();
    let user = User::new(uuid, details.username.to_owned(), session, receive_session);
    let user = Arc::new(Mutex::new(user));
    borrow_room.add_user(Arc::clone(&user)).await;

    // Theoretically there should always be a user in this case.
    guard_user_room
        .get_mut(&details.room_id)
        .unwrap()
        .push(user);
    drop(borrow_room);

    Ok(res)
}
