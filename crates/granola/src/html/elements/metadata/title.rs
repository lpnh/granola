use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<title>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/title)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let title: HtmlTitle = HtmlTitle::empty().id("document_title");
///
/// assert_eq!(title.bake(),
/// r#"<title id="document_title"></title>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let title: HtmlTitle = HtmlTitle::new("On the unabashed art of self-referential examples");
///
/// assert_eq!(title.bake(),
/// r#"<title>On the unabashed art of self-referential examples</title>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <title{{ attrs }}>{{ content | kirei(2) }}</title>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TitleTag, content = Cow<'static, str>)]
pub struct HtmlTitle<M: TitleTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub attrs: Attrs,
}

/// Shorthand for `HtmlTitle`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let title = title!().id("document_title");
///
/// assert_eq!(title.bake(),
/// r#"<title id="document_title"></title>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let title = title!("On the unabashed art of self-referential examples");
///
/// assert_eq!(title.bake(),
/// r#"<title>On the unabashed art of self-referential examples</title>"#);
/// ```
#[macro_export]
macro_rules! title {
    () => {
        $crate::html::HtmlTitle::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTitle::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTitle::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlTitle::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlTitle::<()>::new($crate::bake_inline![$($content),+])
    };
}
