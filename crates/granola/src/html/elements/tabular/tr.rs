use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait TrTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl TrTag for () {}

/// The HTML `<tr>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tr)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let tr: HtmlTr = HtmlTr::empty().id("table_row");
///
/// assert_eq!(tr.bake(),
/// r#"<tr id="table_row"></tr>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let th: HtmlTh = HtmlTh::new("Hot chocolate").scope("row");
/// let td: HtmlTd = HtmlTd::new("Melted dark chocolate with milk");
///
/// let tr: HtmlTr = HtmlTr::new(bake_block![th, td]);
///
/// assert_eq!(tr.bake(),
/// r#"<tr>
///   <th scope="row">Hot chocolate</th>
///   <td>Melted dark chocolate with milk</td>
/// </tr>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <tr
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</tr>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlTr<M: TrTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: TrTag> HtmlTr<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }
}

/// A collection of HTML `<tr>` items.
///
/// The content of [`HtmlTbody`], [`HtmlTfoot`], or [`HtmlThead`].
///
/// ```askama
/// {%- for tr in items %}
/// {{ tr -}}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, PartialEq, Template, Granola)]
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

/// Shorthand for `HtmlTr<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let tr = tr!().id("table_row");
///
/// assert_eq!(tr.bake(),
/// r#"<tr id="table_row"></tr>"#);
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
/// assert_eq!(tr.bake(),
/// r#"<tr>
///   <th scope="row">Hot chocolate</th>
///   <td>Melted dark chocolate with milk</td>
/// </tr>"#);
/// ```
#[macro_export]
macro_rules! tr {
    () => {
        $crate::html::HtmlTr::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTr::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTr::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlTr::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlTr::<()>::new($crate::bake_inline![$($content),+])
    };
}
