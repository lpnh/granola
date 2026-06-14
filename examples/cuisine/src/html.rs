use granola::{homemade::Homemade, macros::*, prelude::*, recipes::MethodPost};

use crate::{css::Stylesheet, handlers::Reset, utils::Palette};

pub fn home_page(palette: Palette, reset: Reset) -> HtmlDocument<Homemade> {
    let body = body!(reset_div(reset), palette_div(&palette));

    let title = title!("cuisine example");

    let mut page = HtmlDocument::from(Homemade)
        .lang("en")
        .push_title(title)
        .body(body);

    if let Some(stylesheet) = reset.stylesheet() {
        page = page.push_link(stylesheet.link());
    }

    let css_rule = rule!(
        @selectors ":root";
        @declarations
        ("--base-100", palette.base_100),
        ("--base-200", palette.base_200),
        ("--base-300", palette.base_300),
        ("--base-content", palette.base_content),
    );

    let style = style!(css_rule);

    page.push_link(Stylesheet::Cuisine.link()).push_style(style)
}

fn reset_div(current: Reset) -> HtmlDiv {
    let h2 = h2!("Reset");

    let select = select!(
        reset_option(current, Reset::None),
        reset_option(current, Reset::AndyBell),
        reset_option(current, Reset::JoshWComeau),
        reset_option(current, Reset::ModernNormalize),
        reset_option(current, Reset::Preflight),
    )
    .name("reset");
    let button = button!("Apply");
    let form = form!(@cookbook MethodPost; select, button).action("/reset_endpoint");

    div!(h2, form).class("palette")
}

fn reset_option(current: Reset, reset: Reset) -> HtmlOption {
    option!(reset.label())
        .value(reset.value())
        .selected(current == reset)
}

fn palette_div(palette: &Palette) -> HtmlDiv {
    let h2 = h2!("Palette");

    let swatches = div!(
        swatch_div("base-100", &palette.base_100),
        swatch_div("base-200", &palette.base_200),
        swatch_div("base-300", &palette.base_300),
        swatch_div("base-content", &palette.base_content),
    )
    .class("swatches");

    let input = input!(@type InputType::Color)
        .name("bg_color")
        .value(palette.source.clone());
    let button = button!("Update");
    let form = form!(@cookbook MethodPost; input, button).action("/palette_endpoint");

    div!(h2, swatches, form).class("palette")
}

fn swatch_div(name: &str, value: &str) -> HtmlDiv {
    let square = div!()
        .class("square")
        .style(format!("background: var(--{name});"));
    let name = p!(name.to_string());
    let val = p!(code!(value.to_string()));

    div!(square, name, val).class("swatch")
}
