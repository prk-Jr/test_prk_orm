use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::MySqlPool;

use super::{User, UserPayload};

pub async fn get_users(State(db): State<MySqlPool>) -> impl IntoResponse {
    let query = User::select().build();
    let users: Result<Vec<User>, sqlx::Error> = sqlx::query_as(&query).fetch_all(&db).await;
    match users {
        Ok(users) => (StatusCode::OK, Json(users)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<User>::new())),
    }
}

pub async fn get_user_by_id(State(db): State<MySqlPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let query = User::select().where_id(id).build();
    let users: Result<User, sqlx::Error> = sqlx::query_as(&query).fetch_one(&db).await;
    match users {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(_) => Err((StatusCode::NOT_FOUND, Json("User not found"))),
    }
}

pub async fn create_user(
    State(db): State<MySqlPool>,
    Json(payload): Json<UserPayload>,
) -> StatusCode {
    let query = User::insert()
        .insert_to_username(&payload.username)
        .insert_to_email(&payload.email)
        .build();
    let response = sqlx::query(&query).execute(&db).await;
    match response {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn update_username_email_by_id(
    State(db): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UserPayload>,
) -> impl IntoResponse {
    let query = User::update()
        .update_username_with_value(&payload.username)
        .update_email_with_value(&payload.email)
        .update_where_id_eq(id);
    let response = sqlx::query(&query).execute(&db).await;
    match response {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_user_by_id(
    State(db): State<MySqlPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let query = User::delete().delete_where_id_eq(id);
    let response = sqlx::query(&query).execute(&db).await;
    match response {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
