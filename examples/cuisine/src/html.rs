use granola::{daisyui::btn, homemade::*, macros::*, prelude::*};

use crate::{css::Stylesheet, handlers::Reset, snippets::snippets, utils::Palette};

pub fn home_page(palette: Palette) -> HtmlDocument {
    let menu = nav![
        h2!("What's On the Menu"),
        p!(
            link!("Palette").href("/palette"),
            " - generate a color palette",
        ),
        p!(
            link!("Resets").href("/reset"),
            " - compare CSS reset stylesheets",
        ),
    ];

    document(
        page(
            "cuisine example",
            body!(header!(h1!("cuisine")), main_card(menu)),
        )
        .push_link(Stylesheet::Cuisine.link())
        .push_style(palette_style(palette)),
    )
}

pub fn palette_page(palette: Palette) -> HtmlDocument {
    let swatches = div!(
        swatch_div("base-100", &palette.base_100),
        swatch_div("base-200", &palette.base_200),
        swatch_div("base-300", &palette.base_300),
        swatch_div("base-content", &palette.base_content),
    )
    .class("flex flex-wrap justify-center gap-4 p-4 sm:p-8");

    document(
        example_page(
            "palette - cuisine example",
            "Palette generator",
            palette_picker(&palette.source),
            swatches,
        )
        .push_link(Stylesheet::Cuisine.link())
        .push_style(palette_style(palette)),
    )
}

pub fn reset_page(reset: Reset) -> HtmlDocument {
    let mut content = example_page(
        "resets - cuisine example",
        "CSS resets",
        reset_picker(reset),
        snippets(),
    );

    if let Some(stylesheet) = reset.stylesheet() {
        content = content.push_link(stylesheet.link());
    }

    document(content)
}

fn page(title: &'static str, body: HtmlBody) -> HomemadeRootContent {
    HomemadeRootContent::new()
        .push_title(title!(title))
        .body(body.class("flex flex-col items-center gap-8"))
}

fn document(content: HomemadeRootContent) -> HtmlDocument {
    HtmlDocument::new().content(HtmlRoot::new().lang("en").content(content))
}

fn main_card(content: impl Into<Bake>) -> HtmlMain {
    main!(content).class("bg-base-200 border border-base-300 rounded-box p-4 sm:p-8 text-center")
}

fn example_page(
    title: &'static str,
    heading: &'static str,
    picker: HtmlForm,
    content: impl Into<Bake>,
) -> HomemadeRootContent {
    page(
        title,
        body!(
            header!(back_link(), h1!(heading)),
            picker,
            main_card(content)
        ),
    )
}

/// Overrides the base colors of the active daisyUI theme.
///
/// daisyUI declares its themes inside `@layer base`. This rule is unlayered, so
/// it outranks every theme block regardless of selector specificity, including
/// the built-in dark theme behind `prefers-color-scheme`.
fn palette_style(palette: Palette) -> HtmlStyle {
    let css_rule = rule!(
        ":root",
        declarations_block![
            ("color-scheme", palette.color_scheme()),
            CssDeclaration::from((base_color("100"), palette.base_100)),
            CssDeclaration::from((base_color("200"), palette.base_200)),
            CssDeclaration::from((base_color("300"), palette.base_300)),
            CssDeclaration::from((base_color("content"), palette.base_content)),
        ]
    );

    style!(css_rule)
}

fn base_color(shade: &str) -> CssCustomProperty {
    CssCustomProperty::new().name(format!("color-base-{shade}"))
}

fn back_link() -> HtmlNav {
    nav!(link!("← demos").href("/"))
}

fn reset_picker(current: Reset) -> HtmlForm {
    form!(
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
        )
        .for_id("select-reset"),
        button!("Apply").button_type(ButtonType::Submit),
    )
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
    form!(
        label!(
            "Select a color: ",
            input!()
                .id("select-palette")
                .class("input bg-base-content")
                .input_type(InputType::Color)
                .name("bg_color")
                .value(palette_source.to_string()),
        )
        .for_id("select-palette"),
        HtmlButton::from(btn::Btn).content("Update"),
    )
    .class("grid gap-4")
    .method(FormMethod::Post)
    .action("/palette_endpoint")
}

fn swatch_div(name: &str, value: &str) -> HtmlDiv {
    let square = div!()
        .class("size-16 rounded-field shadow-sm")
        .css_style(format!("background: var(--color-{name});"));
    let name = p!(name.to_string()).class("text-xs");
    let val = code!(value.to_string());

    div!(square, name, val).class("flex flex-col items-center gap-1")
}
