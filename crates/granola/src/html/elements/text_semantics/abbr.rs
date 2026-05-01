use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait AbbrTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlAbbr<Self>) -> HtmlAbbr<Self> {
        element
    }
}

impl AbbrTag for () {}

/// The HTML `<abbr>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/abbr)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let abbr: HtmlAbbr = HtmlAbbr::empty().id("abbreviation");
///
/// assert_eq!(abbr.bake(),
/// r#"<abbr id="abbreviation"></abbr>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let abbr: HtmlAbbr = HtmlAbbr::new("TMNT").title("Teenage Mutant Ninja Turtles");
///
/// assert_eq!(abbr.bake(),
/// r#"<abbr title="Teenage Mutant Ninja Turtles">TMNT</abbr>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <abbr
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</abbr>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlAbbr<M: AbbrTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: AbbrTag> HtmlAbbr<M> {
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

/// Shorthand for `HtmlAbbr<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let abbr = abbr!().id("abbreviation");
///
/// assert_eq!(abbr.bake(),
/// r#"<abbr id="abbreviation"></abbr>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let abbr = abbr!("TMNT").title("Teenage Mutant Ninja Turtles");
///
/// assert_eq!(abbr.bake(),
/// r#"<abbr title="Teenage Mutant Ninja Turtles">TMNT</abbr>"#);
/// ```
#[macro_export]
macro_rules! abbr {
    () => {
        $crate::html::HtmlAbbr::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlAbbr::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlAbbr::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlAbbr::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlAbbr::<()>::new($crate::bake_inline![$($content),+])
    };
}
