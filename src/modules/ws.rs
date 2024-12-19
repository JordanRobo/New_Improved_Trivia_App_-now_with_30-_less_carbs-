use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_ws::Message;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct AppState {
    pub clients: Mutex<HashMap<usize, actix_ws::Session>>,
    pub counter: AtomicUsize,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "action", content = "payload")]
enum GameReq {
    CreateGame,
    JoinGame { game_id: String },
}

#[derive(Serialize, Debug)]
enum GameRes {
    GameCreated { game_id: String },
    GameJoined { game_id: String },
    Error { message: String },
}

pub async fn ws(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let (response, session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    // Generate unique client ID
    let id = app_state.counter.fetch_add(1, Ordering::Relaxed);

    // Store client session
    app_state.clients.lock().await.insert(id, session.clone());

    // Spawn client handler
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Text(text) => {
                    let mut clients = app_state.clients.lock().await;
                    // Broadcast message to all other clients
                    for (&client_id, client) in clients.iter_mut() {
                        if client_id != id {
                            let _ = client.text(text.to_string()).await;
                        }
                    }
                }
                Message::Close(_) => {
                    app_state.clients.lock().await.remove(&id);
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(response)
}
