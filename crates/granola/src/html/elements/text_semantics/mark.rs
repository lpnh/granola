use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<mark>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/mark)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let mark = HtmlMark::new().id("mark_text");
///
/// assert_eq!(mark.bake(), r#"<mark id="mark_text"></mark>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let but_the_clouds = HtmlMark::new().content("but the clouds");
///
/// let br = HtmlBr::new();
///
/// let the_tower = bake![
///     bake_ws!["Seem", but_the_clouds, "of the sky"],
///     br,
///     "When the horizon fades;",
///     br,
///     "Or a bird's sleepy cry",
///     br,
///     "Among the deepening shades."
/// ];
///
/// assert_eq!(
///     the_tower,
///     "\
/// Seem <mark>but the clouds</mark> of the sky<br />\
/// When the horizon fades;<br />\
/// Or a bird's sleepy cry<br />\
/// Among the deepening shades.\
/// "
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <mark
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</mark>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = MarkRecipe, content = Bake)]
pub struct HtmlMark<R: MarkRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlMark`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let mark = mark!().id("mark_text");
///
/// assert_eq!(mark.bake(), r#"<mark id="mark_text"></mark>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let but_the_clouds = mark!("but the clouds");
///
/// let br = br!();
///
/// let the_tower = bake![
///     bake_ws!["Seem", but_the_clouds, "of the sky"],
///     br,
///     "When the horizon fades;",
///     br,
///     "Or a bird's sleepy cry",
///     br,
///     "Among the deepening shades."
/// ];
///
/// assert_eq!(
///     the_tower,
///     "\
/// Seem <mark>but the clouds</mark> of the sky<br />\
/// When the horizon fades;<br />\
/// Or a bird's sleepy cry<br />\
/// Among the deepening shades.\
/// "
/// );
/// ```
#[macro_export]
macro_rules! mark {
    () => {
        $crate::html::HtmlMark::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlMark::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlMark::new().content($crate::bake![$first $(, $rest)*])
    };

}
