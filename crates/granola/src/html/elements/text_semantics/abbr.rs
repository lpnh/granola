use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

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
/// let abbr: HtmlAbbr = HtmlAbbr::empty().id("abbreviation");
///
/// assert_eq!(abbr.bake(), r#"<abbr id="abbreviation"></abbr>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let abbr: HtmlAbbr = HtmlAbbr::new("TMNT").title("Teenage Mutant Ninja Turtles");
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
/// >{{ content | kirei(2) }}</abbr>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = AbbrRecipe, content = Cow<'static, str>)]
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
        $crate::html::HtmlAbbr::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlAbbr::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlAbbr::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlAbbr::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlAbbr::<()>::new($crate::bake_inline![$($content),+])
    };

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlAbbr::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlAbbr::<$crate::cookbook_type!($($r),+)>::new($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlAbbr::<$crate::cookbook_type!($($r),+)>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlAbbr::<$crate::cookbook_type!($($r),+)>::new($crate::bake_newline!($content))
    };
    (@cookbook $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlAbbr::<$crate::cookbook_type!($($r),+)>::new($crate::bake_inline![$($content),+])
    };
}
