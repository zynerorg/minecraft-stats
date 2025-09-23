// @generated automatically by Diesel CLI.

diesel::table! {
    player_stats (player_id, category_id, target) {
        player_id -> Int4,
        category_id -> Int4,
        target -> Text,
        count -> Int4,
    }
}

diesel::table! {
    players (id) {
        id -> Int4,
        username -> Text,
    }
}

diesel::table! {
    stat_categories (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::joinable!(player_stats -> players (player_id));
diesel::joinable!(player_stats -> stat_categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    player_stats,
    players,
    stat_categories,
);
