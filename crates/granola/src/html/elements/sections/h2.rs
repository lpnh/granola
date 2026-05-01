use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// tab, presentation or none
pub trait H2Tag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlH2<Self>) -> HtmlH2<Self> {
        element
    }
}

impl H2Tag for () {}

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
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</h2>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlH2<M: H2Tag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: H2Tag> HtmlH2<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let element = Self {
            content: content.into(),
            ..Default::default()
        };

        M::recipe(element)
    }

    pub fn empty() -> Self {
        let element = Self::default();

        M::recipe(element)
    }
}

/// Shorthand for `HtmlH2<()>`.
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
