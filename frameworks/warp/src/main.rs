use warp::Filter;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello World!");

    warp::serve(routes).run(([127, 0, 0, 1], 9852)).await;
}
