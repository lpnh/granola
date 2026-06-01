use granola::{
    cookbook::{MethodPost, RelStylesheet},
    homemade::Homemade,
    macros::*,
    prelude::*,
    template::TmplBase,
};

use crate::css::STYLESHEET_URL;

pub fn home_page(
    base_100: String,
    base_200: String,
    base_300: String,
    base_content: String,
    palette_source: String,
) -> TmplBase<Homemade> {
    let body_content = palette_div(
        &base_100,
        &base_200,
        &base_300,
        &base_content,
        palette_source,
    );

    let body = body!(body_content);

    let title = title!("palette example");
    let stylesheet_link = link!(@recipe RelStylesheet; @from_href STYLESHEET_URL.as_str());
    let css_rule = rule!(
        ":root";
        ("--base-100", base_100),
        ("--base-200", base_200),
        ("--base-300", base_300),
        ("--base-content", base_content),
    );
    let style = style!(css_rule);

    TmplBase::<Homemade>::new(body)
        .lang("en")
        .push_title(title)
        .push_link(stylesheet_link)
        .push_style(style)
}

fn palette_div(
    base_100: &str,
    base_200: &str,
    base_300: &str,
    base_content: &str,
    palette_source: String,
) -> HtmlDiv {
    let h1 = h1!("Palette");

    let swatches = div!(
        swatch_div("base-100", base_100),
        swatch_div("base-200", base_200),
        swatch_div("base-300", base_300),
        swatch_div("base-content", base_content),
    )
    .class("swatches");

    let input = input!(@from_type InputType::Color)
        .name("bg_color")
        .value(palette_source);
    let button = button!("Update");
    let form = form!(@recipe MethodPost; input, button).action("/form_endpoint");

    div!(h1, swatches, form).class("palette")
}

fn swatch_div(name: &str, value: &str) -> HtmlDiv {
    let square = div!()
        .class("square")
        .style(format!("background: var(--{name});"));
    let name = p!(name.to_string());
    let val = p!(code!(value.to_string()));

    div!(square, name, val).class("swatch")
}
