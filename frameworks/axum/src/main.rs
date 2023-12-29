use axum::{
    extract::Path as AxumPath,
    http::{
        header::{HeaderName, HeaderValue},
        StatusCode,
    },
    response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
    routing::get,
    Router,
};
use file_fetch::fetch_file;

// Hypothetical helper type for setting a single header
struct SetHeader(String, String);
impl IntoResponseParts for SetHeader {
    type Error = (StatusCode, String);

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        match (self.0.parse::<HeaderName>(), self.1.parse::<HeaderValue>()) {
            (Ok(name), Ok(value)) => {
                res.headers_mut().insert(name, value);
            }
            (Err(_), _) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header name {}", self.0),
                ));
            }
            (_, Err(_)) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header value {}", self.1),
                ));
            }
        }

        Ok(res)
    }
}
// Its also recommended to implement `IntoResponse` so `SetHeader` can be used on its own as
// the response
impl IntoResponse for SetHeader {
    fn into_response(self) -> Response {
        // This gives an empty response with the header
        (self, ()).into_response()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:filename", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9852")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler(AxumPath(path): AxumPath<String>) -> (StatusCode, SetHeader, String) {
    match fetch_file(path).await {
        Some((contents, mime)) => (
            StatusCode::OK,
            SetHeader(String::from("Content-Type"), mime.to_string()),
            contents,
        ),
        None => (
            StatusCode::NOT_FOUND,
            SetHeader(String::from("Content-Type"), String::from("text/plain")),
            String::from("File not found!"),
        ),
    }
}
