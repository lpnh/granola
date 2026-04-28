use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait AsideTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: feed, none, note, presentation, region, search
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl AsideTag for () {}

/// The HTML `<aside>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/aside)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let aside: HtmlAside = HtmlAside::empty().id("aside");
///
/// assert_eq!(aside.bake(),
/// r#"<aside id="aside"></aside>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let tip: HtmlStrong = HtmlStrong::new("Tip:");
/// let content: HtmlP = HtmlP::new(bake_inline![tip, " trust your senses more than the timer."]);
///
/// let aside: HtmlAside = HtmlAside::new(bake_newline!(content)).role("note");
///
/// assert_eq!(aside.bake(),
/// r#"<aside role="note">
///   <p><strong>Tip:</strong> trust your senses more than the timer.</p>
/// </aside>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <aside
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</aside>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlAside<M: AsideTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: AsideTag> HtmlAside<M> {
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

/// Shorthand for `HtmlAside<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let aside = aside!().id("aside");
///
/// assert_eq!(aside.bake(),
/// r#"<aside id="aside"></aside>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let tip = strong!("Tip:");
/// let content = p!(@inline tip, " trust your senses more than the timer.");
///
/// let aside = aside!(@newline content).role("note");
///
/// assert_eq!(aside.bake(),
/// r#"<aside role="note">
///   <p><strong>Tip:</strong> trust your senses more than the timer.</p>
/// </aside>"#);
/// ```
#[macro_export]
macro_rules! aside {
    () => {
        $crate::html::HtmlAside::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlAside::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlAside::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlAside::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlAside::<()>::new($crate::bake_inline![$($content),+])
    };
}
