use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<bdi>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdi)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdi: HtmlBdi = HtmlBdi::empty().id("bidirectional_isolate");
///
/// assert_eq!(bdi.bake(),
/// r#"<bdi id="bidirectional_isolate"></bdi>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let gal: HtmlBdi = HtmlBdi::new("גל גדות");
///
/// let notification = bake_inline![gal, " liked your post"];
///
/// assert_eq!(notification,
/// r#"<bdi>גל גדות</bdi> liked your post"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <bdi{{ attrs }}>{{ content | kirei(2) }}</bdi>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BdiTag, content = Cow<'static, str>)]
pub struct HtmlBdi<M: BdiTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
}

/// Shorthand for `HtmlBdi`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let bdi = bdi!().id("bidirectional_isolate");
///
/// assert_eq!(bdi.bake(),
/// r#"<bdi id="bidirectional_isolate"></bdi>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let gal = bdi!("גל גדות");
///
/// let notification = bake_inline![gal, " liked your post"];
///
/// assert_eq!(notification,
/// r#"<bdi>גל גדות</bdi> liked your post"#);
/// ```
#[macro_export]
macro_rules! bdi {
    () => {
        $crate::html::HtmlBdi::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlBdi::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlBdi::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlBdi::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlBdi::<()>::new($crate::bake_inline![$($content),+])
    };
}
