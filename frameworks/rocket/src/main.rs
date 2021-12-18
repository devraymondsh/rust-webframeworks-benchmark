use std::net::Ipv4Addr;
use rocket::{Config, config};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[launch]
fn rocket() -> _ {
    let cfg = Config {
        port: 9852,
        log_level: config::LogLevel::Off,
        address: Ipv4Addr::new(127, 0, 0, 1).into(),
        ..Config::debug_default()
    };

    rocket::custom(cfg).mount("/", routes![index])
}
