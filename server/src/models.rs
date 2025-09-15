use crate::schema::user;
use serde::{Deserialize, Serialize};

#[derive(Insertable, )]
#[table_name = "user"]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub points_game: String,
    pub points_total: String,
}