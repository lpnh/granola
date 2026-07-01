use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<bdo>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdo)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdo = HtmlBdo::new().id("bidirectional_text_override");
///
/// assert_eq!(
///     bdo.bake(),
///     r#"<bdo id="bidirectional_text_override"></bdo>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdo = HtmlBdo::new().content("looking-glass").dir("rtl");
///
/// assert_eq!(bdo.bake(), r#"<bdo dir="rtl">looking-glass</bdo>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <bdo
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</bdo>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BdoRecipe, content = Cow<'static, str>)]
pub struct HtmlBdo<R: BdoRecipe = ()> {
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

/// Shorthand for `HtmlBdo`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let bdo = bdo!().id("bidirectional_text_override");
///
/// assert_eq!(
///     bdo.bake(),
///     r#"<bdo id="bidirectional_text_override"></bdo>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let bdo = bdo!("looking-glass").dir("rtl");
///
/// assert_eq!(bdo.bake(), r#"<bdo dir="rtl">looking-glass</bdo>"#);
/// ```
#[macro_export]
macro_rules! bdo {
    () => {
        $crate::html::HtmlBdo::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlBdo::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlBdo::new().content($crate::bake![$first $(, $rest)*])
    };

}
