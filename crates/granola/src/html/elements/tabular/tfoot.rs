use askama::Template;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait TfootTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = TableRows;
}

impl TfootTag for () {}

/// The HTML `<tfoot>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tfoot)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let tfoot: HtmlTfoot = HtmlTfoot::empty().id("table_foot");
///
/// assert_eq!(tfoot.bake(),
/// r#"<tfoot id="table_foot"></tfoot>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let td: HtmlTd = HtmlTd::new("Don't see what you're after? We'll do our best.").colspan(2);
/// let tr: HtmlTr = HtmlTr::new(bake_newline!(td));
///
/// let tfoot: HtmlTfoot = HtmlTfoot::new(tr);
///
/// assert_eq!(tfoot.bake(),
/// r#"<tfoot>
///   <tr>
///     <td colspan="2">Don't see what you're after? We'll do our best.</td>
///   </tr>
/// </tfoot>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <tfoot
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</tfoot>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlTfoot<M: TfootTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: TfootTag> HtmlTfoot<M> {
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

/// Shorthand for `HtmlTfoot<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let tfoot = tfoot!().id("table_foot");
///
/// assert_eq!(tfoot.bake(),
/// r#"<tfoot id="table_foot"></tfoot>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let td = td!("Don't see what you're after? We'll do our best.").colspan(2);
/// let tr = tr!(@newline td);
///
/// let tfoot = tfoot!(tr);
///
/// assert_eq!(tfoot.bake(),
/// r#"<tfoot>
///   <tr>
///     <td colspan="2">Don't see what you're after? We'll do our best.</td>
///   </tr>
/// </tfoot>"#);
/// ```
#[macro_export]
macro_rules! tfoot {
    () => {
        $crate::html::HtmlTfoot::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTfoot::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTfoot::<()>::new([$first $(, $rest)*])
    };
}
