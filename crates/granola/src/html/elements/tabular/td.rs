use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

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
/// let td = HtmlTd::new().id("table_data_cell");
///
/// assert_eq!(td.bake(), r#"<td id="table_data_cell"></td>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let td = HtmlTd::new().content("Melted dark chocolate with milk");
///
/// assert_eq!(td.bake(), r#"<td>Melted dark chocolate with milk</td>"#);
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
/// >{{ content | kirei }}</td>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TdRecipe, content = Bake)]
pub struct HtmlTd<R: TdRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: TdAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<td>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/td#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- headers | bake_attr("headers") -}}
/// {{- colspan | bake_attr("colspan") -}}
/// {{- rowspan | bake_attr("rowspan") -}}
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TdAttrs {
    pub headers: Option<Bake>,
    pub colspan: Option<u32>,
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
    fn headers(mut self, value: impl Into<Bake>) -> Self {
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

impl<R: TdRecipe> HasTdAttrs for HtmlTd<R> {
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
/// assert_eq!(td.bake(), r#"<td id="table_data_cell"></td>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let td = td!("Melted dark chocolate with milk");
///
/// assert_eq!(td.bake(), r#"<td>Melted dark chocolate with milk</td>"#);
/// ```
#[macro_export]
macro_rules! td {
    () => {
        $crate::html::HtmlTd::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlTd::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTd::new().content($crate::bake![$first $(, $rest)*])
    };

}
