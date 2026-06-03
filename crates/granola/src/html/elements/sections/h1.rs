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
/// let h1 = HtmlH1::new().id("html_section_heading");
///
/// assert_eq!(h1.bake(), r#"<h1 id="html_section_heading"></h1>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let h1 = HtmlH1::new().content("The Rust Programming Language");
///
/// assert_eq!(h1.bake(), r#"<h1>The Rust Programming Language</h1>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <h1
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</h1>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = H1Recipe, content = Cow<'static, str>)]
pub struct HtmlH1<R: H1Recipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// tab, presentation or none
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
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
/// assert_eq!(h1.bake(), r#"<h1 id="html_section_heading"></h1>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let h1 = h1!("The Rust Programming Language");
///
/// assert_eq!(h1.bake(), r#"<h1>The Rust Programming Language</h1>"#);
/// ```
#[macro_export]
macro_rules! h1 {
    () => {
        $crate::html::HtmlH1::new()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlH1::new().content($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlH1::new().content($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlH1::new().content($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlH1::new().content($crate::bake_inline![$($content),+])
    };
}
