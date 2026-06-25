use askama::Template;
use granola::{homemade::*, macros::*, prelude::*};

use crate::{css::Stylesheet, handlers::Reset, snippets::snippets, utils::Palette};

pub fn home_page(palette: Palette) -> HtmlDocument<Homemade> {
    let menu = nav![
        h2!("What's On the Menu"),
        p!(
            a!("Palette").href("/palette"),
            " - generate a color palette",
        ),
        p!(
            a!("Resets").href("/reset"),
            " - compare CSS reset stylesheets",
        ),
    ];

    page("cuisine example", header!(h1!("cuisine")), main!(menu))
        .push_link(Stylesheet::Cuisine.link())
        .push_style(palette_style(palette))
}

pub fn palette_page(palette: Palette) -> HtmlDocument<Homemade> {
    let swatches = div!(
        swatch_div("color-background", &palette.color_background),
        swatch_div("color-surface", &palette.color_surface),
        swatch_div("color-border", &palette.color_border),
        swatch_div("color-text", &palette.color_text),
    )
    .class("swatches");

    example_page(
        "palette - cuisine example",
        "Palette generator",
        palette_picker(&palette.source),
        swatches,
    )
    .push_link(Stylesheet::Cuisine.link())
    .push_style(palette_style(palette))
}

pub fn reset_page(reset: Reset) -> HtmlDocument<Homemade> {
    let mut page = example_page(
        "resets - cuisine example",
        "CSS resets",
        reset_picker(reset),
        snippets(),
    );

    if let Some(stylesheet) = reset.stylesheet() {
        page = page.push_link(stylesheet.link());
    }

    page
}

fn page(title: &'static str, header: HtmlHeader, main: HtmlMain) -> HtmlDocument<Homemade> {
    HtmlDocument::from(Homemade)
        .lang("en")
        .push_title(title!(title))
        .body(body!(header, main))
}

fn example_page(
    title: &'static str,
    heading: &'static str,
    picker: HtmlForm,
    content: impl Template,
) -> HtmlDocument<Homemade> {
    page(
        title,
        header!(back_link(), h1!(heading)),
        main!(picker, hr!(), content),
    )
}

fn palette_style(palette: Palette) -> HtmlStyle {
    let css_rule = rule!(
        ":root",
        declarations_block![
            CssCustomProperty::from(ColorBackground).value(palette.color_background),
            CssCustomProperty::from(ColorSurface).value(palette.color_surface),
            CssCustomProperty::from(ColorBorder).value(palette.color_border),
            CssCustomProperty::from(ColorText).value(palette.color_text),
        ]
    );

    style!(css_rule)
}

fn back_link() -> HtmlNav {
    nav!(a!("← demos").href("/"))
}

fn reset_picker(current: Reset) -> HtmlForm {
    let select = div![
        label!(
            "Select one reset: ",
            select!(
                reset_option(current, Reset::None),
                reset_option(current, Reset::AndyBell),
                reset_option(current, Reset::JoshWComeau),
                reset_option(current, Reset::ModernNormalize),
                reset_option(current, Reset::Preflight),
            )
            .id("select-reset")
            .name("reset"),
            button!("Apply").button_type(ButtonType::Submit),
        )
        .for_id("select-reset"),
    ];

    form!(select)
        .method(FormMethod::Post)
        .action("/reset_endpoint")
        .aria_label("Stylesheet reset")
}

fn reset_option(current: Reset, reset: Reset) -> HtmlOption {
    option!(reset.label())
        .value(reset.value())
        .selected(current == reset)
}

fn palette_picker(palette_source: &str) -> HtmlForm {
    let color_input = label!(
        "Select a color: ",
        input!()
            .id("select-palette")
            .input_type(InputType::Color)
            .name("bg_color")
            .value(palette_source.to_string()),
    );

    form!(color_input, button!("Update"))
        .method(FormMethod::Post)
        .action("/palette_endpoint")
}

fn swatch_div(name: &str, value: &str) -> HtmlDiv {
    let square = div!()
        .class("square")
        .style(format!("background: var(--{name});"));
    let name = p!(name.to_string());
    let val = p!(code!(value.to_string()));

    div!(square, name, val).class("swatch")
}
