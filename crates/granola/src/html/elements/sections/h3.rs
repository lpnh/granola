use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// tab, presentation or none
pub trait H3Tag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlH3<Self>) -> HtmlH3<Self> {
        element
    }
}

impl H3Tag for () {}

/// The HTML `<h3>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let h3: HtmlH3 = HtmlH3::empty().id("html_section_heading");
///
/// assert_eq!(h3.bake(),
/// r#"<h3 id="html_section_heading"></h3>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let panic: HtmlCode = HtmlCode::new("panic!");
///
/// let content = bake_inline!["Unrecoverable Errors with ", panic];
///
/// let h3: HtmlH3 = HtmlH3::new(content);
///
/// assert_eq!(h3.bake(),
/// r#"<h3>Unrecoverable Errors with <code>panic!</code></h3>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <h3
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</h3>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlH3<M: H3Tag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: H3Tag> HtmlH3<M> {
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

/// Shorthand for `HtmlH3<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let h3 = h3!().id("html_section_heading");
///
/// assert_eq!(h3.bake(),
/// r#"<h3 id="html_section_heading"></h3>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let panic = code!("panic!");
///
/// let h3 = h3!(@inline "Unrecoverable Errors with ", panic);
///
/// assert_eq!(h3.bake(),
/// r#"<h3>Unrecoverable Errors with <code>panic!</code></h3>"#);
/// ```
#[macro_export]
macro_rules! h3 {
    () => {
        $crate::html::HtmlH3::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlH3::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlH3::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlH3::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlH3::<()>::new($crate::bake_inline![$($content),+])
    };
}
