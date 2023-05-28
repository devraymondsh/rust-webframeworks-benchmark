use file_fetch::fetch_file;
use hyper::{
    header::{HeaderName, HeaderValue},
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{convert::Infallible, str::FromStr};

async fn index(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri_mut().path().replace("/", "");
    let (body, mime) = match fetch_file(path).await {
        Some((contents, mime)) => (contents, mime.to_string()),
        None => (String::from("File not found!"), String::from("text/html")),
    };
    let mut res = Response::new(Body::from(body));

    res.headers_mut().append(
        HeaderName::from_str("Content-Type").unwrap(),
        HeaderValue::from_str(mime.as_str()).unwrap(),
    );

    Ok(res)
}

#[tokio::main]
pub async fn main() {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(index)) });

    Server::bind(&([127, 0, 0, 1], 9852).into())
        .serve(make_svc)
        .await
        .unwrap()
}
