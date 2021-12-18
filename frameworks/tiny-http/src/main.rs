use tiny_http;

use std::sync::Arc;
use std::thread;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    let server = Arc::new(tiny_http::Server::http("127.0.0.1:9852").unwrap());

    let mut handles = Vec::new();

    for _ in 0..4 {
        let server = server.clone();

        handles.push(thread::spawn(move || {
            for rq in server.incoming_requests() {
                let response = tiny_http::Response::from_string("hello world".to_string());
                let _ = rq.respond(response);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
