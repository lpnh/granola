use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait TdTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlTd<Self>) -> HtmlTd<Self> {
        element
    }
}

impl TdTag for () {}

/// The HTML `<td>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/td)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let td: HtmlTd = HtmlTd::empty().id("table_data_cell");
///
/// assert_eq!(td.bake(),
/// r#"<td id="table_data_cell"></td>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let td: HtmlTd = HtmlTd::new("Melted dark chocolate with milk");
///
/// assert_eq!(td.bake(),
/// r#"<td>Melted dark chocolate with milk</td>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <td
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</td>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlTd<M: TdTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: TdTag> HtmlTd<M> {
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

    /// Number of columns that the cell is to span.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/td#colspan)
    pub fn colspan(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("colspan", value.to_string());
        self
    }

    /// The header cells for this cell.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/td#headers)
    pub fn headers(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("headers", value);
        self
    }

    /// Number of rows that the cell is to span.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/td#rowspan)
    pub fn rowspan(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("rowspan", value.to_string());
        self
    }
}

/// Shorthand for `HtmlTd<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let td = td!().id("table_data_cell");
///
/// assert_eq!(td.bake(),
/// r#"<td id="table_data_cell"></td>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let td = td!("Melted dark chocolate with milk");
///
/// assert_eq!(td.bake(),
/// r#"<td>Melted dark chocolate with milk</td>"#);
/// ```
#[macro_export]
macro_rules! td {
    () => {
        $crate::html::HtmlTd::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTd::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTd::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlTd::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlTd::<()>::new($crate::bake_inline![$($content),+])
    };
}
