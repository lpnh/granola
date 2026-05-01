use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// listitem
pub trait DtTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlDt<Self>) -> HtmlDt<Self> {
        element
    }
}

impl DtTag for () {}

/// The HTML `<dt>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dt)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let dt: HtmlDt = HtmlDt::empty().id("description_term");
///
/// assert_eq!(dt.bake(),
/// r#"<dt id="description_term"></dt>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let dt: HtmlDt = HtmlDt::new("Pålegg");
/// let dd: HtmlDd = HtmlDd::new("Anything and everything you might put on a slice of bread.");
///
/// let term = bake_block![dt, dd];
///
/// assert_eq!(term,
/// r#"<dt>Pålegg</dt>
/// <dd>Anything and everything you might put on a slice of bread.</dd>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <dt
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</dt>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDt<M: DtTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DtTag> HtmlDt<M> {
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

/// Shorthand for `HtmlDt<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dt = dt!().id("description_term");
///
/// assert_eq!(dt.bake(),
/// r#"<dt id="description_term"></dt>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dt = dt!("Pålegg");
/// let dd = dd!("Anything and everything you might put on a slice of bread.");
///
/// let term = bake_block![dt, dd];
///
/// assert_eq!(term,
/// r#"<dt>Pålegg</dt>
/// <dd>Anything and everything you might put on a slice of bread.</dd>"#);
/// ```
#[macro_export]
macro_rules! dt {
    () => {
        $crate::html::HtmlDt::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDt::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDt::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDt::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDt::<()>::new($crate::bake_inline![$($content),+])
    };
}
