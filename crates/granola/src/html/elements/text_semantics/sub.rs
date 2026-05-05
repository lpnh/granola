use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<sub>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sub)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let sub: HtmlSub = HtmlSub::empty().id("subscript");
///
/// assert_eq!(sub.bake(),
/// r#"<sub id="subscript"></sub>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let sub: HtmlSub = HtmlSub::new("2");
///
/// let water = bake_inline!["H", sub, "O"];
///
/// assert_eq!(water,
/// r#"H<sub>2</sub>O"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <sub{{ attrs }}>{{ content | kirei(2) }}</sub>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SubTag, content = Cow<'static, str>)]
pub struct HtmlSub<M: SubTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
}

/// Shorthand for `HtmlSub`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sub = sub!().id("subscript");
///
/// assert_eq!(sub.bake(),
/// r#"<sub id="subscript"></sub>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sub = sub!("2");
///
/// let water = bake_inline!["H", sub, "O"];
///
/// assert_eq!(water,
/// r#"H<sub>2</sub>O"#);
/// ```
#[macro_export]
macro_rules! sub {
    () => {
        $crate::html::HtmlSub::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSub::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSub::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSub::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSub::<()>::new($crate::bake_inline![$($content),+])
    };
}
