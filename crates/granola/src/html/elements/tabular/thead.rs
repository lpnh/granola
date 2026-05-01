use askama::{FastWritable, Template};
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait TheadTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = TableRows;

    fn recipe(element: HtmlThead<Self>) -> HtmlThead<Self> {
        element
    }
}

impl TheadTag for () {}

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
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</thead>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlThead<M: TheadTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: TheadTag> HtmlThead<M> {
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
}

/// Shorthand for `HtmlThead<()>`.
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
