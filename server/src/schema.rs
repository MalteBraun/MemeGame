// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Text,
        user_name -> Text,
        points_game -> Text,
        points_total -> Nullable<Text>,
    }
}
