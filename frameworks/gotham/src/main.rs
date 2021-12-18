//! A Hello World example application for working with Gotham.
use gotham::state::State;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// Create a `Handler` which is invoked when responding to a `Request`.
///
/// How does a function become a `Handler`?.
/// We've simply implemented the `Handler` trait, for functions that match the signature used here,
/// within Gotham itself.
pub fn index(state: State) -> (State, &'static str) {
    (state, "Hello World!")
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    gotham::start("127.0.0.1:9852", || Ok(index)).unwrap()
}
