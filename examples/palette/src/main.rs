mod handlers;
use handlers::{home, input_handler};

use axum::{
    Router,
    routing::{get, post},
};
use std::sync::{Arc, RwLock};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

struct Palette {
    background: RwLock<String>,
}

#[tokio::main]
async fn main() {
    let palette = Palette {
        background: RwLock::new("#f3eaaf".to_string()),
    };
    let shared_palette = Arc::new(palette);

    let static_service = ServiceBuilder::new().service(ServeDir::new("examples/palette/static"));

    let app = Router::new()
        .route("/", get(home))
        .route("/form_endpoint", post(input_handler))
        .with_state(shared_palette)
        .nest_service("/static", static_service);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    if let Ok(addr) = listener.local_addr() {
        println!("listening on http://{addr}/");
    }

    axum::serve(listener, app).await.unwrap()
}
