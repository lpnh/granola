use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait DfnTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl DfnTag for () {}

/// The HTML `<dfn>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dfn)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let dfn: HtmlDfn = HtmlDfn::empty().id("definition");
///
/// assert_eq!(dfn.bake(),
/// r#"<dfn id="definition"></dfn>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let corro: HtmlDfn = HtmlDfn::new("Corro");
///
/// let about = bake_inline![corro, " the Unsafe Rusturchin"];
/// let paragraph: HtmlP = HtmlP::new(about);
///
/// assert_eq!(paragraph.bake(),
/// r#"<p><dfn>Corro</dfn> the Unsafe Rusturchin</p>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <dfn
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</dfn>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDfn<M: DfnTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DfnTag> HtmlDfn<M> {
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

/// Shorthand for `HtmlDfn<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dfn = dfn!().id("definition");
///
/// assert_eq!(dfn.bake(),
/// r#"<dfn id="definition"></dfn>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let corro = dfn!("Corro");
///
/// let paragraph = p!(@inline corro, " the Unsafe Rusturchin");
///
/// assert_eq!(paragraph.bake(),
/// r#"<p><dfn>Corro</dfn> the Unsafe Rusturchin</p>"#);
/// ```
#[macro_export]
macro_rules! dfn {
    () => {
        $crate::html::HtmlDfn::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDfn::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDfn::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDfn::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDfn::<()>::new($crate::bake_inline![$($content),+])
    };
}
