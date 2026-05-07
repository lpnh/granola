use axum::{
    Form,
    extract::State,
    response::{Html, Redirect},
};
use serde::Deserialize;
use std::sync::Arc;

use granola::{
    macros::*,
    prelude::*,
    recipes::{Homemade, Post, Stylesheet},
    templates::TmplBase,
};

use crate::Palette;

#[derive(Deserialize)]
pub struct PaletteForm {
    bg_color: String,
}

pub async fn input_handler(
    State(shared_palette): State<Arc<Palette>>,
    Form(user_input): Form<PaletteForm>,
) -> Redirect {
    let mut color = shared_palette.background.write().unwrap();
    *color = user_input.bg_color;

    Redirect::to("/")
}

pub async fn home(State(shared_palette): State<Arc<Palette>>) -> Html<String> {
    let color = shared_palette.background.read().unwrap().to_string();

    let title = title!("palette example");
    let link = link!(@recipe Stylesheet; @from_href "/static/style.css");
    let style = style!(format!(r#":root {{ --palette-bg: {color} }}"#));

    let body = body!(palette_div(&color));

    let tmpl: TmplBase<Homemade> = TmplBase::new(body)
        .lang("en")
        .push_title(title)
        .push_link(link)
        .push_style(style);

    Html(tmpl.bake())
}

fn palette_div(color: &str) -> HtmlDiv {
    let h1 = h1!("Palette Form");

    let input = input!(@from_type InputType::Color)
        .name("bg_color")
        .value(color.to_string());
    let p = p!(@inline "Current color: ", code!(color.to_string()));
    let button = button!("Update Palette");

    let form = form!(@recipe Post; input, p, button).action("/form_endpoint");

    div!(h1, form).class("palette")
}
