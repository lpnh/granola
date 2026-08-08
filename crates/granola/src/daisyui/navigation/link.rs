use crate::prelude::*;

/// The daisyUI `link` component.
///
/// [daisyUI Documentation](https://daisyui.com/components/link/)
///
/// # Example
///
/// ```rust
/// use granola::{daisyui::link, prelude::*};
///
/// let link = HtmlA::from(link::Link)
///     .content("Read the docs")
///     .href("https://askama.rs")
///     .color(link::Color::Primary)
///     .modifier(link::Modifier::Hover);
///
/// assert_eq!(
///     link.bake(),
///     r#"<a class="link link-primary link-hover" href="https://askama.rs">Read the docs</a>"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Link;

daisyui_component! {
    Link {
        base: "link";
        shared: [Color];
        Modifier {
            Hover => "hover",
        }
    }
}

impl ARecipe for Link {
    recipe_boilerplate!(ARecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class(BASE_CLASS)
    }
}

/// Shorthand for `HtmlA<Link>`.
///
/// ```rust
/// use granola::{daisyui::link, macros::*, prelude::*};
///
/// let link = link!("Read more")
///     .href("https://example.com")
///     .color(link::Color::Primary);
///
/// assert_eq!(
///     link.bake(),
///     r#"<a class="link link-primary" href="https://example.com">Read more</a>"#
/// );
/// ```
#[macro_export]
macro_rules! link {
    () => {
        $crate::html::HtmlA::from($crate::daisyui::Link)
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlA::from($crate::daisyui::Link).content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlA::from($crate::daisyui::Link)
            .content($crate::bake![$first $(, $rest)*])
    };
}
