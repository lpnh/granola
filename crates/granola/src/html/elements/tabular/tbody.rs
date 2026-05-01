use askama::{FastWritable, Template};
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait TbodyTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = TableRows;

    fn recipe(element: HtmlTbody<Self>) -> HtmlTbody<Self> {
        element
    }
}

impl TbodyTag for () {}

/// The HTML `<tbody>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tbody)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let tbody: HtmlTbody = HtmlTbody::empty().id("table_body");
///
/// assert_eq!(tbody.bake(),
/// r#"<tbody id="table_body"></tbody>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
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
/// assert_eq!(tbody.bake(),
/// r#"<tbody>
///   <tr>
///     <th scope="row">Black coffee</th>
///     <td>A good, hot, black coffee</td>
///   </tr>
///   <tr>
///     <th scope="row">Hot chocolate</th>
///     <td>Melted dark chocolate with milk</td>
///   </tr>
/// </tbody>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <tbody
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</tbody>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlTbody<M: TbodyTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: TbodyTag> HtmlTbody<M> {
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

/// Shorthand for `HtmlTbody<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let tbody = tbody!().id("table_body");
///
/// assert_eq!(tbody.bake(),
/// r#"<tbody id="table_body"></tbody>"#);
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
/// assert_eq!(tbody.bake(),
/// r#"<tbody>
///   <tr>
///     <th scope="row">Black coffee</th>
///     <td>A good, hot, black coffee</td>
///   </tr>
///   <tr>
///     <th scope="row">Hot chocolate</th>
///     <td>Melted dark chocolate with milk</td>
///   </tr>
/// </tbody>"#);
/// ```
#[macro_export]
macro_rules! tbody {
    () => {
        $crate::html::HtmlTbody::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTbody::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTbody::<()>::new([$first $(, $rest)*])
    };
}
