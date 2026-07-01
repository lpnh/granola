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
    let heading = HtmlH1::new().content("Hello, world!");
    let body = HtmlBody::new().content(heading);
    let root = HtmlRoot::new().content(body);

    let home_page = HtmlDocument::new().content(root);

    Html(home_page.bake())
}
