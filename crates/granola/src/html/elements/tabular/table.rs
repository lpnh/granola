use askama::{FastWritable, Template};
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<table>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/table)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let table = HtmlTable::new().id("table");
///
/// assert_eq!(table.bake(), r#"<table id="table"></table>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let caption = HtmlCaption::new().content("Our favorites, yours to try.");
///
/// let col_1 = HtmlCol::new().class("item");
/// let col_2 = HtmlCol::new().class("description");
///
/// let colgroup = HtmlColgroup::new().content([col_1, col_2]);
///
/// let head_1 = HtmlTh::new().content("Item").scope("col");
/// let head_2 = HtmlTh::new().content("Description").scope("col");
///
/// let tr_head = HtmlTr::new().fold_in(head_1).fold_in(head_2);
///
/// let thead = HtmlThead::new().content(tr_head);
///
/// let th_1 = HtmlTh::new().content("Black coffee").scope("row");
/// let td_1 = HtmlTd::new().content("A good, hot, black coffee");
///
/// let black_coffee = HtmlTr::new().fold_in(th_1).fold_in(td_1);
///
/// let th_2 = HtmlTh::new().content("Hot chocolate").scope("row");
/// let td_2 = HtmlTd::new().content("Melted dark chocolate with milk");
///
/// let hot_chocolate = HtmlTr::new().fold_in(th_2).fold_in(td_2);
///
/// let tbody = HtmlTbody::new().content([black_coffee, hot_chocolate]);
///
/// let td_foot = HtmlTd::new()
///     .content("Don't see what you're after? We'll do our best.")
///     .colspan(2);
/// let tr_foot = HtmlTr::new().content(td_foot);
///
/// let tfoot = HtmlTfoot::new().content(tr_foot);
///
/// let content = bake![caption, colgroup, thead, tbody, tfoot,];
///
/// let table = HtmlTable::new().content(content);
///
/// assert_eq!(
///     table.bake_pretty(),
///     r#"<table>
///   <caption>Our favorites, yours to try.</caption>
///   <colgroup>
///     <col class="item" />
///     <col class="description" />
///   </colgroup>
///   <thead>
///     <tr>
///       <th scope="col">Item</th>
///       <th scope="col">Description</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <th scope="row">Black coffee</th>
///       <td>A good, hot, black coffee</td>
///     </tr>
///     <tr>
///       <th scope="row">Hot chocolate</th>
///       <td>Melted dark chocolate with milk</td>
///     </tr>
///   </tbody>
///   <tfoot>
///     <tr>
///       <td colspan="2">Don't see what you're after? We'll do our best.</td>
///     </tr>
///   </tfoot>
/// </table>
/// "#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <table
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</table>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TableRecipe, content = Bake)]
pub struct HtmlTable<R: TableRecipe = ()> {
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

impl<R: TableRecipe<Content = Bake>> HtmlTable<R> {
    pub fn fold_in(mut self, content: impl FastWritable) -> Self {
        self.content.fold_in(content);
        self
    }
}

/// Shorthand for `HtmlTable`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let table = table!().id("table");
///
/// assert_eq!(table.bake(), r#"<table id="table"></table>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let caption = caption!("Our favorites, yours to try.");
///
/// let col_1 = col!().class("item");
/// let col_2 = col!().class("description");
///
/// let colgroup = colgroup!(col_1, col_2);
///
/// let head_1 = th!("Item").scope("col");
/// let head_2 = th!("Description").scope("col");
///
/// let tr_head = tr!(head_1, head_2);
///
/// let thead = thead!(tr_head);
///
/// let th_1 = th!("Black coffee").scope("row");
/// let td_1 = td!("A good, hot, black coffee");
///
/// let black_coffee = tr!(th_1, td_1);
///
/// let th_2 = th!("Hot chocolate").scope("row");
/// let td_2 = td!("Melted dark chocolate with milk");
///
/// let hot_chocolate = tr!(th_2, td_2);
///
/// let tbody = tbody!(black_coffee, hot_chocolate);
///
/// let td_foot = td!("Don't see what you're after? We'll do our best.").colspan(2);
/// let tr_foot = tr!(td_foot);
///
/// let tfoot = tfoot!(tr_foot);
///
/// let table = table![caption, colgroup, thead, tbody, tfoot,];
///
/// assert_eq!(
///     table.bake_pretty(),
///     r#"<table>
///   <caption>Our favorites, yours to try.</caption>
///   <colgroup>
///     <col class="item" />
///     <col class="description" />
///   </colgroup>
///   <thead>
///     <tr>
///       <th scope="col">Item</th>
///       <th scope="col">Description</th>
///     </tr>
///   </thead>
///   <tbody>
///     <tr>
///       <th scope="row">Black coffee</th>
///       <td>A good, hot, black coffee</td>
///     </tr>
///     <tr>
///       <th scope="row">Hot chocolate</th>
///       <td>Melted dark chocolate with milk</td>
///     </tr>
///   </tbody>
///   <tfoot>
///     <tr>
///       <td colspan="2">Don't see what you're after? We'll do our best.</td>
///     </tr>
///   </tfoot>
/// </table>
/// "#
/// );
/// ```
#[macro_export]
macro_rules! table {
    () => {
        $crate::html::HtmlTable::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlTable::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTable::new().content($crate::bake![$first $(, $rest)*])
    };

}
