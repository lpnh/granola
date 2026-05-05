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
/// let b: HtmlB = HtmlB::empty().id("bring_attention_to");
///
/// assert_eq!(b.bake(),
/// r#"<b id="bring_attention_to"></b>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let flour: HtmlB = HtmlB::new("flour");
/// let water: HtmlB = HtmlB::new("water");
/// let salt: HtmlB = HtmlB::new("salt");
///
/// let recipe = bake_inline!["Mix ", flour, ", ", water, ", and ", salt, "."];
///
/// assert_eq!(recipe,
/// r#"Mix <b>flour</b>, <b>water</b>, and <b>salt</b>."#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <b{{ attrs }}>{{ content | kirei(2) }}</b>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BTag, content = Cow<'static, str>)]
pub struct HtmlB<M: BTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
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
/// assert_eq!(b.bake(),
/// r#"<b id="bring_attention_to"></b>"#);
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
/// assert_eq!(recipe,
/// r#"Mix <b>flour</b>, <b>water</b>, and <b>salt</b>."#);
/// ```
#[macro_export]
macro_rules! b {
    () => {
        $crate::html::HtmlB::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlB::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlB::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlB::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlB::<()>::new($crate::bake_inline![$($content),+])
    };
}
