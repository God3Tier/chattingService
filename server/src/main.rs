use std::{collections::HashMap, sync::Arc};

use actix_web::{App, HttpServer, web};
use tokio::sync::Mutex;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rooms: RoomMap = Arc::new(Mutex::new(HashMap::new()));
    let users: UserMap = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&rooms)))
            .app_data(web::Data::new(Arc::clone(&users)))
            .route("/ws/joinroom", web::get().to(controller::join_room))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
