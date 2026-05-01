use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// alert, alertdialog, application, banner, complementary, contentinfo, dialog,
/// document, feed, log, main, marquee, navigation, none, note, presentation,
/// search, status, tabpanel
pub trait SectionTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlSection<Self>) -> HtmlSection<Self> {
        element
    }
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
