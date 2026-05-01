use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait STag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlS<Self>) -> HtmlS<Self> {
        element
    }
}

impl STag for () {}

/// The HTML `<s>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/s)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let s: HtmlS = HtmlS::empty().id("strikethrough");
///
/// assert_eq!(s.bake(),
/// r#"<s id="strikethrough"></s>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let nine: HtmlS = HtmlS::new("nine");
///
/// let solar_system = bake_inline!["Our solar system has ", nine ," eight planets"];
///
/// assert_eq!(solar_system,
/// r#"Our solar system has <s>nine</s> eight planets"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <s
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</s>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlS<M: STag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: STag> HtmlS<M> {
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

/// Shorthand for `HtmlS<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let s = s!().id("strikethrough");
///
/// assert_eq!(s.bake(),
/// r#"<s id="strikethrough"></s>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let nine = s!("nine");
///
/// let solar_system = bake_inline!["Our solar system has ", nine ," eight planets"];
///
/// assert_eq!(solar_system,
/// r#"Our solar system has <s>nine</s> eight planets"#);
/// ```
#[macro_export]
macro_rules! s {
    () => {
        $crate::html::HtmlS::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlS::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlS::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlS::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlS::<()>::new($crate::bake_inline![$($content),+])
    };
}
