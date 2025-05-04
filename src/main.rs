mod handler;

use std::net::SocketAddr;

use axum::{Router, routing::post};
use tokio;

use handler::make_reservation;

#[tokio::main]
async fn main() {
    // Setup the routing

    let app = Router::new().route("/reserve", post(make_reservation));

    let address = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running at http://{}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let _ = axum::serve(listener, app).await.unwrap();
}
