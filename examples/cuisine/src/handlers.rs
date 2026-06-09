use axum::{
    Form,
    body::Body,
    extract::State,
    http::header,
    response::{Html, Redirect},
};
use http::Response;
use serde::Deserialize;

use crate::{SharedState, css::Stylesheet, html::home_page, utils::Palette};

#[derive(strum::IntoStaticStr, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Reset {
    AndyBell,
    JoshWComeau,
    ModernNormalize,
    Preflight,
    #[serde(other)]
    None,
}

impl Reset {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::AndyBell => "Andy Bell",
            Self::JoshWComeau => "Josh W. Comeau",
            Self::ModernNormalize => "modern-normalize",
            Self::Preflight => "Preflight",
        }
    }

    pub fn value(self) -> &'static str {
        self.into()
    }

    pub fn stylesheet(self) -> Option<Stylesheet> {
        match self {
            Self::None => None,
            Self::AndyBell => Some(Stylesheet::AndyBell),
            Self::JoshWComeau => Some(Stylesheet::JoshWComeau),
            Self::ModernNormalize => Some(Stylesheet::ModernNormalize),
            Self::Preflight => Some(Stylesheet::Preflight),
        }
    }
}

#[derive(Deserialize)]
pub struct PaletteForm {
    bg_color: String,
}

#[derive(Deserialize)]
pub struct ResetForm {
    reset: Reset,
}

pub async fn home_handler(State(state): State<SharedState>) -> Html<String> {
    let (palette, reset) = {
        let state = state.read().unwrap();
        (state.palette.clone(), state.reset)
    };

    Html(home_page(palette, reset).bake())
}

pub async fn palette_handler(
    State(state): State<SharedState>,
    Form(user_input): Form<PaletteForm>,
) -> Redirect {
    if let Some(palette) = Palette::from_hex(user_input.bg_color.trim()) {
        state.write().unwrap().palette = palette;
    }

    Redirect::to("/")
}

pub async fn reset_handler(
    State(state): State<SharedState>,
    Form(form): Form<ResetForm>,
) -> Redirect {
    state.write().unwrap().reset = form.reset;

    Redirect::to("/")
}

pub async fn stylesheet_handler(stylesheet: &'static str) -> Response<Body> {
    let body = Body::from(stylesheet);
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
