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
/// let thead: HtmlThead = HtmlThead::empty().id("table_head");
///
/// assert_eq!(thead.bake(),
/// r#"<thead id="table_head"></thead>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let item: HtmlTh = HtmlTh::new("Item").scope("col");
/// let description: HtmlTh = HtmlTh::new("Description").scope("col");
///
/// let tr: HtmlTr = HtmlTr::new(bake_block![item, description]);
///
/// let thead: HtmlThead = HtmlThead::new(tr);
///
/// assert_eq!(thead.bake(),
/// r#"<thead>
///   <tr>
///     <th scope="col">Item</th>
///     <th scope="col">Description</th>
///   </tr>
/// </thead>"#);
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
#[recipe(name = TheadTag, content = TableRows)]
pub struct HtmlThead<M: TheadTag = ()> {
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

/// Shorthand for `HtmlThead`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let thead = thead!().id("table_head");
///
/// assert_eq!(thead.bake(),
/// r#"<thead id="table_head"></thead>"#);
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
/// assert_eq!(thead.bake(),
/// r#"<thead>
///   <tr>
///     <th scope="col">Item</th>
///     <th scope="col">Description</th>
///   </tr>
/// </thead>"#);
/// ```
#[macro_export]
macro_rules! thead {
    () => {
        $crate::html::HtmlThead::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlThead::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlThead::<()>::new([$first $(, $rest)*])
    };
}
