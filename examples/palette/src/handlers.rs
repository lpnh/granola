use axum::{
    Form,
    body::Body,
    extract::State,
    http::header,
    response::{Html, Redirect},
};
use http::Response;
use serde::Deserialize;
use std::sync::{Arc, RwLock};

use crate::{
    css::STATIC_STYLESHEET,
    html::home_page,
    utils::{Palette, is_valid_hex},
};

#[derive(Deserialize)]
pub struct PaletteForm {
    bg_color: String,
}

pub async fn home_handler(State(shared_palette): State<Arc<RwLock<Palette>>>) -> Html<String> {
    let palette = shared_palette.read().unwrap().clone();

    let home_page = home_page(
        palette.base_100,
        palette.base_200,
        palette.base_300,
        palette.base_content,
        palette.source,
    );

    Html(home_page.bake())
}

pub async fn input_handler(
    State(shared_palette): State<Arc<RwLock<Palette>>>,
    Form(user_input): Form<PaletteForm>,
) -> Redirect {
    if is_valid_hex(&user_input.bg_color) {
        let mut palette = shared_palette.write().unwrap();
        *palette = Palette::from_hex(&user_input.bg_color);
    }

    Redirect::to("/")
}

pub async fn stylesheet_handler() -> Response<Body> {
    let body = Body::new(STATIC_STYLESHEET.to_string());
    let mut res = Response::new(body);

    // The HTTP `Content-Type: text/css; charset=utf-8` header.
    //
    // <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Type#serving_assets_with_correct_content_type>
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static(mime::TEXT_CSS_UTF_8.as_ref()),
    );

    // The HTTP `Cache-Control: public, max-age=31536000, immutable` header.
    //
    // <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cache-Control>
    res.headers_mut().insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("public, max-age=31536000, immutable"),
    );

    res
}
