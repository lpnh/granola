mod css;
use css::Stylesheet;
mod handlers;
mod html;
use handlers::{Reset, home_handler, palette_handler, reset_handler, stylesheet_handler};
mod utils;
use utils::Palette;

use axum::{
    Router,
    routing::{get, post},
};
use std::sync::{Arc, RwLock};
use strum::IntoEnumIterator;
use tower_http::compression::CompressionLayer;

pub struct AppState {
    pub palette: Palette,
    pub reset: Reset,
}

pub type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() {
    Stylesheet::preload();

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/palette_endpoint", post(palette_handler))
        .route("/reset_endpoint", post(reset_handler))
        .with_state(shared_state())
        .merge(static_router());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    if let Ok(addr) = listener.local_addr() {
        println!("listening on http://{addr}/");
    }

    axum::serve(listener, app).await.unwrap()
}

pub fn shared_state() -> SharedState {
    let app_state = AppState {
        palette: Palette::from_hex("#f3eaaf").unwrap(),
        reset: Reset::None,
    };

    Arc::new(RwLock::new(app_state))
}

pub fn static_router() -> Router {
    Stylesheet::iter()
        .fold(Router::new(), |router, stylesheet| {
            router.route(
                stylesheet.url(),
                get(move || stylesheet_handler(stylesheet.body())),
            )
        })
        // Compresses the response based on the client's `Accept-Encoding` request header and
        // appends the `content-encoding` and `vary: accept-encoding` HTTP headers.
        //
        // <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Accept-Encoding>
        // <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Encoding>
        // <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Vary>
        .layer(CompressionLayer::new())
}
