use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait TimeTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlTime<Self>) -> HtmlTime<Self> {
        element
    }
}

impl TimeTag for () {}

/// The HTML `<time>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/time)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let time: HtmlTime = HtmlTime::empty().id("time_date");
///
/// assert_eq!(time.bake(),
/// r#"<time id="time_date"></time>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let time: HtmlTime = HtmlTime::new("Unix epoch").datetime("1970-01-01T00:00:00Z");
///
/// assert_eq!(time.bake(),
/// r#"<time datetime="1970-01-01T00:00:00Z">Unix epoch</time>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <time
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</time>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlTime<M: TimeTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: TimeTag> HtmlTime<M> {
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

    /// Machine-readable datetime representation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time#datetime)
    pub fn datetime(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("datetime", value);
        self
    }
}

/// Shorthand for `HtmlTime<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let time = time!().id("time_date");
///
/// assert_eq!(time.bake(),
/// r#"<time id="time_date"></time>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let time = time!("Unix epoch").datetime("1970-01-01T00:00:00Z");
///
/// assert_eq!(time.bake(),
/// r#"<time datetime="1970-01-01T00:00:00Z">Unix epoch</time>"#);
/// ```
#[macro_export]
macro_rules! time {
    () => {
        $crate::html::HtmlTime::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTime::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTime::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlTime::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlTime::<()>::new($crate::bake_inline![$($content),+])
    };
}
