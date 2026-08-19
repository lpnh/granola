use axum::{
    Router,
    body::Body,
    http::{Response, header},
    response::Html,
    routing::get,
};
use fixtures::css::Stylesheet;

#[tokio::main]
async fn main() {
    let stylesheet = Stylesheet::OatsAndEnds;
    let app = Router::new().route("/", get(handler)).route(
        stylesheet.url(),
        get(move || stylesheet_handler(stylesheet.body())),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    if let Ok(addr) = listener.local_addr() {
        println!("listening on http://{addr}/");
    }

    axum::serve(listener, app).await.unwrap()
}

async fn handler() -> Html<String> {
    Html(fixtures::standard::page().bake())
}

async fn stylesheet_handler(stylesheet: &'static str) -> Response<Body> {
    let mut res = Response::new(Body::from(stylesheet));
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/css; charset=utf-8"),
    );
    res.headers_mut().insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("public, max-age=31536000, immutable"),
    );
    res
}
