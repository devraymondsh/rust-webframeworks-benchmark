use tide::{self, Error, Request};

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

async fn index(_: Request<()>) -> Result<&'static str, Error> {
    Ok("Hello world!")
}

#[async_std::main]
async fn main() {
    let mut app = tide::new();

    app.at("/").get(index);

    app.listen("127.0.0.1:9852").await.unwrap();
}
