use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<tbody>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tbody)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let tbody = HtmlTbody::new().id("table_body");
///
/// assert_eq!(tbody.bake(), r#"<tbody id="table_body"></tbody>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
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
/// assert_eq!(
///     tbody.bake_pretty(),
///     r#"<tbody>
///   <tr>
///     <th scope="row">Black coffee</th>
///     <td>A good, hot, black coffee</td>
///   </tr>
///   <tr>
///     <th scope="row">Hot chocolate</th>
///     <td>Melted dark chocolate with milk</td>
///   </tr>
/// </tbody>
/// "#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <tbody
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</tbody>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TbodyRecipe, content = TableRows)]
pub struct HtmlTbody<R: TbodyRecipe = ()> {
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

/// Shorthand for `HtmlTbody`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let tbody = tbody!().id("table_body");
///
/// assert_eq!(tbody.bake(), r#"<tbody id="table_body"></tbody>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
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
/// assert_eq!(
///     tbody.bake_pretty(),
///     r#"<tbody>
///   <tr>
///     <th scope="row">Black coffee</th>
///     <td>A good, hot, black coffee</td>
///   </tr>
///   <tr>
///     <th scope="row">Hot chocolate</th>
///     <td>Melted dark chocolate with milk</td>
///   </tr>
/// </tbody>
/// "#
/// );
/// ```
#[macro_export]
macro_rules! tbody {
    () => {
        $crate::html::HtmlTbody::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlTbody::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTbody::new().content([$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlTbody::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlTbody::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTbody::<$crate::cookbook_type!($($r),+)>::from_cookbook().content([$first $(, $rest)*])
    };
}
