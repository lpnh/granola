use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

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
/// let table: HtmlTable = HtmlTable::empty().id("table");
///
/// assert_eq!(table.bake(),
/// r#"<table id="table"></table>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let caption: HtmlCaption = HtmlCaption::new("Our favorites, yours to try.");
///
/// let col_1: HtmlCol = HtmlCol::new().class("item");
/// let col_2: HtmlCol = HtmlCol::new().class("description");
///
/// let colgroup: HtmlColgroup = HtmlColgroup::new([col_1, col_2]);
///
/// let head_1: HtmlTh = HtmlTh::new("Item").scope("col");
/// let head_2: HtmlTh = HtmlTh::new("Description").scope("col");
///
/// let tr_head: HtmlTr = HtmlTr::new(bake_block![head_1, head_2]);
///
/// let thead: HtmlThead = HtmlThead::new(tr_head);
///
/// let th_1: HtmlTh = HtmlTh::new("Black coffee").scope("row");
/// let td_1: HtmlTd = HtmlTd::new("A good, hot, black coffee");
///
/// let black_coffee: HtmlTr = HtmlTr::new(bake_block![th_1, td_1]);
///
/// let th_2: HtmlTh = HtmlTh::new("Hot chocolate").scope("row");
/// let td_2: HtmlTd = HtmlTd::new("Melted dark chocolate with milk");
///
/// let hot_chocolate: HtmlTr = HtmlTr::new(bake_block![th_2, td_2]);
///
/// let tbody: HtmlTbody = HtmlTbody::new([black_coffee, hot_chocolate]);
///
/// let td_foot: HtmlTd = HtmlTd::new("Don't see what you're after? We'll do our best.").colspan(2);
/// let tr_foot: HtmlTr = HtmlTr::new(bake_newline!(td_foot));
///
/// let tfoot: HtmlTfoot = HtmlTfoot::new(tr_foot);
///
/// let content = bake_block![
///     caption,
///     colgroup,
///     thead,
///     tbody,
///     tfoot,
/// ];
///
/// let table: HtmlTable = HtmlTable::new(content);
///
/// assert_eq!(table.bake(),
/// r#"<table>
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
/// </table>"#);
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
/// >{{ content | kirei(2) }}</table>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TableTag, content = Cow<'static, str>)]
pub struct HtmlTable<M: TableTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
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
/// assert_eq!(table.bake(),
/// r#"<table id="table"></table>"#);
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
/// let tr_foot = tr!(@newline td_foot);
///
/// let tfoot = tfoot!(tr_foot);
///
/// let table = table![
///     caption,
///     colgroup,
///     thead,
///     tbody,
///     tfoot,
/// ];
///
/// assert_eq!(table.bake(),
/// r#"<table>
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
/// </table>"#);
/// ```
#[macro_export]
macro_rules! table {
    () => {
        $crate::html::HtmlTable::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTable::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTable::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlTable::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlTable::<()>::new($crate::bake_inline![$($content),+])
    };
}
