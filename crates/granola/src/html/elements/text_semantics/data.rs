use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait DataTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlData<Self>) -> HtmlData<Self> {
        element
    }
}

impl DataTag for () {}

/// The HTML `<data>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/data)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let data: HtmlData = HtmlData::empty().id("data");
///
/// assert_eq!(data.bake(),
/// r#"<data id="data"></data>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let data: HtmlData = HtmlData::new("$13.37").value("1337");
///
/// assert_eq!(data.bake(),
/// r#"<data value="1337">$13.37</data>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <data
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</data>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlData<M: DataTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DataTag> HtmlData<M> {
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

    /// Machine-readable value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data#value)
    pub fn value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("value", value);
        self
    }
}

/// Shorthand for `HtmlData<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let data = data!().id("data");
///
/// assert_eq!(data.bake(),
/// r#"<data id="data"></data>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let data = data!("$13.37").value("1337");
///
/// assert_eq!(data.bake(),
/// r#"<data value="1337">$13.37</data>"#);
/// ```
#[macro_export]
macro_rules! data {
    () => {
        $crate::html::HtmlData::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlData::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlData::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlData::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlData::<()>::new($crate::bake_inline![$($content),+])
    };
}
