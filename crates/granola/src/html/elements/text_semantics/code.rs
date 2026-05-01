use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait CodeTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlCode<Self>) -> HtmlCode<Self> {
        element
    }
}

impl CodeTag for () {}

/// The HTML `<code>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/code)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let code: HtmlCode = HtmlCode::empty().id("inline_code");
///
/// assert_eq!(code.bake(),
/// r#"<code id="inline_code"></code>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let code: HtmlCode = HtmlCode::new("todo!()");
///
/// assert_eq!(code.bake(),
/// r#"<code>todo!()</code>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <code
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</code>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlCode<M: CodeTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: CodeTag> HtmlCode<M> {
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

/// Shorthand for `HtmlCode<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let code = code!().id("inline_code");
///
/// assert_eq!(code.bake(),
/// r#"<code id="inline_code"></code>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let code = code!("todo!()");
///
/// assert_eq!(code.bake(),
/// r#"<code>todo!()</code>"#);
/// ```
#[macro_export]
macro_rules! code {
    () => {
        $crate::html::HtmlCode::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlCode::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlCode::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlCode::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlCode::<()>::new($crate::bake_inline![$($content),+])
    };
}
