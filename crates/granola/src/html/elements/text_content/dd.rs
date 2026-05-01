use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait DdTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlDd<Self>) -> HtmlDd<Self> {
        element
    }
}

impl DdTag for () {}

/// The HTML `<dd>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dd)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let dd: HtmlDd = HtmlDd::empty().id("description_details");
///
/// assert_eq!(dd.bake(),
/// r#"<dd id="description_details"></dd>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let dt: HtmlDt = HtmlDt::new("Hiraeth");
/// let dd: HtmlDd = HtmlDd::new("A longing for a home that no longer exists, or perhaps never did.");
///
/// let term = bake_block![dt, dd];
///
/// assert_eq!(term,
/// r#"<dt>Hiraeth</dt>
/// <dd>A longing for a home that no longer exists, or perhaps never did.</dd>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <dd
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</dd>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDd<M: DdTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DdTag> HtmlDd<M> {
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

/// Shorthand for `HtmlDd<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dd = dd!().id("description_details");
///
/// assert_eq!(dd.bake(),
/// r#"<dd id="description_details"></dd>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dt = dt!("Hiraeth");
/// let dd = dd!("A longing for a home that no longer exists, or perhaps never did.");
///
/// let term = bake_block![dt, dd];
///
/// assert_eq!(term,
/// r#"<dt>Hiraeth</dt>
/// <dd>A longing for a home that no longer exists, or perhaps never did.</dd>"#);
/// ```
#[macro_export]
macro_rules! dd {
    () => {
        $crate::html::HtmlDd::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDd::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDd::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDd::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDd::<()>::new($crate::bake_inline![$($content),+])
    };
}
