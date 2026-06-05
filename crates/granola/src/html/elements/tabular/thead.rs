use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<thead>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/thead)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let thead = HtmlThead::new().id("table_head");
///
/// assert_eq!(thead.bake(), r#"<thead id="table_head"></thead>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let item = HtmlTh::new().content("Item").scope("col");
/// let description = HtmlTh::new().content("Description").scope("col");
///
/// let tr = HtmlTr::new().content(bake_block![item, description]);
///
/// let thead = HtmlThead::new().content(tr);
///
/// assert_eq!(
///     thead.bake(),
///     r#"<thead>
///   <tr>
///     <th scope="col">Item</th>
///     <th scope="col">Description</th>
///   </tr>
/// </thead>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <thead
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</thead>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TheadRecipe, content = TableRows)]
pub struct HtmlThead<R: TheadRecipe = ()> {
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

/// Shorthand for `HtmlThead`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let thead = thead!().id("table_head");
///
/// assert_eq!(thead.bake(), r#"<thead id="table_head"></thead>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let item = th!("Item").scope("col");
/// let description = th!("Description").scope("col");
///
/// let tr = tr!(item, description);
///
/// let thead = thead!(tr);
///
/// assert_eq!(
///     thead.bake(),
///     r#"<thead>
///   <tr>
///     <th scope="col">Item</th>
///     <th scope="col">Description</th>
///   </tr>
/// </thead>"#
/// );
/// ```
#[macro_export]
macro_rules! thead {
    () => {
        $crate::html::HtmlThead::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlThead::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlThead::new().content([$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlThead::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlThead::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlThead::<$crate::cookbook_type!($($r),+)>::from_cookbook().content([$first $(, $rest)*])
    };
}
