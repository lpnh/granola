use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<h2>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let h2: HtmlH2 = HtmlH2::empty().id("html_section_heading");
///
/// assert_eq!(h2.bake(),
/// r#"<h2 id="html_section_heading"></h2>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let h2: HtmlH2 = HtmlH2::new("Error Handling");
///
/// assert_eq!(h2.bake(),
/// r#"<h2>Error Handling</h2>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <h2
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</h2>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = H2Tag, content = Cow<'static, str>)]
pub struct HtmlH2<R: H2Tag = ()> {
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

/// Shorthand for `HtmlH2`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let h2 = h2!().id("html_section_heading");
///
/// assert_eq!(h2.bake(),
/// r#"<h2 id="html_section_heading"></h2>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let h2 = h2!("Error Handling");
///
/// assert_eq!(h2.bake(),
/// r#"<h2>Error Handling</h2>"#);
/// ```
#[macro_export]
macro_rules! h2 {
    () => {
        $crate::html::HtmlH2::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlH2::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlH2::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlH2::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlH2::<()>::new($crate::bake_inline![$($content),+])
    };
}
