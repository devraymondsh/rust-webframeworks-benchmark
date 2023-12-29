use file_fetch::fetch_file;
use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    header::{HeaderName, HeaderValue},
    server::conn::http1,
    service::service_fn,
    Request, Response,
};
use hyper_util::rt::TokioIo;
use std::{convert::Infallible, net::SocketAddr, str::FromStr};
use tokio::net::TcpListener;

async fn index(mut req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = req.uri_mut().path().replace("/", "");
    let (body, mime) = match fetch_file(path).await {
        Some((contents, mime)) => (contents, mime.to_string()),
        None => (String::from("File not found!"), String::from("text/html")),
    };
    let mut res = Response::new(Full::new(Bytes::from(body)));

    res.headers_mut().append(
        HeaderName::from_str("Content-Type").unwrap(),
        HeaderValue::from_str(mime.as_str()).unwrap(),
    );

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 9852));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let _ = http1::Builder::new()
                .serve_connection(io, service_fn(index))
                .await;
        });
    }
}
