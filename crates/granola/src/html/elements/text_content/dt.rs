use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<dt>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dt)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let dt = HtmlDt::new().id("description_term");
///
/// assert_eq!(dt.bake(), r#"<dt id="description_term"></dt>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let dt = HtmlDt::new().content("Pålegg");
/// let dd = HtmlDd::new().content("Anything and everything you might put on a slice of bread.");
///
/// let term = bake_block![dt, dd];
///
/// assert_eq!(
///     term,
///     r#"<dt>Pålegg</dt>
/// <dd>Anything and everything you might put on a slice of bread.</dd>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <dt
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</dt>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DtRecipe, content = Cow<'static, str>)]
pub struct HtmlDt<R: DtRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// listitem
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlDt`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dt = dt!().id("description_term");
///
/// assert_eq!(dt.bake(), r#"<dt id="description_term"></dt>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dt = dt!("Pålegg");
/// let dd = dd!("Anything and everything you might put on a slice of bread.");
///
/// let term = bake_block![dt, dd];
///
/// assert_eq!(
///     term,
///     r#"<dt>Pålegg</dt>
/// <dd>Anything and everything you might put on a slice of bread.</dd>"#
/// );
/// ```
#[macro_export]
macro_rules! dt {
    () => {
        $crate::html::HtmlDt::new()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDt::new().content($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDt::new().content($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDt::new().content($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDt::new().content($crate::bake_inline![$($content),+])
    };
}
