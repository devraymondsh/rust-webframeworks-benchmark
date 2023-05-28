use file_fetch::fetch_file;
use std::convert::Infallible;
use warp::{http::Response, Filter};

pub async fn index(filename: String) -> Result<Response<String>, Infallible> {
    match fetch_file(filename).await {
        Some((contents, mime)) => Ok(Response::builder()
            .header("Content-Type", mime.to_string())
            .body(String::from(contents))
            .unwrap()),
        None => Ok(Response::builder()
            .status(404)
            .header("Content-Type", "text/plain")
            .body(String::from("File not found!"))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() {
    let routes = warp::path!(String).and_then(index);

    warp::serve(routes).run(([127, 0, 0, 1], 9852)).await;
}
