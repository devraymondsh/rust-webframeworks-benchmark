use salvo::{
    async_trait,
    prelude::{fn_handler, Request, Router, Server, TcpListener},
    routing, Depot, Handler, Response, Writer,
};

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[fn_handler]
async fn index() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() {
    let router = Router::new().get(index);

    Server::new(TcpListener::bind("127.0.0.1:9852"))
        .serve(router)
        .await;
}
