use axum::{routing::get, Router};
use std::net::SocketAddr;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

async fn index() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(index));

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 9852)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
