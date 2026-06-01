mod css;
use css::STYLESHEET_URL;
mod handlers;
mod html;
use handlers::{home_handler, input_handler, stylesheet_handler};
mod utils;
use utils::Palette;

use axum::{
    Router,
    routing::{get, post},
};
use std::sync::{Arc, RwLock};
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(home_router()).merge(static_router());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    if let Ok(addr) = listener.local_addr() {
        println!("listening on http://{addr}/");
    }

    axum::serve(listener, app).await.unwrap()
}

fn home_router() -> Router {
    let palette = Palette::from_hex("#f3eaaf");
    let shared_palette = Arc::new(RwLock::new(palette));

    Router::new()
        .route("/", get(home_handler))
        .route("/form_endpoint", post(input_handler))
        .with_state(shared_palette)
}

pub fn static_router() -> Router {
    Router::new()
        .route(STYLESHEET_URL.as_str(), get(stylesheet_handler))
        // Compresses the response based on the client's `Accept-Encoding` request header and adds
        // the HTTP `Content-Encoding` header.
        //
        // <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Accept-Encoding>
        // <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Encoding>
        .layer(CompressionLayer::new())
}
