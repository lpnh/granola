use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait H1Tag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: tab, presentation or none
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl H1Tag for () {}

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
/// <h1
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</h1>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlH1<M: H1Tag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: H1Tag> HtmlH1<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }
}

/// Shorthand for `HtmlH1<()>`.
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
