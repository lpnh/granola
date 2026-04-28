use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait EmTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl EmTag for () {}

/// The HTML `<em>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/em)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let em: HtmlEm = HtmlEm::empty().id("emphasis");
///
/// assert_eq!(em.bake(),
/// r#"<em id="emphasis"></em>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let owned: HtmlEm = HtmlEm::new("owned");
/// let borrow_checker = bake_inline!("I never said he ", owned, " it.");
///
/// assert_eq!(borrow_checker,
/// r#"I never said he <em>owned</em> it."#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <em
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</em>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlEm<M: EmTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: EmTag> HtmlEm<M> {
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

/// Shorthand for `HtmlEm<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let em = em!().id("emphasis");
///
/// assert_eq!(em.bake(),
/// r#"<em id="emphasis"></em>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let owned = em!("owned");
/// let borrow_checker = bake_inline!("I never said he ", owned, " it.");
///
/// assert_eq!(borrow_checker,
/// r#"I never said he <em>owned</em> it."#);
/// ```
#[macro_export]
macro_rules! em {
    () => {
        $crate::html::HtmlEm::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlEm::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlEm::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlEm::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlEm::<()>::new($crate::bake_inline![$($content),+])
    };
}
