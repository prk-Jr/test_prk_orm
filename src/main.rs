// Need postman

use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::*};

mod responses;

mod api;
mod database;

#[tokio::main]
async fn main() {
    let db = database::connect().await;
    let routes = Router::new()
        .route("/", get(root))
        .route("/todos", get(api::get_all_todos).post(api::create_a_todo))
        .route(
            "/todos/:user_id",
            get(api::get_all_todos_for_user_by_user_id),
        )
        .route(
            "/users/:id",
            get(api::get_user_by_id)
                .put(api::update_username_email_by_id)
                .delete(api::delete_user_by_id),
        )
        .route("/users", get(api::get_users).post(api::create_user))
        .with_state(db);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Connected to database\nServer running on http://localhost:3000");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("Could not create server")
}

async fn root() -> impl IntoResponse {
    "Hello World"
}
