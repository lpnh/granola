use axum::{
    Form,
    extract::State,
    response::{Html, Redirect},
};
use serde::Deserialize;
use std::sync::{Arc, RwLock};

use granola::{
    macros::*,
    prelude::*,
    recipes::{Homemade, Post, Stylesheet},
    templates::TmplBase,
};

use crate::utils::{Palette, is_valid_hex};

#[derive(Deserialize)]
pub struct PaletteForm {
    bg_color: String,
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

pub async fn home(State(shared_palette): State<Arc<RwLock<Palette>>>) -> Html<String> {
    let palette = shared_palette.read().unwrap();

    let title = title!("palette example");
    let link = link!(@recipe Stylesheet; @from_href "/static/style.css");
    let style = style!(format!(
        ":root {{ --base-100: {}; --base-200: {}; --base-300: {}; --base-content: {} }}",
        palette.base_100, palette.base_200, palette.base_300, palette.base_content,
    ));

    let body = body!(palette_div(&palette));

    let tmpl: TmplBase<Homemade> = TmplBase::new(body)
        .lang("en")
        .push_title(title)
        .push_link(link)
        .push_style(style);

    Html(tmpl.bake())
}

fn swatch(name: &str, value: &str) -> HtmlDiv {
    let square = div!()
        .class("square")
        .style(format!("background: var(--{name});"));
    let name = p!(name.to_string());
    let val = p!(code!(value.to_string()));
    div!(square, name, val).class("swatch")
}

fn palette_div(palette: &Palette) -> HtmlDiv {
    let h1 = h1!("Palette");

    let swatches = div!(
        swatch("base-100", &palette.base_100),
        swatch("base-200", &palette.base_200),
        swatch("base-300", &palette.base_300),
        swatch("base-content", &palette.base_content),
    )
    .class("swatches");

    let input = input!(@from_type InputType::Color)
        .name("bg_color")
        .value(palette.source.clone());
    let button = button!("Update");
    let form = form!(@recipe Post; input, button).action("/form_endpoint");

    div!(h1, swatches, form).class("palette")
}
