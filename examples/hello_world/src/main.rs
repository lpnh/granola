use axum::{Router, response::Html, routing::get};

use granola::prelude::*;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    if let Ok(addr) = listener.local_addr() {
        println!("listening on http://{addr}/");
    }

    axum::serve(listener, app).await.unwrap()
}

async fn handler() -> Html<String> {
    let doctype = HtmlDoctype::new();

    let heading: HtmlH1 = HtmlH1::new("Hello, world!");
    let body: HtmlBody = HtmlBody::new(heading);
    let root: HtmlRoot = HtmlRoot::new(body);

    let home = bake_block![doctype, root];

    Html(home)
}
