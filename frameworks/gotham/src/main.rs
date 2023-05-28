use file_fetch::fetch_file;
use futures_util::future::FutureExt;
use gotham::{
    handler::HandlerFuture,
    helpers::http::response,
    hyper::StatusCode,
    prelude::{DefineSingleRoute, DrawRoutes, FromState},
    router::{builder, response::StaticResponseExtender},
    state::{State, StateData},
};
use serde::Deserialize;
use std::pin::Pin;

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct IndexPathParams {
    filename: String,
}

fn index(mut state: State) -> Pin<Box<HandlerFuture>> {
    let IndexPathParams { filename } = IndexPathParams::take_from(&mut state);
    async move {
        let (body, mime, status) = match fetch_file(filename).await {
            Some((contents, mime)) => (contents, mime, StatusCode::OK),
            None => (
                String::from("File not found!"),
                mime::TEXT_PLAIN,
                StatusCode::NOT_FOUND,
            ),
        };
        let response = response::create_response(&state, status, mime, body);

        Ok((state, response))
    }
    .boxed()
}

fn main() {
    gotham::start(
        "127.0.0.1:9852",
        builder::build_simple_router(|route| {
            route
                .get("/:filename")
                .with_path_extractor::<IndexPathParams>()
                .to(index);
        }),
    )
    .unwrap()
}
