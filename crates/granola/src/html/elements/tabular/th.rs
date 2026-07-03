use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<th>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let th = HtmlTh::new().id("table_header");
///
/// assert_eq!(th.bake(), r#"<th id="table_header"></th>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let th = HtmlTh::new().content("Hot chocolate").scope("row");
///
/// assert_eq!(th.bake(), r#"<th scope="row">Hot chocolate</th>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <th
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</th>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ThRecipe, content = Bake)]
pub struct HtmlTh<R: ThRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: ThAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<th>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- abbr | bake_attr("abbr") -}}
/// {{- colspan | bake_attr("colspan") -}}
/// {{- headers | bake_attr("headers") -}}
/// {{- rowspan | bake_attr("rowspan") -}}
/// {{- scope | bake_attr("scope") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct ThAttrs {
    abbr: Option<Bake>,
    colspan: Option<u32>,
    headers: Option<Bake>,
    rowspan: Option<u32>,
    scope: Option<Bake>,
}

pub trait HasThAttrs: Sized {
    fn th_attrs_mut(&mut self) -> &mut ThAttrs;

    /// Alternative label to use for the header cell when referencing the cell
    /// in other contexts.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th#abbr)
    fn abbr(mut self, value: impl Into<Bake>) -> Self {
        self.th_attrs_mut().abbr = Some(value.into());
        self
    }

    /// Number of columns that the cell is to span.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th#colspan)
    fn colspan(mut self, value: u32) -> Self {
        self.th_attrs_mut().colspan = Some(value);
        self
    }

    /// The header cells for this cell.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th#headers)
    fn headers(mut self, value: impl Into<Bake>) -> Self {
        self.th_attrs_mut().headers = Some(value.into());
        self
    }

    /// Number of rows that the cell is to span.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th#rowspan)
    fn rowspan(mut self, value: u32) -> Self {
        self.th_attrs_mut().rowspan = Some(value);
        self
    }

    /// Specifies which cells the header cell applies to.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th#scope)
    fn scope(mut self, value: impl Into<Bake>) -> Self {
        self.th_attrs_mut().scope = Some(value.into());
        self
    }
}

impl HasThAttrs for ThAttrs {
    fn th_attrs_mut(&mut self) -> &mut ThAttrs {
        self
    }
}

impl HasThAttrs for &mut ThAttrs {
    fn th_attrs_mut(&mut self) -> &mut ThAttrs {
        self
    }
}

impl<R: ThRecipe> HasThAttrs for HtmlTh<R> {
    fn th_attrs_mut(&mut self) -> &mut ThAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlTh`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let th = th!().id("table_header");
///
/// assert_eq!(th.bake(), r#"<th id="table_header"></th>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let th = th!("Hot chocolate").scope("row");
///
/// assert_eq!(th.bake(), r#"<th scope="row">Hot chocolate</th>"#);
/// ```
#[macro_export]
macro_rules! th {
    () => {
        $crate::html::HtmlTh::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlTh::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTh::new().content($crate::bake![$first $(, $rest)*])
    };

}
