use crate::prelude::*;

/// The daisyUI `btn` component.
///
/// [daisyUI Documentation](https://daisyui.com/components/button/)
///
/// # Example
///
/// ```rust
/// use granola::{daisyui::btn, prelude::*};
///
/// let button = HtmlButton::from(btn::Btn)
///     .content("Continue")
///     .color(btn::Color::Primary)
///     .size(btn::Size::Lg)
///     .style(btn::Style::Outline)
///     .modifier(btn::Modifier::Wide);
///
/// assert_eq!(
///     button.bake(),
///     r#"<button class="btn btn-primary btn-lg btn-outline btn-wide">Continue</button>"#
/// );
/// ```
///
/// ```rust
/// use granola::{daisyui::btn, prelude::*};
///
/// let a = HtmlA::from(btn::Btn);
/// let input = HtmlInput::from(btn::Btn);
/// let div = HtmlDiv::from(btn::Btn);
/// let span = HtmlSpan::from(btn::Btn);
/// let label = HtmlLabel::from(btn::Btn);
///
/// assert_eq!(a.bake(), r#"<a class="btn"></a>"#);
/// assert_eq!(input.bake(), r#"<input class="btn" />"#);
/// assert_eq!(div.bake(), r#"<div class="btn" role="button"></div>"#);
/// assert_eq!(span.bake(), r#"<span class="btn"></span>"#);
/// assert_eq!(label.bake(), r#"<label class="btn"></label>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Btn;

daisyui_component! {
    Btn {
        module: "btn";
        macro: "btn";
        base: "btn";
        parts: [];
        shared: [Color, Size];
        Style {
            Outline => "btn-outline",
            Dash => "btn-dash",
            Soft => "btn-soft",
            Ghost => "btn-ghost",
            Link => "btn-link",
        }
        Modifier {
            Wide => "btn-wide",
            Block => "btn-block",
            Square => "btn-square",
            Circle => "btn-circle",
            Active => "btn-active",
            Disabled => "btn-disabled",
        }
    }
}

impl ButtonRecipe for Btn {
    recipe_boilerplate!(ButtonRecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class(BASE_CLASS)
    }
}

impl ARecipe for Btn {
    recipe_boilerplate!(ARecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class(BASE_CLASS)
    }
}

impl InputRecipe for Btn {
    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class(BASE_CLASS)
    }
}

impl DivRecipe for Btn {
    recipe_boilerplate!(DivRecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class(BASE_CLASS).role("button")
    }
}

impl SpanRecipe for Btn {
    recipe_boilerplate!(SpanRecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class(BASE_CLASS)
    }
}

impl LabelRecipe for Btn {
    recipe_boilerplate!(LabelRecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class(BASE_CLASS)
    }
}

/// Shorthand for `HtmlButton<Btn>`.
///
/// ```rust
/// use granola::{daisyui::btn, macros::*, prelude::*};
///
/// let button = btn!("Continue").color(btn::Color::Primary);
///
/// assert_eq!(
///     button.bake(),
///     r#"<button class="btn btn-primary">Continue</button>"#
/// );
/// ```
#[macro_export]
macro_rules! btn {
    () => {
        $crate::html::HtmlButton::from($crate::daisyui::Btn)
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlButton::from($crate::daisyui::Btn).content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlButton::from($crate::daisyui::Btn)
            .content($crate::bake![$first $(, $rest)*])
    };
}
