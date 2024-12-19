use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::DbError;

#[derive(Debug, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub pass: String,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub pass: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub user_id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub game_id: Option<String>,
    pub score: Option<i32>,
}

impl User {
    pub fn new(input: NewUser) -> User {
        User {
            user_id: xid::new().to_string(),
            username: input.username,
            pass: input.pass,
            avatar: input.avatar,
        }
    }

    pub fn to_player(self) -> Player {
        Player {
            user_id: self.user_id,
            username: self.username,
            avatar: self.avatar,
            game_id: None,
            score: None,
        }
    }

    pub fn register(conn: &mut SqliteConnection, user: User) -> Result<Option<usize>, DbError> {
        use crate::schema::users::dsl::*;

        let insert_query = diesel::insert_into(users).values(&user).execute(conn)?;

        if insert_query > 0 {
            Ok(Some(insert_query))
        } else {
            Ok(None)
        }
    }

    pub fn login(
        conn: &mut SqliteConnection,
        name: String,
        pin: String,
    ) -> Result<Option<Player>, DbError> {
        use crate::schema::users::dsl::*;

        let login_query = users
            .filter(username.eq(name))
            .filter(pass.eq(pin))
            .first::<User>(conn)
            .optional()?;

        let response = login_query.map(|user| user.to_player());

        Ok(response)
    }
}
