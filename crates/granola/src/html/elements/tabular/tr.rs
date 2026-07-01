use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<tr>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tr)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let tr = HtmlTr::new().id("table_row");
///
/// assert_eq!(tr.bake(), r#"<tr id="table_row"></tr>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let th = HtmlTh::new().content("Hot chocolate").scope("row");
/// let td = HtmlTd::new().content("Melted dark chocolate with milk");
///
/// let tr = HtmlTr::new().fold_in(th).fold_in(td);
///
/// assert_eq!(
///     tr.bake(),
///     r#"<tr><th scope="row">Hot chocolate</th><td>Melted dark chocolate with milk</td></tr>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <tr
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</tr>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TrRecipe, content = Cow<'static, str>)]
pub struct HtmlTr<R: TrRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl<R: TrRecipe<Content = Cow<'static, str>>> HtmlTr<R> {
    pub fn fold_in(mut self, content: impl Into<Cow<'static, str>>) -> Self {
        FoldIn::fold_in(&mut self.content, content.into());
        self
    }
}

/// A collection of HTML `<tr>` items.
///
/// The content of [`HtmlTbody`], [`HtmlTfoot`], or [`HtmlThead`].
///
/// ```askama
/// {%- for tr in items -%}
///     {{ tr }}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TableRows {
    items: Vec<HtmlTr>,
}

impl<I: IntoIterator<Item = HtmlTr>> From<I> for TableRows {
    fn from(items: I) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

impl From<HtmlTr> for TableRows {
    fn from(item: HtmlTr) -> Self {
        Self { items: vec![item] }
    }
}

/// Shorthand for `HtmlTr`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let tr = tr!().id("table_row");
///
/// assert_eq!(tr.bake(), r#"<tr id="table_row"></tr>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let th = th!("Hot chocolate").scope("row");
/// let td = td!("Melted dark chocolate with milk");
///
/// let tr = tr!(th, td);
///
/// assert_eq!(
///     tr.bake(),
///     r#"<tr><th scope="row">Hot chocolate</th><td>Melted dark chocolate with milk</td></tr>"#
/// );
/// ```
#[macro_export]
macro_rules! tr {
    () => {
        $crate::html::HtmlTr::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlTr::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTr::new().content($crate::bake![$first $(, $rest)*])
    };

}
