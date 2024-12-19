use crate::{
    db::DbError,
    users::{Player, User},
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::games)]
pub struct Game {
    pub game_id: String,
    pub host_id: String,
    pub status: Option<i32>,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::player_scores)]
pub struct PlayerScore {
    pub game_id: String,
    pub player_id: String,
    pub score: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRequest {
    pub user_id: String,
    pub game_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerResponse {
    pub user_id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub score: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct GameResponse {
    pub game_id: String,
    pub host_id: String,
    pub status: Option<i32>,
    pub players: Vec<PlayerResponse>,
}

impl GameResponse {
    pub fn from_models(game: Game, players: Vec<(User, Option<PlayerScore>)>) -> Self {
        GameResponse {
            game_id: game.game_id,
            host_id: game.host_id,
            status: game.status,
            players: players
                .into_iter()
                .map(|(user, player_score)| PlayerResponse {
                    user_id: user.user_id,
                    username: user.username,
                    avatar: user.avatar,
                    score: player_score.and_then(|ps| ps.score),
                })
                .collect(),
        }
    }
}

pub fn get_game(
    conn: &mut SqliteConnection,
    game_id: String,
) -> Result<Option<GameResponse>, DbError> {
    use crate::schema::{games, player_scores, users};

    // First fetch the game
    let game = games::table
        .filter(games::game_id.eq(&game_id))
        .first::<Game>(conn)
        .optional()?;

    let game = match game {
        Some(game) => game,
        None => return Ok(None),
    };

    // Then fetch all players and their scores
    let players: Vec<(User, Option<PlayerScore>)> = users::table
        .left_join(
            player_scores::table.on(users::user_id
                .eq(player_scores::player_id)
                .and(player_scores::game_id.eq(&game_id))),
        )
        .select((User::as_select(), Option::<PlayerScore>::as_select()))
        .load(conn)?;

    Ok(Some(GameResponse::from_models(game, players)))
}

pub fn join_game(
    conn: &mut SqliteConnection,
    player: JoinRequest,
) -> Result<Option<Player>, DbError> {
    use crate::schema::{player_scores::dsl::*, users};

    // First check if the user is already in the game
    let existing_player = player_scores
        .filter(game_id.eq(&player.game_id))
        .filter(player_id.eq(&player.user_id))
        .first::<PlayerScore>(conn)
        .optional()?;

    if existing_player.is_some() {
        return Ok(None); // Player already in game
    }

    // Create new player score entry
    let new_score = PlayerScore {
        game_id: player.game_id.clone(),
        player_id: player.user_id.clone(),
        score: Some(0),
    };

    // Insert the new score in a transaction
    conn.transaction(|conn| {
        diesel::insert_into(player_scores)
            .values(&new_score)
            .execute(conn)?;

        // Fetch user information to return complete Player struct
        let user: User = users::table
            .filter(users::user_id.eq(&player.user_id))
            .first(conn)?;

        Ok(Some(Player {
            user_id: user.user_id,
            username: user.username,
            avatar: user.avatar,
            game_id: Some(player.game_id),
            score: Some(0),
        }))
    })
}
