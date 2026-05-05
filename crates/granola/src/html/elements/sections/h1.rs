use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<h1>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let h1: HtmlH1 = HtmlH1::empty().id("html_section_heading");
///
/// assert_eq!(h1.bake(),
/// r#"<h1 id="html_section_heading"></h1>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let h1: HtmlH1 = HtmlH1::new("The Rust Programming Language");
///
/// assert_eq!(h1.bake(),
/// r#"<h1>The Rust Programming Language</h1>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <h1{{ attrs }}>{{ content | kirei(2) }}</h1>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = H1Tag, content = Cow<'static, str>)]
pub struct HtmlH1<M: H1Tag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// tab, presentation or none
    pub attrs: Attrs,
}

/// Shorthand for `HtmlH1`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let h1 = h1!().id("html_section_heading");
///
/// assert_eq!(h1.bake(),
/// r#"<h1 id="html_section_heading"></h1>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let h1 = h1!("The Rust Programming Language");
///
/// assert_eq!(h1.bake(),
/// r#"<h1>The Rust Programming Language</h1>"#);
/// ```
#[macro_export]
macro_rules! h1 {
    () => {
        $crate::html::HtmlH1::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlH1::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlH1::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlH1::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlH1::<()>::new($crate::bake_inline![$($content),+])
    };
}
