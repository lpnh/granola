use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// group, presentation or none
pub trait FooterTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlFooter<Self>) -> HtmlFooter<Self> {
        element
    }
}

impl FooterTag for () {}

/// The HTML `<footer>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/footer)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let footer: HtmlFooter = HtmlFooter::empty().id("footer");
///
/// assert_eq!(footer.bake(),
/// r#"<footer id="footer"></footer>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let content: HtmlSmall = HtmlSmall::new("&copy; 2026 Oats &amp; Ends Café");
/// let paragraph: HtmlP = HtmlP::new(content);
///
/// let footer: HtmlFooter = HtmlFooter::new(bake_newline!(paragraph));
///
/// assert_eq!(footer.bake(),
/// r#"<footer>
///   <p><small>&copy; 2026 Oats &amp; Ends Café</small></p>
/// </footer>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <footer
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</footer>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlFooter<M: FooterTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: FooterTag> HtmlFooter<M> {
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

/// Shorthand for `HtmlFooter<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let footer = footer!().id("footer");
///
/// assert_eq!(footer.bake(),
/// r#"<footer id="footer"></footer>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
///
/// let content = small!("&copy; 2026 Oats &amp; Ends Café");
/// let paragraph = p!(content);
///
/// let footer = footer!(@newline paragraph);
///
/// assert_eq!(footer.bake(),
/// r#"<footer>
///   <p><small>&copy; 2026 Oats &amp; Ends Café</small></p>
/// </footer>"#);
/// ```
#[macro_export]
macro_rules! footer {
    () => {
        $crate::html::HtmlFooter::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlFooter::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlFooter::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlFooter::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlFooter::<()>::new($crate::bake_inline![$($content),+])
    };
}
