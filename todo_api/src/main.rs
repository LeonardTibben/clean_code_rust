mod controllers;
mod requests;

use std::net::{SocketAddr, Ipv4Addr};
use crate::controllers::todo_controller::create_todo_routes;

#[tokio::main]
async fn main() {
    let app = create_todo_routes().await;

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}