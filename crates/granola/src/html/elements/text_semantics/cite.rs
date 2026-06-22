use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<cite>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/cite)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let cite = HtmlCite::new().id("citation");
///
/// assert_eq!(cite.bake(), r#"<cite id="citation"></cite>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let cite = HtmlCite::new().content("Act Without Words I");
///
/// assert_eq!(cite.bake(), r#"<cite>Act Without Words I</cite>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <cite
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</cite>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = CiteRecipe, content = Cow<'static, str>)]
pub struct HtmlCite<R: CiteRecipe = ()> {
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

/// Shorthand for `HtmlCite`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let cite = cite!().id("citation");
///
/// assert_eq!(cite.bake(), r#"<cite id="citation"></cite>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let cite = cite!("Act Without Words I");
///
/// assert_eq!(cite.bake(), r#"<cite>Act Without Words I</cite>"#);
/// ```
#[macro_export]
macro_rules! cite {
    () => {
        $crate::html::HtmlCite::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlCite::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlCite::new().content($crate::bake![$first $(, $rest)*])
    };

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlCite::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlCite::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlCite::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
