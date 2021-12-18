use thruster::{m, middleware_fn};
use thruster::{App, BasicContext as Ctx, Request, Server, ThrusterServer};
use thruster::{MiddlewareNext, MiddlewareResult};

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[middleware_fn]
async fn index(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.body("Hello World!");

    Ok(context)
}

fn main() {
    let mut app = App::<Request, Ctx, ()>::new_basic();

    app.get("/", m![index]);

    let server = Server::new(app);
    
    server.start("127.0.0.1", 9852);
}