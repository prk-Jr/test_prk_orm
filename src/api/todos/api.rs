use crate::{
    api::{CreateTodo, Todo, TodoModel},
    responses::{self, response::AppResponse},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use responses::response::*;
use sqlx::{query, MySqlPool};

pub async fn get_all_todos(State(db): State<MySqlPool>) -> impl IntoResponse {
    let query = TodoModel::select()
        .select_str("users.username")
        .left_join_by_user_id("users", "id")
        .build();
    let response: Result<Vec<Todo>, sqlx::Error> = sqlx::query_as(&query).fetch_all(&db).await;
    match response {
        Ok(todos) => (
            StatusCode::OK,
            Json(AppResponse::Ok::<Vec<Todo>, String>(OkResponse(todos))),
        ),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppResponse::Err::<Vec<Todo>, String>(ErrResponse(
                _e.to_string(),
            ))),
        ),
    }
}

pub async fn create_a_todo(
    State(db): State<MySqlPool>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let query = TodoModel::insert()
        .insert_to_user_id(payload.user_id)
        .insert_to_task(payload.task)
        .build();

    let response = sqlx::query(&query).execute(&db).await;
    match response {
        Ok(created) => (
            StatusCode::CREATED,
            Json(AppResponse::Ok(OkResponse(created.last_insert_id()))),
        ),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppResponse::Err(ErrResponse(_e.to_string()))),
        ),
    }
}

pub async fn get_all_todos_for_user_by_user_id(
    State(db): State<MySqlPool>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    let query = TodoModel::select().where_user_id(user_id).build();
    let response: Result<Vec<CreateTodo>, sqlx::Error> =
        sqlx::query_as(&query).fetch_all(&db).await;
    match response {
        Ok(todos) => (StatusCode::OK, Json(AppResponse::Ok(OkResponse(todos)))),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppResponse::Err(ErrResponse(_e.to_string()))),
        ),
    }
}
