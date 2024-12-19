// @generated automatically by Diesel CLI.

diesel::table! {
    games (game_id) {
        game_id -> Text,
        host_id -> Text,
        status -> Nullable<Integer>,
    }
}

diesel::table! {
    player_scores (game_id, player_id) {
        game_id -> Text,
        player_id -> Text,
        score -> Nullable<Integer>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Text,
        username -> Text,
        pass -> Text,
        avatar -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    games,
    player_scores,
    users,
);
