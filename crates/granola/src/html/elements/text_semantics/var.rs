use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<var>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/var)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let var = HtmlVar::new().id("variable");
///
/// assert_eq!(var.bake(), r#"<var id="variable"></var>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let var = HtmlVar::new().content("a");
///
/// let triangle = bake!["An equilateral triangle with side ", var];
///
/// assert_eq!(
///     triangle,
///     r#"An equilateral triangle with side <var>a</var>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <var
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</var>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = VarRecipe, content = Bake)]
pub struct HtmlVar<R: VarRecipe = ()> {
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

/// Shorthand for `HtmlVar`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let var = var!().id("variable");
///
/// assert_eq!(var.bake(), r#"<var id="variable"></var>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let var = var!("a");
///
/// let triangle = bake!["An equilateral triangle with side ", var];
///
/// assert_eq!(
///     triangle,
///     r#"An equilateral triangle with side <var>a</var>"#
/// );
/// ```
#[macro_export]
macro_rules! var {
    () => {
        $crate::html::HtmlVar::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlVar::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlVar::new().content($crate::bake![$first $(, $rest)*])
    };

}
