use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<sup>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sup)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let sup: HtmlSup = HtmlSup::empty().id("superscript");
///
/// assert_eq!(sup.bake(),
/// r#"<sup id="superscript"></sup>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let sup: HtmlSup = HtmlSup::new("e");
///
/// let anniv = bake_inline!["100", sup, " anniversaire"];
///
/// assert_eq!(anniv,
/// r#"100<sup>e</sup> anniversaire"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <sup{{ attrs }}>{{ content | kirei(2) }}</sup>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SupTag, content = Cow<'static, str>)]
pub struct HtmlSup<M: SupTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
}

/// Shorthand for `HtmlSup`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sup = sup!().id("superscript");
///
/// assert_eq!(sup.bake(),
/// r#"<sup id="superscript"></sup>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sup = sup!("e");
///
/// let anniv = bake_inline!["100", sup, " anniversaire"];
///
/// assert_eq!(anniv,
/// r#"100<sup>e</sup> anniversaire"#);
/// ```
#[macro_export]
macro_rules! sup {
    () => {
        $crate::html::HtmlSup::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSup::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSup::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSup::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSup::<()>::new($crate::bake_inline![$($content),+])
    };
}
