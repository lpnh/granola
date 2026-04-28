use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait SectionTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: alert, alertdialog, application, banner, complementary, contentinfo,
    ///     dialog, document, feed, log, main, marquee, navigation, none, note, presentation,
    ///     search, status, tabpanel
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl SectionTag for () {}

/// The HTML `<section>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/section)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let section: HtmlSection = HtmlSection::empty().id("generic_section");
///
/// assert_eq!(section.bake(),
/// r#"<section id="generic_section"></section>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let h2: HtmlH2 = HtmlH2::new("Latest news");
/// let ul: HtmlUl = HtmlUl::new(HtmlLi::new("New café on Oak Street"));
///
/// let section: HtmlSection = HtmlSection::new(bake_block![h2, ul]);
///
/// assert_eq!(section.bake(),
/// r#"<section>
///   <h2>Latest news</h2>
///   <ul>
///     <li>New café on Oak Street</li>
///   </ul>
/// </section>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <section
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</section>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlSection<M: SectionTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: SectionTag> HtmlSection<M> {
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

/// Shorthand for `HtmlSection<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let section = section!().id("generic_section");
///
/// assert_eq!(section.bake(),
/// r#"<section id="generic_section"></section>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let h2 = h2!("Latest news");
/// let ul = ul!(li!("New café on Oak Street"));
///
/// let section = section!(h2, ul);
///
/// assert_eq!(section.bake(),
/// r#"<section>
///   <h2>Latest news</h2>
///   <ul>
///     <li>New café on Oak Street</li>
///   </ul>
/// </section>"#);
/// ```
#[macro_export]
macro_rules! section {
    () => {
        $crate::html::HtmlSection::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSection::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSection::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSection::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSection::<()>::new($crate::bake_inline![$($content),+])
    };
}
