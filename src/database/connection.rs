use sqlx::*;

pub async fn connect() -> MySqlPool {
    MySqlPool::connect("YOUR DATABSE URL")
        .await
        .expect("Could not connect to database")
}
