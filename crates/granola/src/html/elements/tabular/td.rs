use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</td>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TdTag, content = Cow<'static, str>, attrs = TdAttrs)]
pub struct HtmlTd<M: TdTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: TdAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<todo>` element specific attributes.
///
/// [MDN Documentation]()
///
/// # Askama template
///
/// ```askama
/// {{- colspan | bake_attr("colspan") -}}
/// {{- headers | bake_attr("headers") -}}
/// {{- rowspan | bake_attr("rowspan") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TdAttrs {
    pub colspan: Option<u32>,
    pub headers: Option<Cow<'static, str>>,
    pub rowspan: Option<u32>,
}

pub trait HasTdAttrs: Sized {
    fn td_attrs_mut(&mut self) -> &mut TdAttrs;

    /// Number of columns that the cell is to span.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/td#colspan)
    fn colspan(mut self, value: u32) -> Self {
        self.td_attrs_mut().colspan = Some(value);
        self
    }

    /// The header cells for this cell.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/td#headers)
    fn headers(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.td_attrs_mut().headers = Some(value.into());
        self
    }

    /// Number of rows that the cell is to span.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/td#rowspan)
    fn rowspan(mut self, value: u32) -> Self {
        self.td_attrs_mut().rowspan = Some(value);
        self
    }
}

impl HasTdAttrs for TdAttrs {
    fn td_attrs_mut(&mut self) -> &mut TdAttrs {
        self
    }
}

impl HasTdAttrs for &mut TdAttrs {
    fn td_attrs_mut(&mut self) -> &mut TdAttrs {
        self
    }
}

impl<M: TdTag> HasTdAttrs for HtmlTd<M> {
    fn td_attrs_mut(&mut self) -> &mut TdAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlTd`.
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
