mod db;
mod handler;

use std::{env, net::SocketAddr, sync::Arc};

use axum::{Extension, Router, routing::post};
use dotenvy::dotenv;
use sqlx::PgPool;
use tokio;

use handler::make_reservation;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file");
    let pgpool = match PgPool::connect(&db_url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to the DB: {}", e);
            return;
        }
    };
    let shared_pool = Arc::new(pgpool);

    // Setup the routing
    let app = Router::new()
        .route("/reserve", post(make_reservation))
        .layer(Extension(shared_pool));

    let address = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running at http://{}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let _ = axum::serve(listener, app).await.unwrap();
}
