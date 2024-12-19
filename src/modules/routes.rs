use actix_web::{
    error, get, post,
    web::{self, block, Data, Json, Path},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;

use crate::{
    db::DbPool,
    games,
    users::{NewUser, User},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    pass: String,
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(login)
            .service(register)
            .service(get_game),
    );
}

#[post("/login")]
async fn login(pool: Data<DbPool>, credentials: Json<LoginRequest>) -> Result<impl Responder> {
    let name = credentials.username.clone();
    let pin = credentials.pass.clone();

    let user_auth = block(move || {
        let mut conn = pool.get()?;
        User::login(&mut conn, name, pin)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match user_auth {
        Some(user_auth) => HttpResponse::Accepted().json(user_auth),
        None => HttpResponse::Unauthorized().json("Authentication error, please try again"),
    })
}

#[post("/register")]
async fn register(pool: Data<DbPool>, credentials: Json<NewUser>) -> Result<impl Responder> {
    let new_user = User::new(credentials.into_inner());

    let query = block(move || {
        let mut conn = pool.get()?;
        User::register(&mut conn, new_user)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match query {
        Some(_) => HttpResponse::Ok().json("User successfully created."),
        None => HttpResponse::NotImplemented().finish(),
    })
}

#[get("/games/{game_id}")]
async fn get_game(path: Path<String>, pool: Data<DbPool>) -> Result<impl Responder> {
    let game_id = path.into_inner();

    let game = block(move || {
        let mut conn = pool.get()?;
        games::get_game(&mut conn, game_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match game {
        Some(game) => HttpResponse::Ok().json(game),
        None => HttpResponse::NotFound().finish(),
    })
}
