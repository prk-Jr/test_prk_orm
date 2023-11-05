use prkorm::Table;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Table)]
#[table_name("todos")]
pub struct TodoModel {
    id: i32,
    user_id: i32,
    task: String,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateTodo {
    pub id: Option<i32>,
    pub user_id: i32,
    pub task: String,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub task: String,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
