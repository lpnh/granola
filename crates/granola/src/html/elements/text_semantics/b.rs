use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<b>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/b)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let b = HtmlB::new().id("bring_attention_to");
///
/// assert_eq!(b.bake(), r#"<b id="bring_attention_to"></b>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let flour = HtmlB::new().content("flour");
/// let water = HtmlB::new().content("water");
/// let salt = HtmlB::new().content("salt");
///
/// let recipe = bake_inline!["Mix ", flour, ", ", water, ", and ", salt, "."];
///
/// assert_eq!(
///     recipe,
///     r#"Mix <b>flour</b>, <b>water</b>, and <b>salt</b>."#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <b
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</b>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BRecipe, content = Cow<'static, str>)]
pub struct HtmlB<R: BRecipe = ()> {
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

/// Shorthand for `HtmlB`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let b = b!().id("bring_attention_to");
///
/// assert_eq!(b.bake(), r#"<b id="bring_attention_to"></b>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let flour = b!("flour");
/// let water = b!("water");
/// let salt = b!("salt");
///
/// let recipe = bake_inline!["Mix ", flour, ", ", water, ", and ", salt, "."];
///
/// assert_eq!(
///     recipe,
///     r#"Mix <b>flour</b>, <b>water</b>, and <b>salt</b>."#
/// );
/// ```
#[macro_export]
macro_rules! b {
    () => {
        $crate::html::HtmlB::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlB::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlB::new().content($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content:expr $(,)?) => {
        $crate::html::HtmlB::new().content($crate::bake_newline!($content))
    };
    (@inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlB::new().content($crate::bake_inline![$($content),+])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlB::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlB::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlB::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_block![$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlB::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_newline!($content))
    };
    (@cookbook $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlB::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_inline![$($content),+])
    };
}
