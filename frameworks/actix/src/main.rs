use actix_web::{get, web, App, HttpResponse, HttpServer};
use file_fetch::fetch_file;

#[get("/{filename}")]
async fn index(path: web::Path<String>) -> HttpResponse {
    match fetch_file(path.to_string()).await {
        Some((contents, mime)) => HttpResponse::Ok()
            .insert_header(("Content-Type", mime.to_string()))
            .body(contents),
        None => HttpResponse::NotFound()
            .insert_header(("Content-Type", "text/plain"))
            .body("File not found!"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 9852))?
        .run()
        .await
}
