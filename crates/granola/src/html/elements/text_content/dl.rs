use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait DlTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: group, list, none, presentation
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl DlTag for () {}

/// The HTML `<dl>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dl)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let dl: HtmlDl = HtmlDl::empty().id("description_list");
///
/// assert_eq!(dl.bake(),
/// r#"<dl id="description_list"></dl>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let dt_1: HtmlDt = HtmlDt::new("Hiraeth");
/// let dd_1: HtmlDd = HtmlDd::new("A longing for a home that no longer exists, or perhaps never did.");
///
/// let group_1 = bake_block![dt_1, dd_1];
///
/// let dt_2: HtmlDt = HtmlDt::new("Pålegg");
/// let dd_2: HtmlDd = HtmlDd::new("Anything and everything you might put on a slice of bread.");
///
/// let group_2 = bake_block![dt_2, dd_2];
///
/// let list = bake_block![group_1, "", group_2];
///
/// let dl: HtmlDl = HtmlDl::new(list);
///
/// assert_eq!(dl.bake(),
/// r#"<dl>
///   <dt>Hiraeth</dt>
///   <dd>A longing for a home that no longer exists, or perhaps never did.</dd>
///
///   <dt>Pålegg</dt>
///   <dd>Anything and everything you might put on a slice of bread.</dd>
/// </dl>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <dl
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</dl>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDl<M: DlTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DlTag> HtmlDl<M> {
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

/// Shorthand for `HtmlDl<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dl = dl!().id("description_list");
///
/// assert_eq!(dl.bake(),
/// r#"<dl id="description_list"></dl>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dt_1 = dt!("Hiraeth");
/// let dd_1 = dd!("A longing for a home that no longer exists, or perhaps never did.");
///
/// let group_1 = bake_block![dt_1, dd_1];
///
/// let dt_2 = dt!("Pålegg");
/// let dd_2 = dd!("Anything and everything you might put on a slice of bread.");
///
/// let group_2 = bake_block![dt_2, dd_2];
///
/// let dl = dl!(group_1, "", group_2);
///
/// assert_eq!(dl.bake(),
/// r#"<dl>
///   <dt>Hiraeth</dt>
///   <dd>A longing for a home that no longer exists, or perhaps never did.</dd>
///
///   <dt>Pålegg</dt>
///   <dd>Anything and everything you might put on a slice of bread.</dd>
/// </dl>"#);
/// ```
#[macro_export]
macro_rules! dl {
    () => {
        $crate::html::HtmlDl::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDl::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDl::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDl::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDl::<()>::new($crate::bake_inline![$($content),+])
    };
}
