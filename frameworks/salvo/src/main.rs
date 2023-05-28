use file_fetch::fetch_file;
use salvo::{
    prelude::{handler, Router, Server, StatusCode, TcpListener},
    writer::Text,
    Listener, Request, Response,
};

#[handler]
async fn index(req: &mut Request, res: &mut Response) {
    let path = req.param::<String>("filename").unwrap();
    match fetch_file(path).await {
        Some((contents, mime)) => {
            if mime == mime::TEXT_PLAIN {
                Text::Plain(contents.to_string());
            } else if mime == mime::APPLICATION_JAVASCRIPT {
                Text::Json(contents.to_string());
            } else if mime == mime::TEXT_XML {
                Text::Xml(contents.to_string());
            } else if mime == mime::TEXT_HTML {
                Text::Html(contents.to_string());
            } else if mime == mime::TEXT_JAVASCRIPT {
                Text::Js(contents.to_string());
            } else if mime == mime::TEXT_CSS {
                Text::Css(contents.to_string());
            } else if mime == mime::TEXT_CSV {
                Text::Csv(contents.to_string());
            } else {
                Text::Plain(contents.to_string());
            };

            res.render(contents);
        }
        None => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Text::Html("File not found!"));
        }
    };
}

#[tokio::main]
async fn main() {
    let router = Router::with_path("<filename>").get(index);

    Server::new(TcpListener::new("127.0.0.1:9852").bind().await)
        .serve(router)
        .await;
}
