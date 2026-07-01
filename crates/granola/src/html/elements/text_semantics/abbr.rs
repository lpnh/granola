use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<abbr>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/abbr)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let abbr = HtmlAbbr::new().id("abbreviation");
///
/// assert_eq!(abbr.bake(), r#"<abbr id="abbreviation"></abbr>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let abbr = HtmlAbbr::new()
///     .content("TMNT")
///     .title("Teenage Mutant Ninja Turtles");
///
/// assert_eq!(
///     abbr.bake(),
///     r#"<abbr title="Teenage Mutant Ninja Turtles">TMNT</abbr>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <abbr
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</abbr>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = AbbrRecipe, content = Bake)]
pub struct HtmlAbbr<R: AbbrRecipe = ()> {
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

/// Shorthand for `HtmlAbbr`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let abbr = abbr!().id("abbreviation");
///
/// assert_eq!(abbr.bake(), r#"<abbr id="abbreviation"></abbr>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let abbr = abbr!("TMNT").title("Teenage Mutant Ninja Turtles");
///
/// assert_eq!(
///     abbr.bake(),
///     r#"<abbr title="Teenage Mutant Ninja Turtles">TMNT</abbr>"#
/// );
/// ```
#[macro_export]
macro_rules! abbr {
    () => {
        $crate::html::HtmlAbbr::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlAbbr::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlAbbr::new().content($crate::bake![$first $(, $rest)*])
    };

}
