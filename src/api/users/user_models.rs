use prkorm::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Table, Debug, FromRow, Serialize, Deserialize)]
#[table_name("users")]
pub struct User {
    id: i32,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub email: String,
}
