use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// Permitted ARIA roles: any
pub trait ThTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlTh<Self>) -> HtmlTh<Self> {
        element
    }
}

impl ThTag for () {}

/// The HTML `<th>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let th: HtmlTh = HtmlTh::empty().id("table_header");
///
/// assert_eq!(th.bake(),
/// r#"<th id="table_header"></th>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let th: HtmlTh = HtmlTh::new("Hot chocolate").scope("row");
///
/// assert_eq!(th.bake(),
/// r#"<th scope="row">Hot chocolate</th>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <th
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</th>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlTh<M: ThTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: ThTag> HtmlTh<M> {
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

    /// Alternative label to use for the header cell when referencing the cell in other contexts.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th#abbr)
    pub fn abbr(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("abbr", value);
        self
    }

    /// Number of columns that the cell is to span.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th#colspan)
    pub fn colspan(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("colspan", value.to_string());
        self
    }

    /// The header cells for this cell.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th#headers)
    pub fn headers(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("headers", value);
        self
    }

    /// Number of rows that the cell is to span.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th#rowspan)
    pub fn rowspan(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("rowspan", value.to_string());
        self
    }

    /// Specifies which cells the header cell applies to.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th#scope)
    pub fn scope(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("scope", value);
        self
    }
}

/// Shorthand for `HtmlTh<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let th = th!().id("table_header");
///
/// assert_eq!(th.bake(),
/// r#"<th id="table_header"></th>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let th = th!("Hot chocolate").scope("row");
///
/// assert_eq!(th.bake(),
/// r#"<th scope="row">Hot chocolate</th>"#);
/// ```
#[macro_export]
macro_rules! th {
    () => {
        $crate::html::HtmlTh::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTh::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTh::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlTh::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlTh::<()>::new($crate::bake_inline![$($content),+])
    };
}
