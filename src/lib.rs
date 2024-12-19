use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use tokio::sync::Mutex;

pub mod db;
pub mod modules;
pub mod schema;
pub mod utils;

use modules::{games, routes, users, ws};

#[derive(RustEmbed)]
#[folder = "app/dist"]
struct Asset;

pub fn run() -> std::io::Result<Server> {
    let db = db::establish_connection();
    let app_data = web::Data::new(db);

    let state = Arc::new(ws::AppState {
        clients: Mutex::new(HashMap::new()),
        counter: AtomicUsize::new(0),
    });

    let app_state = web::Data::new(state);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .app_data(app_state.clone())
            .wrap(Cors::permissive())
            .configure(routes::routes_config)
            .route("/ws", web::get().to(ws::ws))
            .service(index)
            .service(dist)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(server)
}

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

#[get("/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}
