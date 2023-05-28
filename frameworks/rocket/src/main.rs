use file_fetch::fetch_file;
use rocket::{
    config,
    http::{ContentType, Status},
    Config,
};
use std::{net::Ipv4Addr, str::FromStr};

#[macro_use]
extern crate rocket;

#[get("/<filename>")]
async fn index(filename: String) -> (Status, (ContentType, String)) {
    match fetch_file(filename).await {
        Some((contents, mime)) => (
            Status::Ok,
            (
                ContentType::from_str(mime.to_string().as_str()).unwrap_or(ContentType::Text),
                contents,
            ),
        ),
        None => (
            Status::NotFound,
            (ContentType::Text, String::from("File not found!")),
        ),
    }
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
